//! Window creation: main (ntfy web UI), settings, about; tray; shutdown.

use crate::{menu, poll, service, state::AppState, store::DEF_INSTANCE_URL};
use std::sync::atomic::Ordering;
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    webview::{NewWindowResponse, PageLoadEvent},
    AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent,
};

/// Injected into every page of the main window (replaces `renderer.js`):
/// clicking any button in the ntfy web UI resets the unread badge.
const INIT_SCRIPT: &str = r#"
(function () {
  if (window.__ntfyDesktopInit) return;
  window.__ntfyDesktopInit = true;
  function reset() {
    try { window.__TAURI_INTERNALS__.invoke('badge_reset'); } catch (e) {}
  }
  document.addEventListener('click', function (ev) {
    var t = ev.target;
    if (t && t.closest && t.closest('button,[role="button"],[class*="Button"],[class*="Mui"],[class*="btn"]')) reset();
  }, true);
})();
"#;

pub fn open_external(url: &str) {
    if !poll::is_allowed_external(url) {
        tracing::warn!("[navigation] blocked opening external url with unsupported protocol: {url}");
        return;
    }
    if let Err(e) = tauri_plugin_opener::open_url(url, None::<&str>) {
        tracing::error!("failed to open {url}: {e}");
    }
}

pub fn show_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}

pub fn graceful_shutdown(app: &AppHandle) {
    tracing::info!("[shutdown] initiating graceful shutdown");
    let state = app.state::<AppState>();
    state.shutting_down.store(true, Ordering::SeqCst);
    service::stop_polling(app);
    app.exit(0);
}

pub fn create_main(app: &AppHandle) -> tauri::Result<()> {
    let state = app.state::<AppState>();
    // The configured url is navigated to by `load_instance` once validated;
    // start on the official page so the window is never blank.
    let start = url::Url::parse(DEF_INSTANCE_URL).expect("static url");

    let nav_handle = app.clone();
    let load_handle = app.clone();

    let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::External(start))
        .title(format!("Ntfy Desktop : v{} ({})", env!("CARGO_PKG_VERSION"), env!("NTFY_BUILD")))
        .inner_size(1280.0, 720.0)
        .visible(!state.start_hidden())
        .background_color(tauri::utils::config::Color(33, 33, 33, 255))
        .initialization_script(INIT_SCRIPT)
        .on_navigation(move |url| {
            let st = nav_handle.state::<AppState>();
            let inst = st.store.get_str("instanceURL").unwrap_or_else(|| DEF_INSTANCE_URL.into());
            if poll::is_internal_navigation(url, &inst, DEF_INSTANCE_URL) {
                true
            } else {
                windows_open_external_async(url.as_str());
                false
            }
        })
        .on_new_window(move |url, _features| {
            open_external(url.as_str());
            NewWindowResponse::Deny
        })
        .on_page_load(move |webview, payload| {
            if payload.event() == PageLoadEvent::Finished {
                let st = load_handle.state::<AppState>();
                let msg = st.status_msg.lock().unwrap().clone();
                if let Some(msg) = msg {
                    let _ = webview.eval(service::banner_script(&msg));
                }
            }
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone())?;
    }
    let win = builder.build()?;

    // Close button: hide to tray, or quit with `--terminate` / "Quit on Exit".
    let h = app.clone();
    win.on_window_event(move |ev| match ev {
        WindowEvent::CloseRequested { api, .. } => {
            let st = h.state::<AppState>();
            if !st.shutting_down.load(Ordering::SeqCst) {
                api.prevent_close();
                if st.quit_on_close() {
                    graceful_shutdown(&h);
                } else if let Some(w) = h.get_webview_window("main") {
                    let _ = w.hide();
                }
            }
        }
        // Restoring the window clears the unread badge (like Electron's `restore`).
        WindowEvent::Focused(true) if h.state::<AppState>().badge.load(Ordering::SeqCst) != 0 => {
            service::reset_badge(&h);
        }
        _ => {}
    });
    Ok(())
}

fn windows_open_external_async(url: &str) {
    let url = url.to_string();
    std::thread::spawn(move || open_external(&url));
}

pub fn create_tray(app: &AppHandle) -> tauri::Result<()> {
    let tray_menu = menu::build_tray_menu(app)?;
    let mut tb = TrayIconBuilder::with_id("tray")
        .tooltip("Ntfy Desktop")
        .menu(&tray_menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                let app = tray.app_handle();
                if let Some(w) = app.get_webview_window("main") {
                    if w.is_visible().unwrap_or(false) {
                        let _ = w.hide();
                    } else {
                        show_main(app);
                    }
                }
            }
        });
    if let Some(icon) = app.default_window_icon() {
        tb = tb.icon(icon.clone());
    }
    tb.build(app)?;
    Ok(())
}

fn secondary_window(app: &AppHandle, label: &str, page: &str, title: &str, w: f64, h: f64, modal: bool) -> tauri::Result<()> {
    if let Some(existing) = app.get_webview_window(label) {
        let _ = existing.eval(format!("window.location.replace({})", serde_json::to_string(page).unwrap()));
        let _ = existing.show();
        let _ = existing.set_focus();
        return Ok(());
    }
    let mut b = WebviewWindowBuilder::new(app, label, WebviewUrl::App(page.into()))
        .title(title)
        .inner_size(w, h)
        .center()
        .background_color(tauri::utils::config::Color(33, 33, 33, 255));
    if let Some(main) = app.get_webview_window("main") {
        b = b.parent(&main)?;
    }
    if modal {
        b = b.resizable(false).minimizable(false).maximizable(false);
    }
    let win = b.build()?;
    let _ = win.remove_menu();
    Ok(())
}

pub fn open_settings(app: &AppHandle, section: &str) {
    let title = match section {
        "general" => "General Settings",
        "instance" => "Instance Settings",
        "token" => "Set API Token",
        "topics" => "Set Subscribed Topics",
        "notifications" => "Notifications",
        _ => "Settings",
    };
    if let Err(e) = secondary_window(app, "settings", &format!("settings.html?section={section}"), title, 560.0, 560.0, false) {
        tracing::error!("failed to open settings window: {e}");
    }
}

pub fn open_about(app: &AppHandle) {
    if let Err(e) = secondary_window(app, "about", "about.html", "About", 480.0, 520.0, true) {
        tracing::error!("failed to open about window: {e}");
    }
}
