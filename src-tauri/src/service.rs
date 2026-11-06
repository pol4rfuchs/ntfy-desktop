//! Background behaviour: api token storage, badge, notifications, polling and
//! loading the configured ntfy instance into the main window.

use crate::{
    datefmt,
    poll::{self, Warning},
    state::AppState,
    store::{self, DEF_DATETIME, DEF_INSTANCE_URL, DEF_POLLRATE, DEF_TOPICS},
};
use serde_json::Value;
use std::{sync::atomic::Ordering, time::Duration};
use tauri::{AppHandle, Manager};

const KEYRING_SERVICE: &str = "ntfy-desktop";
const KEYRING_USER: &str = "api-token";

// ---------------------------------------------------------------- api token

/// Token from the OS keychain, falling back to the plain `apiToken` setting
/// (used when no keychain is available).
pub fn get_api_token(state: &AppState) -> String {
    if let Some(t) = state.token_cache.lock().unwrap().clone() {
        return t;
    }
    let token = read_api_token(state);
    *state.token_cache.lock().unwrap() = Some(token.clone());
    token
}

fn read_api_token(state: &AppState) -> String {
    if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER) {
        match entry.get_password() {
            Ok(t) if !t.is_empty() => return t,
            Ok(_) | Err(keyring::Error::NoEntry) => {}
            Err(e) => tracing::warn!("keychain read failed, using settings file: {e}"),
        }
    }
    state.store.get_str("apiToken").unwrap_or_default()
}

pub fn set_api_token(state: &AppState, token: &str) {
    let token = token.trim();
    *state.token_cache.lock().unwrap() = None;
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER);
    if token.is_empty() {
        if let Ok(e) = &entry {
            let _ = e.delete_credential();
        }
        state.store.set("apiToken", Value::String(String::new()));
        return;
    }
    match entry.and_then(|e| e.set_password(token)) {
        Ok(()) => state.store.set("apiToken", Value::String(String::new())),
        Err(e) => {
            tracing::warn!("keychain unavailable ({e}); storing api token in the settings file");
            state.store.set("apiToken", Value::String(token.to_string()));
        }
    }
}

// -------------------------------------------------------------------- badge

fn show_badge(app: &AppHandle, count: i64) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.set_badge_count(if count > 0 { Some(count) } else { None });
    }
    if let Some(tray) = app.tray_by_id("tray") {
        let tip = if count > 0 {
            format!("Ntfy Desktop ({count} new)")
        } else {
            "Ntfy Desktop".to_string()
        };
        let _ = tray.set_tooltip(Some(tip));
    }
}

pub fn update_badge(app: &AppHandle, by: i64) {
    let state = app.state::<AppState>();
    let n = state.badge.fetch_add(by, Ordering::SeqCst) + by;
    state.store.set("indicatorMessages", Value::from(n));
    show_badge(app, n);
}

pub fn reset_badge(app: &AppHandle) {
    let state = app.state::<AppState>();
    state.badge.store(0, Ordering::SeqCst);
    state.store.set("indicatorMessages", Value::from(0));
    show_badge(app, 0);
}

// ------------------------------------------------------------ notifications

pub fn send_notification(title: String, body: String, persistent: bool, open_url: String) {
    // notify-rust can block (D-Bus / COM), keep it off the async runtime.
    std::thread::spawn(move || {
        let mut n = notify_rust::Notification::new();
        n.summary(&title).body(&body).appname("Ntfy Desktop");
        if persistent {
            n.timeout(notify_rust::Timeout::Never);
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            n.sound_name("message-new-instant");
            n.action("default", "Open");
        }
        #[cfg(target_os = "windows")]
        {
            // Only an installed app has a registered toast id; when started
            // via `cargo run` keep notify-rust's default id so toasts show.
            if !cfg!(debug_assertions) {
                n.app_id("com.ntfydesktop.id");
            }
        }
        #[cfg(target_os = "macos")]
        {
            let _ = notify_rust::set_application("com.ntfydesktop.id");
            n.sound_name("Pop");
        }
        match n.show() {
            Ok(_handle) => {
                #[cfg(all(unix, not(target_os = "macos")))]
                _handle.wait_for_action(|action| {
                    if action == "default" && poll::is_allowed_external(&open_url) {
                        let _ = tauri_plugin_opener::open_url(&open_url, None::<&str>);
                    }
                });
                #[cfg(not(all(unix, not(target_os = "macos"))))]
                let _ = &open_url;
            }
            Err(e) => tracing::error!("failed to show notification: {e}"),
        }
    });
}

