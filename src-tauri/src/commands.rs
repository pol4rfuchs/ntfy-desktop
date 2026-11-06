//! Tauri commands exposed to the web frontends.
//!
//! * `badge_reset` is the only command the remote ntfy page may call.
//! * Everything else is restricted to the local settings / about windows via
//!   `capabilities/*.json`.

use crate::{
    menu, service,
    state::AppState,
    store::{clamp_pollrate, DEF_DATETIME, DEF_INSTANCE_URL, DEF_POLLRATE, DEF_TOPICS},
    windows,
};
use serde_json::{json, Map, Value};
use std::sync::atomic::Ordering;
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub fn badge_reset(app: AppHandle) {
    service::reset_badge(&app);
}

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Value {
    let s = &state.store;
    json!({
        "bDevTools": s.get_int("bDevTools"),
        "bHotkeys": s.get_int("bHotkeys"),
        "bQuitOnClose": s.get_int("bQuitOnClose"),
        "bStartHidden": s.get_int("bStartHidden"),
        "instanceURL": s.get_str("instanceURL").unwrap_or_else(|| DEF_INSTANCE_URL.into()),
        "bLocalhost": s.get_int("bLocalhost"),
        "topics": s.get_sanitized("topics", DEF_TOPICS),
        "bPersistentNoti": s.get_int("bPersistentNoti"),
        "datetime": s.get_str("datetime").filter(|d| !d.is_empty()).unwrap_or_else(|| DEF_DATETIME.into()),
        "pollrate": if s.get_int("pollrate") == 0 { DEF_POLLRATE } else { s.get_int("pollrate") },
        "apiTokenSet": !service::get_api_token(&state).is_empty(),
        "defaults": { "instanceURL": DEF_INSTANCE_URL, "topics": DEF_TOPICS, "datetime": DEF_DATETIME, "pollrate": DEF_POLLRATE },
    })
}

const FLAG_KEYS: [&str; 6] = ["bDevTools", "bHotkeys", "bQuitOnClose", "bStartHidden", "bLocalhost", "bPersistentNoti"];

/// Apply a partial settings update and run the side effects the Electron menu
/// handlers used to trigger (reload page, restart polling, rebuild menu).
#[tauri::command]
pub fn save_settings(app: AppHandle, state: State<'_, AppState>, patch: Map<String, Value>) -> Result<(), String> {
    let s = &state.store;
    let mut rebuild_menu = false;
    let mut reload = false;
    let mut restart_poll = false;

    for (key, val) in &patch {
        match key.as_str() {
            k if FLAG_KEYS.contains(&k) => {
                let new = if crate::store::value_to_int(Some(val)) != 0 { 1 } else { 0 };
                if s.get_int(k) != new {
                    if k == "bDevTools" || k == "bHotkeys" {
                        rebuild_menu = true;
                    }
                    if k == "bLocalhost" {
                        reload = true;
                    }
                }
                s.set(k, Value::from(new));
            }
            "instanceURL" => {
                let raw = val.as_str().unwrap_or("").trim();
                let new = if raw.is_empty() { DEF_INSTANCE_URL.to_string() } else { raw.to_string() };
                if url::Url::parse(&new).is_err() {
                    return Err(format!("Invalid URL: {new}"));
                }
                if s.get_str("instanceURL").as_deref() != Some(new.as_str()) {
                    reload = true;
                }
                s.set("instanceURL", Value::String(new));
            }
            "topics" => {
                let new = val.as_str().unwrap_or("").to_string();
                if s.get_str("topics").as_deref() != Some(new.as_str()) {
                    s.set("topics", Value::String(new));
                    reload = true;
                }
            }
            "datetime" => {
                let new = val.as_str().unwrap_or("").trim();
                s.set("datetime", Value::String(if new.is_empty() { DEF_DATETIME.into() } else { new.into() }));
            }
            "pollrate" => {
                let n = crate::store::value_to_int(Some(val));
                let n = clamp_pollrate(if n == 0 { DEF_POLLRATE } else { n });
                s.set("pollrate", Value::from(n));
                restart_poll = true;
            }
            _ => {}
        }
    }

    if patch.contains_key("instanceURL") || patch.contains_key("bLocalhost") {
        // a manual change resets the "bad url" state; load_instance re-validates
        state.bad_url.store(false, Ordering::SeqCst);
    }
    if rebuild_menu {
        menu::refresh(&app);
    }
    if restart_poll {
        service::start_polling(&app);
    }
    if reload {
        service::load_instance(&app);
    }
    Ok(())
}

#[tauri::command]
pub fn set_api_token(state: State<'_, AppState>, token: String) {
    service::set_api_token(&state, &token);
}

#[tauri::command]
pub fn get_app_info() -> Value {
    json!({
        "name": "ntfy-desktop",
        "title": "Ntfy Desktop",
        "version": env!("CARGO_PKG_VERSION"),
        "build": env!("NTFY_BUILD"),
        "author": "pol4rfuchs",
        "repo": menu::HOMEPAGE,
        "sponsor": menu::SPONSOR,
        "tauri": tauri::VERSION,
    })
}

#[tauri::command]
pub fn open_external(url: String) {
    windows::open_external(&url);
}

#[tauri::command]
pub fn close_window(app: AppHandle, label: String) {
    if let Some(w) = app.get_webview_window(&label) {
        let _ = w.close();
    }
}
