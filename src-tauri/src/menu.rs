//! Application menu and tray menu (port of `classes/Menu.js`).

use crate::{service, state::AppState, windows};
use tauri::{
    menu::{Menu, MenuBuilder, MenuEvent, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder},
    AppHandle, Manager, Wry,
};

pub const HOMEPAGE: &str = "https://github.com/pol4rfuchs/ntfy-desktop";
pub const SPONSOR: &str = "https://buymeacoffee.com/aetherinox";

pub fn build_main_menu(app: &AppHandle) -> tauri::Result<Menu<Wry>> {
    let state = app.state::<AppState>();
    let hotkeys = state.hotkeys_enabled();
    let acc = |a: &'static str| if hotkeys { Some(a) } else { None };

    let item = |id: &str, label: &str, accel: Option<&str>| -> tauri::Result<_> {
        let mut b = MenuItemBuilder::with_id(id, label);
        if let Some(a) = accel {
            b = b.accelerator(a);
        }
        b.build(app)
    };

    let settings = SubmenuBuilder::new(app, "&Settings")
        .item(&item("settings:general", "General", acc("CmdOrCtrl+G"))?)
        .item(&item("settings:instance", "Instance", acc("CmdOrCtrl+I"))?)
        .item(&item("settings:token", "API Token", acc("CmdOrCtrl+T"))?)
        .item(&item("settings:topics", "Topics", acc("CmdOrCtrl+Shift+T"))?)
        .item(&item("settings:notifications", "Notifications", acc("CmdOrCtrl+N"))?)
        .build()?;

    let app_menu = SubmenuBuilder::new(app, "&App")
        .item(&settings)
        .separator()
        .item(&item("quit", "Quit", acc("CmdOrCtrl+Q"))?)
        .build()?;

    let edit = SubmenuBuilder::new(app, "&Edit")
        .item(&PredefinedMenuItem::undo(app, None)?)
        .item(&PredefinedMenuItem::redo(app, None)?)
        .separator()
        .item(&PredefinedMenuItem::cut(app, None)?)
        .item(&PredefinedMenuItem::copy(app, None)?)
        .item(&PredefinedMenuItem::paste(app, None)?)
        .item(&PredefinedMenuItem::select_all(app, None)?)
        .build()?;

    let mut view = SubmenuBuilder::new(app, "&View")
        .item(&item("view:back", "Back", Some("CmdOrCtrl+B"))?)
        .item(&item("view:forward", "Forward", Some("CmdOrCtrl+F"))?)
        .separator()
        .item(&item("view:reload", "Reload", Some("CmdOrCtrl+R"))?)
        .item(&item(
            "view:fullscreen",
            "Toggle Full Screen",
            Some(if cfg!(target_os = "macos") { "Ctrl+Cmd+F" } else { "F11" }),
        )?);
    if hotkeys {
        view = view
            .separator()
            .item(&item("view:zoom_in", "Zoom In", Some("CmdOrCtrl+="))?)
            .item(&item("view:zoom_out", "Zoom Out", Some("CmdOrCtrl+-"))?)
            .item(&item("view:zoom_reset", "Reset Zoom", Some("CmdOrCtrl+0"))?)
            .item(&item("view:minimize", "Minimize to Tray", Some("CmdOrCtrl+M"))?);
    }
    if state.devtools_enabled() {
        view = view.separator().item(&item(
            "view:devtools",
            "Toggle Dev Tools",
            Some(if cfg!(target_os = "macos") { "Alt+Cmd+I" } else { "CmdOrCtrl+Shift+I" }),
        )?);
    }
    let view = view.build()?;

    let help = SubmenuBuilder::new(app, "&Help")
        .item(&item("help:test", "Send Test Notification", None)?)
        .separator()
        .item(&item("help:updates", "Check for Updates", None)?)
        .item(&item("help:sponsor", "Sponsor", None)?)
        .item(&item("help:about", "About", None)?)
        .build()?;

    MenuBuilder::new(app).items(&[&app_menu, &edit, &view, &help]).build()
}

pub fn build_tray_menu(app: &AppHandle) -> tauri::Result<Menu<Wry>> {
    MenuBuilder::new(app)
        .item(&MenuItemBuilder::with_id("tray:show", "Show App").build(app)?)
        .item(&MenuItemBuilder::with_id("quit", "Quit").build(app)?)
        .build()
}

/// Rebuild and apply the main menu (after hotkey / dev tools changes).
pub fn refresh(app: &AppHandle) {
    match build_main_menu(app) {
        Ok(menu) => {
            if let Err(e) = app.set_menu(menu) {
                tracing::error!("failed to set menu: {e}");
            }
            for label in ["settings", "about"] {
                if let Some(w) = app.get_webview_window(label) {
                    let _ = w.remove_menu();
                }
            }
        }
        Err(e) => tracing::error!("failed to build menu: {e}"),
    }
}

fn eval_main(app: &AppHandle, js: &str) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.eval(js);
    }
}

pub fn on_menu_event(app: &AppHandle, event: MenuEvent) {
    let id = event.id().as_ref().to_string();
    let state = app.state::<AppState>();
    match id.as_str() {
        "quit" => windows::graceful_shutdown(app),
        "tray:show" => {
            service::reset_badge(app);
            windows::show_main(app);
        }
        "view:back" => eval_main(app, "history.back()"),
        "view:forward" => eval_main(app, "history.forward()"),
        "view:reload" => {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.reload();
            }
        }
        "view:fullscreen" => {
            if let Some(w) = app.get_webview_window("main") {
                let fs = w.is_fullscreen().unwrap_or(false);
                let _ = w.set_fullscreen(!fs);
            }
        }
        "view:zoom_in" | "view:zoom_out" | "view:zoom_reset" => {
            let mut z = state.zoom.lock().unwrap();
            *z = match id.as_str() {
                "view:zoom_in" => (*z + 0.1).min(5.0),
                "view:zoom_out" => (*z - 0.1).max(0.3),
                _ => 1.0,
            };
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.set_zoom(*z);
            }
        }
        "view:minimize" => {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.hide();
            }
        }
        "view:devtools" => {
            if let Some(w) = app.get_webview_window("main") {
                if w.is_devtools_open() {
                    w.close_devtools();
                } else {
                    w.open_devtools();
                }
            }
        }
        "help:test" => {
            service::send_notification(
                app,
                "Ntfy Desktop - Test Notification".into(),
                "This is a test notification which determines if desktop notifications are working on your system. If you can see this, then everything is good.".into(),
                true,
            );
            service::update_badge(app, 1);
        }
        "help:updates" => windows::open_external(HOMEPAGE),
        "help:sponsor" => windows::open_external(SPONSOR),
        "help:about" => windows::open_about(app),
        s if s.starts_with("settings:") => windows::open_settings(app, &s["settings:".len()..]),
        _ => {}
    }
}
