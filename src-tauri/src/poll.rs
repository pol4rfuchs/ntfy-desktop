//! ntfy polling: URL building, response parsing, de-duplication and the
//! network calls. Port of `GetMessages` / `GetMessageData` / `IsValidUrl`
//! from the Electron `index.js`.

use serde_json::Value;
use std::{collections::VecDeque, time::Duration};

pub const POLL_OVERLAP_SECS: i64 = 5;
pub const MAX_MSG_HISTORY: usize = 1000;
pub const ALLOWED_EXTERNAL_PROTOCOLS: [&str; 3] = ["http", "https", "mailto"];

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub id: String,
    pub time: i64,
    pub topic: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Warning {
    /// A line was not valid JSON (instance probably not a ntfy server).
    InvalidJson,
    Unauthorized { link: String },
    RateLimited { link: String },
    ServerError { error: String, code: i64, link: String },
}

#[derive(Debug, Default, PartialEq)]
pub struct Parsed {
    pub messages: Vec<Message>,
    pub warnings: Vec<Warning>,
}

/// Remembers recently shown message ids so the overlapping `since` window
/// never produces duplicate notifications.
#[derive(Default)]
pub struct History {
    ids: VecDeque<String>,
}

impl History {
    pub fn contains(&self, id: &str) -> bool {
        self.ids.iter().any(|i| i == id)
    }

    /// Returns `true` if the id was new (and is now remembered).
    pub fn insert(&mut self, id: &str) -> bool {
        if self.contains(id) {
            return false;
        }
        self.ids.push_back(id.to_string());
        while self.ids.len() > MAX_MSG_HISTORY {
            self.ids.pop_front();
        }
        true
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.ids.len()
    }

}

/// `since` window: whole interval since the last successful poll plus a small
/// overlap; the very first poll just looks back one poll interval.
pub fn compute_since(last_poll_start_ms: Option<i64>, now_ms: i64, pollrate: i64) -> i64 {
    match last_poll_start_ms {
        None => pollrate,
        Some(last) => {
            let elapsed = (now_ms - last).max(0);
            (elapsed + 999) / 1000 + POLL_OVERLAP_SECS
        }
    }
}

/// Build the JSON poll URL. `https://ntfy.sh/app` is the web UI of the
/// official instance; the API lives at the host root.
pub fn build_poll_url(instance: &str, topics: &str, since_secs: i64) -> String {
    let base = instance.trim_end_matches('/');
    let uri = format!("{base}/{topics}/json?since={since_secs}s&poll=1");
    uri.replacen("ntfy.sh/app", "ntfy.sh", 1)
}

/// Parse the newline separated JSON body returned by `/json?poll=1`.
pub fn parse_body(body: &str) -> Parsed {
    let mut out = Parsed::default();
    for line in body.split('\n').filter(|l| !l.is_empty()) {
        let Ok(v) = serde_json::from_str::<Value>(line) else {
            out.warnings.push(Warning::InvalidJson);
            continue;
        };
        let link = v.get("link").and_then(Value::as_str).unwrap_or("").to_string();
        let http = v.get("http").and_then(Value::as_i64).unwrap_or(0);

        if http == 401 {
            out.warnings.push(Warning::Unauthorized { link });
            continue;
        }
        if http == 429 {
            out.warnings.push(Warning::RateLimited { link });
            continue;
        }
        if let Some(err) = v.get("error").and_then(Value::as_str) {
            out.warnings.push(Warning::ServerError { error: err.to_string(), code: http, link });
        }
        if v.get("event").and_then(Value::as_str) != Some("message") {
            continue;
        }
        let s = |k: &str| v.get(k).and_then(Value::as_str).unwrap_or("").to_string();
        out.messages.push(Message {
            id: s("id"),
            time: v.get("time").and_then(Value::as_i64).unwrap_or(0),
            topic: s("topic"),
            message: s("message"),
        });
    }
    out
}

/// True if the url may be handed to the operating system.
pub fn is_allowed_external(url: &str) -> bool {
    url::Url::parse(url)
        .map(|u| ALLOWED_EXTERNAL_PROTOCOLS.contains(&u.scheme()))
        .unwrap_or(false)
}

/// True if navigating to `target` should stay inside the app window: same host
/// as the configured instance or the official one.
pub fn is_internal_navigation(target: &url::Url, instance: &str, official: &str) -> bool {
    let host = |s: &str| url::Url::parse(s).ok().and_then(|u| u.host_str().map(str::to_string));
    match target.scheme() {
        "tauri" | "about" | "data" | "blob" => return true,
        _ => {}
    }
    let Some(t) = target.host_str() else { return true };
    if t == "localhost" && target.scheme() == "http" && host(instance).as_deref() == Some("localhost") {
        return true;
    }
    host(instance).as_deref() == Some(t) || host(official).as_deref() == Some(t)
}

pub fn user_agent() -> String {
    format!("ntfy-desktop/{}", env!("CARGO_PKG_VERSION"))
}

pub fn http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent(user_agent())
        .build()
        .expect("failed to build http client")
}

/// Fetch the raw poll body. `None` on network / timeout errors.
pub async fn fetch_body(client: &reqwest::Client, uri: &str, token: &str) -> Option<String> {
    let mut req = client
        .get(uri)
        .timeout(Duration::from_secs(30))
        .header("Accept", "application/json")
        .header("Cache-Control", "no-cache");
    if !token.is_empty() {
        req = req.bearer_auth(token);
    }
    match req.send().await {
        Ok(resp) => match resp.text().await {
            Ok(t) => Some(t),
            Err(e) => {
                tracing::error!("failed to read response from {uri}: {e}");
                None
            }
        },
        Err(e) if e.is_timeout() => {
            tracing::warn!("request to ntfy server timed out: {uri}");
            None
        }
        Err(e) => {
            tracing::error!("failed to get messages from {uri}: {e}");
            None
        }
    }
}

