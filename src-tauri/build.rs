fn main() {
    // Commit the binary was built from; shown in the window title and About page
    // so different builds with the same version can be told apart.
    let hash = std::env::var("NTFY_BUILD_SHA")
        .or_else(|_| std::env::var("GITHUB_SHA"))
        .ok()
        .map(|h| h.chars().take(7).collect::<String>())
        .filter(|h| !h.is_empty())
        .or_else(|| {
            std::process::Command::new("git")
                .args(["rev-parse", "--short=7", "HEAD"])
                .output()
                .ok()
                .filter(|o| o.status.success())
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        })
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=NTFY_BUILD={hash}");
    println!("cargo:rerun-if-env-changed=NTFY_BUILD_SHA");
    println!("cargo:rerun-if-env-changed=GITHUB_SHA");
    println!("cargo:rerun-if-changed=../.git/HEAD");

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
