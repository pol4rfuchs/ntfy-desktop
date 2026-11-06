//! Persistent key/value settings store.
//!
//! Port of `classes/Storage.js`. Settings live in `prefs.json` inside the same
//! per-user directory the Electron version used (`<config dir>/ntfy-desktop`),
//! with the same keys, so existing settings carry over after the switch.

use serde_json::{Map, Value};
use std::{
    fs,
    path::PathBuf,
    sync::Mutex,
};

pub const DEF_INSTANCE_URL: &str = "https://ntfy.sh/app";
pub const DEF_DATETIME: &str = "YYYY-MM-DD hh:mm a";
pub const DEF_TOPICS: &str = "announcements,stats";
pub const DEF_POLLRATE: i64 = 60;
pub const MIN_POLLRATE: i64 = 5;
pub const MAX_POLLRATE: i64 = 3600;

pub struct Store {
    path: PathBuf,
    data: Mutex<Map<String, Value>>,
}

/// Directory for settings; matches Electron's `app.getPath('userData')`.
pub fn user_data_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntfy-desktop")
}

fn defaults() -> Map<String, Value> {
    let v = serde_json::json!({
        "instanceURL": DEF_INSTANCE_URL,
        "apiToken": "",
        "topics": DEF_TOPICS,
        "indicatorMessages": 0,
        "bHotkeys": 0,
        "bDevTools": 0,
        "bQuitOnClose": 0,
        "bStartHidden": 0,
        "bPersistentNoti": 0,
        "bLocalhost": 0,
        "datetime": DEF_DATETIME
    });
    v.as_object().cloned().unwrap_or_default()
}

impl Store {
    pub fn open(path: PathBuf) -> Self {
        let data = fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str::<Value>(&s).ok())
            .and_then(|v| v.as_object().cloned())
            .unwrap_or_else(defaults);
        Store { path, data: Mutex::new(data) }
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        self.data.lock().unwrap().get(key).cloned()
    }

    /// String value, or `None` if missing / not a string.
    pub fn get_str(&self, key: &str) -> Option<String> {
        match self.get(key) {
            Some(Value::String(s)) => Some(s),
            _ => None,
        }
    }

    /// Value with all whitespace removed; `fallback` if missing / not a string.
    pub fn get_sanitized(&self, key: &str, fallback: &str) -> String {
        match self.get_str(key) {
            Some(s) => s.chars().filter(|c| !c.is_whitespace()).collect(),
            None => fallback.to_string(),
        }
    }

    /// Lenient integer read (`Storage.getInt`): numeric strings, bools and
    /// numbers are accepted, everything else is 0.
    pub fn get_int(&self, key: &str) -> i64 {
        value_to_int(self.get(key).as_ref())
    }

    pub fn set(&self, key: &str, val: Value) {
        let mut data = self.data.lock().unwrap();
        data.insert(key.to_string(), val);
        if let Some(dir) = self.path.parent() {
            let _ = fs::create_dir_all(dir);
        }
        match serde_json::to_string(&*data) {
            Ok(json) => {
                if let Err(e) = fs::write(&self.path, json) {
                    tracing::error!("failed to write settings: {e}");
                }
            }
            Err(e) => tracing::error!("failed to serialize settings: {e}"),
        }
    }
}

pub fn value_to_int(v: Option<&Value>) -> i64 {
    match v {
        None | Some(Value::Null) => 0,
        Some(Value::Bool(b)) => *b as i64,
        Some(Value::Number(n)) => n.as_f64().map(|f| f.trunc() as i64).unwrap_or(0),
        Some(Value::String(s)) => {
            let t = s.trim();
            if t.is_empty() {
                return 0;
            }
            if let Some(hex) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
                return i64::from_str_radix(hex, 16).unwrap_or(0);
            }
            t.parse::<f64>().ok().filter(|f| f.is_finite()).map(|f| f.trunc() as i64).unwrap_or(0)
        }
        _ => 0,
    }
}

pub fn clamp_pollrate(rate: i64) -> i64 {
    rate.clamp(MIN_POLLRATE, MAX_POLLRATE)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("ntfy-desktop-test-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        p.join("prefs.json")
    }

    #[test]
    fn defaults_when_file_missing() {
        let s = Store::open(tmp("defaults"));
        assert_eq!(s.get_str("instanceURL").as_deref(), Some(DEF_INSTANCE_URL));
        assert_eq!(s.get_int("bHotkeys"), 0);
    }

    #[test]
    fn set_persists_and_reloads() {
        let p = tmp("persist");
        let s = Store::open(p.clone());
        s.set("topics", Value::String("a,b".into()));
        let s2 = Store::open(p);
        assert_eq!(s2.get_str("topics").as_deref(), Some("a,b"));
    }

    #[test]
    fn get_int_matches_js_semantics() {
        let cases: Vec<(Value, i64)> = vec![
            (Value::Null, 0),
            (Value::Bool(true), 1),
            (Value::Bool(false), 0),
            (Value::String("".into()), 0),
            (Value::String("   ".into()), 0),
            (Value::String("0042".into()), 42),
            (Value::String("0xFF".into()), 255),
            (Value::String("1e3".into()), 1000),
            (Value::String("123abc".into()), 0),
            (Value::String("abc".into()), 0),
            (serde_json::json!(7.9), 7),
            (serde_json::json!(-3.9), -3),
        ];
        for (v, want) in cases {
            assert_eq!(value_to_int(Some(&v)), want, "{v:?}");
        }
        assert_eq!(value_to_int(None), 0);
    }

    #[test]
    fn sanitized_strips_whitespace() {
        let s = Store::open(tmp("san"));
        s.set("topics", Value::String(" a, b ,\tc\n".into()));
        assert_eq!(s.get_sanitized("topics", "x"), "a,b,c");
        assert_eq!(s.get_sanitized("missing", "fallback"), "fallback");
        s.set("n", serde_json::json!(5));
        assert_eq!(s.get_sanitized("n", "fb"), "fb");
    }

    #[test]
    fn pollrate_is_clamped() {
        assert_eq!(clamp_pollrate(1), MIN_POLLRATE);
        assert_eq!(clamp_pollrate(60), 60);
        assert_eq!(clamp_pollrate(99999), MAX_POLLRATE);
    }
}
