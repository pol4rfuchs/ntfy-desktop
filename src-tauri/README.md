# Ntfy Desktop — Tauri / Rust version

Rust port of the Electron client in `../src`, built on [Tauri 2](https://tauri.app).
Same features: tray icon, menu, polling of ntfy topics, native notifications, unread
badge, single instance, the CLI switches (`--hidden --devtools --terminate --hotkeys --env`).

Settings are read from / written to the same `prefs.json` as the Electron version
(`~/.config/ntfy-desktop`, `%APPDATA%\ntfy-desktop`, `~/Library/Application Support/ntfy-desktop`),
so existing settings carry over. **The API token has to be entered once again**: the Electron
build encrypted it with a key only Electron can use. It is now stored in the OS keychain
(falls back to `prefs.json` when no keychain is available).

## Build

Linux needs the WebKitGTK dev packages:

```bash
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libssl-dev libdbus-1-dev pkg-config
```

```bash
cd src-tauri
cargo run                      # dev run
cargo test                     # unit tests
cargo install tauri-cli --version "^2" --locked
cargo tauri build              # installers in target/release/bundle
# or from the repository root: ./build.sh (Linux/macOS) / build.bat (Windows)
```

## Layout

| Path | Purpose |
|---|---|
| `src/poll.rs` | URL building, response parsing, de-duplication, HTTP calls |
| `src/service.rs` | polling loop, notifications, badge, api token, instance loading |
| `src/menu.rs`, `src/windows.rs` | app / tray menu, main / settings / about windows |
| `src/commands.rs` | commands the settings / about pages may call |
| `src/store.rs`, `src/datefmt.rs` | `prefs.json` store, moment.js → chrono date formats |
| `ui/` | bundled settings + about pages (plain HTML/JS, no build step) |
| `capabilities/` | the remote ntfy page may only call `badge_reset`; local pages get the rest |

## Known differences to the Electron version

* Notifications show the ntfy message title when the message has one (`<title>`, then message, topic and date).
* Clicking a notification opens the instance only on Linux; Windows/macOS toasts via `notify-rust` have no click callback.
* `allowRunningInsecureContent` has no equivalent: an HTTPS page talking to a plain-HTTP endpoint may be blocked by the system webview.
* The five settings prompts are one local settings window opened at the matching section.
* The badge uses the platform badge API where it exists, plus the tray tooltip (`Ntfy Desktop (3 new)`).