// ------------------------------------------------------------------ polling

pub fn pollrate(state: &AppState) -> i64 {
    let rate = state.store.get_int("pollrate");
    store::clamp_pollrate(if rate == 0 { DEF_POLLRATE } else { rate })
}

/// (Re)start the poll loop with the configured rate.
pub fn start_polling(app: &AppHandle) {
    let state = app.state::<AppState>();
    let rate = pollrate(&state);
    let interval = Duration::from_millis(rate as u64 * 1000 + 600);

    let mut slot = state.poll_task.lock().unwrap();
    if let Some(old) = slot.take() {
        old.abort();
    }
    let handle = app.clone();
    *slot = Some(tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(interval).await;
            get_messages(&handle).await;
        }
    }));
    tracing::info!("[polling] started message polling, interval {}ms", interval.as_millis());
}

pub fn stop_polling(app: &AppHandle) {
    let state = app.state::<AppState>();
    let task = state.poll_task.lock().unwrap().take();
    if let Some(h) = task {
        h.abort();
    }
}

struct PollingGuard<'a>(&'a AppState);
impl Drop for PollingGuard<'_> {
    fn drop(&mut self) {
        self.0.polling.store(false, Ordering::SeqCst);
    }
}

/// One poll: fetch, de-duplicate, notify.
pub async fn get_messages(app: &AppHandle) {
    let state = app.state::<AppState>();
    if state.shutting_down.load(Ordering::SeqCst) {
        return;
    }
    if state.polling.swap(true, Ordering::SeqCst) {
        tracing::debug!("[messages] polling already in progress, skipping");
        return;
    }
    let _guard = PollingGuard(&state);

    let instance = state.store.get_str("instanceURL").unwrap_or_default();
    let topics = state.store.get_sanitized("topics", DEF_TOPICS);
    let rate = pollrate(&state);
    if state.store.get_int("pollrate") != rate {
        state.store.set("pollrate", Value::from(rate));
    }

    if instance.is_empty() {
        tracing::error!("[messages] aborting: instance url missing");
        return;
    }
    if topics.trim().is_empty() {
        tracing::warn!("[messages] no topics configured, skipping message fetch");
        return;
    }

    let now_ms = chrono::Utc::now().timestamp_millis();
    let last = *state.last_poll_start.lock().unwrap();
    let since = poll::compute_since(last, now_ms, rate);
    let uri = poll::build_poll_url(&instance, &topics, since);

    if state.bad_url.load(Ordering::SeqCst) {
        tracing::error!("[messages] instance url could not be resolved; not polling {uri}");
        return;
    }

    let token = get_api_token(&state);
    tracing::debug!("[messages] fetching {uri}");
    let Some(body) = poll::fetch_body(&state.client, &uri, &token).await else {
        return;
    };
    *state.last_poll_start.lock().unwrap() = Some(now_ms);

    let parsed = poll::parse_body(&body);
    for w in &parsed.warnings {
        match w {
            Warning::InvalidJson => tracing::error!(
                "[messages] polling returned invalid json; skipping entry. Change your instance URL to a valid ntfy instance."
            ),
            Warning::Unauthorized { link } => tracing::warn!(
                "[auth] unauthorized: ensure the instance URL and your API token are correct ({link})"
            ),
            Warning::RateLimited { link } => tracing::warn!(
                "[auth] poll limit reached: too many auth failures; increase your limits with a paid plan ({link})"
            ),
            Warning::ServerError { error, code, link } => {
                tracing::warn!("[auth] skipped pending messages: {error} (code {code}) {link}")
            }
        }
    }

    let persistent = state.store.get_int("bPersistentNoti") != 0;
    let fmt = state.store.get_str("datetime").filter(|s| !s.is_empty()).unwrap_or_else(|| DEF_DATETIME.into());
    for m in parsed.messages {
        let is_new = state.history.lock().unwrap().insert(&m.id);
        if !is_new {
            continue;
        }
        let date = datefmt::format_unix(m.time, &fmt);
        send_notification(
            format!("{} - {}", m.topic, date),
            m.message.clone(),
            persistent,
            state.store.get_str("instanceURL").unwrap_or_default(),
        );
        update_badge(app, 1);
        tracing::debug!("[messages] sent notification id={} topic={}", m.id, m.topic);
    }
    tracing::info!("[instance] messages checked: {uri}");
}

