//! Ntfy Desktop — Tauri / Rust port of the Electron client.

mod commands;
mod datefmt;
mod menu;
mod poll;
mod service;
mod state;
mod store;
mod windows;

use state::{AppState, Cli};
use std::{fs, sync::atomic::Ordering};
use tauri::{Manager, RunEvent};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn init_logging(cli: &Cli) {
    let default = if cli.env == "development" { "debug" } else { "info" };
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default));

    let dir = store::user_data_dir().join("logs");
    let _ = fs::create_dir_all(&dir);
    let file = fs::OpenOptions::new().create(true).append(true).open(dir.join("main.log")).ok();

    let stderr = fmt::layer().with_writer(std::io::stderr);
    let file_layer = file.map(|f| fmt::layer().with_ansi(false).with_writer(std::sync::Mutex::new(f)));
    let _ = tracing_subscriber::registry().with(filter).with(stderr).with(file_layer).try_init();
}

pub fn run() {
    let cli = Cli::parse(std::env::args().skip(1));
    if cli.help {
        println!("Ntfy Desktop v{}\n\n{}", env!("CARGO_PKG_VERSION"), state::HELP);
        return;
    }
    init_logging(&cli);
    tracing::info!("[initiate] starting ntfy-desktop {} env={}", env!("CARGO_PKG_VERSION"), cli.env);

    let store = store::Store::open(store::user_data_dir().join("prefs.json"));
    let app_state = AppState::new(cli, store);

    let mut builder = tauri::Builder::default();
    // a second instance would poll the same topics and show every notification twice
    builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
        windows::show_main(app);
    }));

    let app = builder
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::badge_reset,
            commands::get_settings,
            commands::save_settings,
            commands::set_api_token,
            commands::get_app_info,
            commands::open_external,
            commands::close_window,
        ])
        .on_menu_event(menu::on_menu_event)
        .setup(|app| {
            let handle = app.handle().clone();
            let state = handle.state::<AppState>();

            windows::create_main(&handle)?;
            menu::refresh(&handle);
            windows::create_tray(&handle)?;
            service::load_instance(&handle);
            service::start_polling(&handle);

            if state.badge.load(Ordering::SeqCst) > 0 {
                // keep the persisted unread count visible after a restart
                service::update_badge(&handle, 0);
            }

            if state.store.get_sanitized("topics", store::DEF_TOPICS).is_empty() {
                let h = handle.clone();
                tauri::async_runtime::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
                    h.dialog()
                        .message(format!(
                            "You must define topics that you wish to receive notifications for.\nIn the top menu, select \"App\" -> \"Settings\" -> \"Topics\"\n\nTopics should be listed as\n      {},topic3,topic4",
                            store::DEF_TOPICS
                        ))
                        .title("Topics Not Set")
                        .kind(MessageDialogKind::Warning)
                        .show(|_| {});
                });
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building ntfy-desktop");

    app.run(|handle, event| match event {
        RunEvent::ExitRequested { .. } => {
            let st = handle.state::<AppState>();
            st.shutting_down.store(true, Ordering::SeqCst);
            service::stop_polling(handle);
        }
        #[cfg(target_os = "macos")]
        RunEvent::Reopen { .. } => windows::show_main(handle),
        _ => {}
    });
}
