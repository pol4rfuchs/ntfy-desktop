fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new().app_manifest(
            tauri_build::AppManifest::new().commands(&[
                "badge_reset",
                "get_settings",
                "save_settings",
                "set_api_token",
                "get_app_info",
                "open_external",
                "close_window",
            ]),
        ),
    )
    .expect("failed to run tauri build");
}