// ---------------------------------------------------------------- instance

/// Validate (unless disabled) and load the configured instance into the main
/// window. Falls back to the official instance when it cannot be reached.
pub fn load_instance(app: &AppHandle) {
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        let state = app.state::<AppState>();
        let mut url = state.store.get_str("instanceURL").unwrap_or_default();
        if url.is_empty() {
            url = DEF_INSTANCE_URL.to_string();
            state.store.set("instanceURL", Value::String(url.clone()));
            *state.status_msg.lock().unwrap() =
                Some(format!("Invalid instance URL specified; defaulting to {DEF_INSTANCE_URL}"));
        }

        if state.store.get_int("bLocalhost") != 1 {
            match poll::is_valid_url(&state.client, &url, 3, Duration::from_secs(1)).await {
                Ok(()) => {
                    tracing::info!("[instance] specified instance successfully resolves: {url}");
                    state.bad_url.store(false, Ordering::SeqCst);
                    *state.status_msg.lock().unwrap() = None;
                }
                Err(e) => {
                    tracing::error!("[instance] failed to resolve {url}: {e}; switching to {DEF_INSTANCE_URL}");
                    state.bad_url.store(true, Ordering::SeqCst);
                    *state.status_msg.lock().unwrap() =
                        Some(format!("Failed to resolve {url}; defaulting to {DEF_INSTANCE_URL}"));
                    state.store.set("instanceURL", Value::String(DEF_INSTANCE_URL.into()));
                    url = DEF_INSTANCE_URL.to_string();
                }
            }
        }

        navigate_main(&app, &url);
    });
}

pub fn navigate_main(app: &AppHandle, url: &str) {
    match url::Url::parse(url) {
        Ok(u) => {
            if let Some(win) = app.get_webview_window("main") {
                if let Err(e) = win.navigate(u) {
                    tracing::error!("navigation to {url} failed: {e}");
                }
            }
        }
        Err(e) => tracing::error!("invalid instance url {url}: {e}"),
    }
}

/// Error banner (`did-finish-load` in the Electron version).
pub fn banner_script(msg: &str) -> String {
    format!(
        r#"(function(){{
  if (document.getElementById('ntfy-desktop-banner')) return;
  const div = document.createElement('div');
  div.id = 'ntfy-desktop-banner';
  Object.assign(div.style, {{bottom:'0',position:'sticky',height:'43px',width:'100%',zIndex:'3000',overflow:'hidden',
    color:'#FFFFFF',backgroundColor:'rgb(137 41 41)',textAlign:'center',fontSize:'0.93rem',fontWeight:'100',
    display:'grid',alignItems:'center'}});
  const span = document.createElement('span');
  span.setAttribute('class','ntfy-notify error');
  span.textContent = {};
  div.appendChild(span);
  document.body.appendChild(div);
}})();"#,
        serde_json::to_string(msg).unwrap_or_else(|_| "\"\"".into())
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_escapes_message() {
        let s = banner_script("a\"</script>'b");
        assert!(s.contains(r#"\"</script>"#));
        assert!(!s.contains("textContent = a"));
    }
}