/// Check that an instance url answers. Any status below 500 counts as
/// reachable (a 401/403 still means a live server). Tries HEAD first, then GET.
pub async fn is_valid_url(client: &reqwest::Client, uri: &str, tries: u32, delay: Duration) -> Result<(), String> {
    let mut last_err = String::from("unknown error");
    for attempt in 0..tries.max(1) {
        for (method, secs) in [(reqwest::Method::HEAD, 10u64), (reqwest::Method::GET, 5)] {
            let res = client
                .request(method, uri)
                .timeout(Duration::from_secs(secs))
                .header("Accept", "*/*")
                .header("Cache-Control", "no-cache")
                .send()
                .await;
            match res {
                Ok(r) if r.status().as_u16() < 500 => return Ok(()),
                Ok(r) => last_err = format!("Server error: {}", r.status().as_u16()),
                Err(e) if e.is_timeout() => last_err = "timeout".into(),
                Err(e) => last_err = e.to_string(),
            }
        }
        if attempt + 1 < tries {
            tokio::time::sleep(delay).await;
        }
    }
    Err(format!("Failed to resolve URL after {tries} attempts: {last_err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn since_first_poll_uses_pollrate() {
        assert_eq!(compute_since(None, 1_000_000, 60), 60);
    }

    #[test]
    fn since_covers_gap_plus_overlap() {
        // 60.2s elapsed -> ceil = 61, + 5 overlap
        assert_eq!(compute_since(Some(0), 60_200, 60), 66);
        assert_eq!(compute_since(Some(0), 60_000, 60), 65);
        // clock going backwards never yields a negative window
        assert_eq!(compute_since(Some(10_000), 0, 60), 5);
    }

    #[test]
    fn url_for_selfhosted_and_official() {
        assert_eq!(
            build_poll_url("https://ntfy.example.org/", "a,b", 65),
            "https://ntfy.example.org/a,b/json?since=65s&poll=1"
        );
        assert_eq!(
            build_poll_url("https://ntfy.sh/app", "a", 60),
            "https://ntfy.sh/a/json?since=60s&poll=1"
        );
        assert_eq!(
            build_poll_url("http://192.168.1.2:8080///", "a", 60),
            "http://192.168.1.2:8080/a/json?since=60s&poll=1"
        );
    }

    #[test]
    fn parses_messages_and_ignores_other_events() {
        let body = concat!(
            r#"{"id":"1","time":100,"event":"open","topic":"t"}"#, "\n",
            r#"{"id":"2","time":101,"event":"message","topic":"t","message":"hello"}"#, "\n",
            r#"{"id":"3","time":102,"event":"keepalive","topic":"t"}"#, "\n",
        );
        let p = parse_body(body);
        assert_eq!(
            p.messages,
            vec![Message { id: "2".into(), time: 101, topic: "t".into(), message: "hello".into() }]
        );
        assert!(p.warnings.is_empty());
    }

    #[test]
    fn invalid_lines_are_skipped_not_fatal() {
        let body = "<html>nope</html>\n{\"id\":\"9\",\"event\":\"message\",\"topic\":\"x\",\"message\":\"m\",\"time\":1}\n";
        let p = parse_body(body);
        assert_eq!(p.messages.len(), 1);
        assert_eq!(p.warnings, vec![Warning::InvalidJson]);
    }

    #[test]
    fn auth_and_rate_limit_errors() {
        let body = concat!(
            r#"{"code":40101,"http":401,"error":"unauthorized","link":"https://docs/x"}"#, "\n",
            r#"{"code":42909,"http":429,"error":"limit reached","link":"https://docs/y"}"#, "\n",
            r#"{"code":50001,"http":500,"error":"boom","link":"l"}"#, "\n",
        );
        let p = parse_body(body);
        assert!(p.messages.is_empty());
        assert_eq!(p.warnings.len(), 3);
        assert!(matches!(p.warnings[0], Warning::Unauthorized { .. }));
        assert!(matches!(p.warnings[1], Warning::RateLimited { .. }));
        assert!(matches!(p.warnings[2], Warning::ServerError { code: 500, .. }));
    }

    #[test]
    fn history_dedupes_and_is_bounded() {
        let mut h = History::default();
        assert!(h.insert("a"));
        assert!(!h.insert("a"));
        for i in 0..(MAX_MSG_HISTORY + 50) {
            h.insert(&format!("id{i}"));
        }
        assert_eq!(h.len(), MAX_MSG_HISTORY);
        assert!(!h.contains("a"));
        assert!(h.contains(&format!("id{}", MAX_MSG_HISTORY + 49)));
    }

    #[test]
    fn external_protocol_allowlist() {
        assert!(is_allowed_external("https://example.org"));
        assert!(is_allowed_external("http://example.org"));
        assert!(is_allowed_external("mailto:a@b.c"));
        assert!(!is_allowed_external("file:///etc/passwd"));
        assert!(!is_allowed_external("smb://host/share"));
        assert!(!is_allowed_external("javascript:alert(1)"));
        assert!(!is_allowed_external("not a url"));
    }

    #[test]
    fn navigation_stays_on_instance_or_official() {
        let u = |s: &str| url::Url::parse(s).unwrap();
        let inst = "https://ntfy.example.org/";
        let off = "https://ntfy.sh/app";
        assert!(is_internal_navigation(&u("https://ntfy.example.org/sub"), inst, off));
        assert!(is_internal_navigation(&u("https://ntfy.sh/app"), inst, off));
        assert!(!is_internal_navigation(&u("https://evil.example.com"), inst, off));
    }
}
