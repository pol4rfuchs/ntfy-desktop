//! Shared application state and command line options.

use crate::{poll::History, store::Store};
use std::sync::{
    atomic::{AtomicBool, AtomicI64},
    Mutex,
};

/// Command line switches (same as the Electron version).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Cli {
    /// `--hidden`: start minimised to the tray
    pub hidden: bool,
    /// `--devtools`: add developer tools to the menu
    pub devtools: bool,
    /// `--terminate`: close button quits instead of hiding to tray
    pub terminate: bool,
    /// `--hotkeys`: enable keyboard shortcuts
    pub hotkeys: bool,
    /// `--env <name>`: production | development | test
    pub env: String,
    /// `--help` / `-h`
    pub help: bool,
}

impl Cli {
    pub fn parse<I: IntoIterator<Item = String>>(args: I) -> Cli {
        let mut cli = Cli { env: "production".into(), ..Default::default() };
        let args: Vec<String> = args.into_iter().collect();
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--hidden" => cli.hidden = true,
                "--devtools" => cli.devtools = true,
                "--terminate" => cli.terminate = true,
                "--hotkeys" => cli.hotkeys = true,
                "--help" | "-h" => cli.help = true,
                "--env" if i + 1 < args.len() => {
                    let v = args[i + 1].to_lowercase();
                    if ["production", "development", "test"].contains(&v.as_str()) {
                        cli.env = v;
                        i += 1;
                    } else {
                        eprintln!("Unknown environment: {v}, defaulting to production.");
                        eprintln!("Valid environments: production, development, test");
                    }
                }
                _ => {}
            }
            i += 1;
        }
        cli
    }
}

pub const HELP: &str = "\
Usage:
    ntfy-desktop [options]

Options:
    --hidden          Start the app minimized to the system tray
    --devtools        Enable developer tools in the app menu
    --terminate       Quit the app fully when the close button is pressed
                      (default: close button minimizes to tray)
    --hotkeys         Enable keyboard shortcuts (Ctrl+R, Ctrl+Q, Ctrl+M, zoom, etc.)
    --env <name>      Set runtime environment: production, development, test
    --help, -h        Show this help message and exit

Settings such as instance URL, topics, and polling rate are configured
from within the app: App -> Settings
";

pub struct AppState {
    pub cli: Cli,
    pub store: Store,
    pub client: reqwest::Client,
    pub history: Mutex<History>,
    /// Start time (unix ms) of the last successful poll.
    pub last_poll_start: Mutex<Option<i64>>,
    pub polling: AtomicBool,
    pub shutting_down: AtomicBool,
    /// Set when the configured instance url could not be reached.
    pub bad_url: AtomicBool,
    /// Error banner shown at the bottom of the page after load.
    pub status_msg: Mutex<Option<String>>,
    pub poll_task: Mutex<Option<tauri::async_runtime::JoinHandle<()>>>,
    pub zoom: Mutex<f64>,
    pub badge: AtomicI64,
    /// Cached api token (`None` = not loaded yet) so the keychain is not hit on every poll.
    pub token_cache: Mutex<Option<String>>,
}

impl AppState {
    pub fn new(cli: Cli, store: Store) -> Self {
        let badge = store.get_int("indicatorMessages");
        AppState {
            cli,
            store,
            client: crate::poll::http_client(),
            history: Mutex::new(History::default()),
            last_poll_start: Mutex::new(None),
            polling: AtomicBool::new(false),
            shutting_down: AtomicBool::new(false),
            bad_url: AtomicBool::new(false),
            status_msg: Mutex::new(None),
            poll_task: Mutex::new(None),
            zoom: Mutex::new(1.0),
            badge: AtomicI64::new(badge.max(0)),
            token_cache: Mutex::new(None),
        }
    }

    pub fn hotkeys_enabled(&self) -> bool {
        self.cli.hotkeys || self.store.get_int("bHotkeys") == 1
    }

    pub fn devtools_enabled(&self) -> bool {
        self.cli.devtools || self.store.get_int("bDevTools") == 1
    }

    pub fn quit_on_close(&self) -> bool {
        self.cli.terminate || self.store.get_int("bQuitOnClose") == 1
    }

    pub fn start_hidden(&self) -> bool {
        self.cli.hidden || self.store.get_int("bStartHidden") == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parses_flags() {
        let c = Cli::parse(args(&["--hidden", "--devtools", "--terminate", "--hotkeys"]));
        assert!(c.hidden && c.devtools && c.terminate && c.hotkeys && !c.help);
        assert_eq!(c.env, "production");
    }

    #[test]
    fn parses_env_and_help() {
        let c = Cli::parse(args(&["--env", "Development", "-h"]));
        assert_eq!(c.env, "development");
        assert!(c.help);
    }

    #[test]
    fn invalid_env_is_ignored() {
        let c = Cli::parse(args(&["--env", "bogus"]));
        assert_eq!(c.env, "production");
    }
}
