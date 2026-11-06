//! Converts moment.js style format strings (as used in the old Electron
//! settings, default `YYYY-MM-DD hh:mm a`) to chrono output.

use chrono::{DateTime, Local, TimeZone};

/// Tokens ordered longest-first so greedy matching works.
const TOKENS: &[(&str, &str)] = &[
    ("YYYY", "%Y"),
    ("MMMM", "%B"),
    ("dddd", "%A"),
    ("MMM", "%b"),
    ("ddd", "%a"),
    ("YY", "%y"),
    ("MM", "%m"),
    ("DD", "%d"),
    ("HH", "%H"),
    ("hh", "%I"),
    ("mm", "%M"),
    ("ss", "%S"),
    ("ZZ", "%z"),
    ("M", "%-m"),
    ("D", "%-d"),
    ("H", "%-H"),
    ("h", "%-I"),
    ("m", "%-M"),
    ("s", "%-S"),
    ("A", "%p"),
    ("a", "%P"),
    ("Z", "%:z"),
];

/// Translate a moment.js format string into a chrono (strftime) one.
/// Text in `[brackets]` is copied literally, like in moment.
pub fn moment_to_strftime(fmt: &str) -> String {
    let mut out = String::new();
    let mut rest = fmt;
    'outer: while !rest.is_empty() {
        if let Some(stripped) = rest.strip_prefix('[') {
            if let Some(end) = stripped.find(']') {
                out.push_str(&stripped[..end].replace('%', "%%"));
                rest = &stripped[end + 1..];
                continue;
            }
        }
        for (tok, repl) in TOKENS {
            if let Some(r) = rest.strip_prefix(tok) {
                out.push_str(repl);
                rest = r;
                continue 'outer;
            }
        }
        let ch = rest.chars().next().unwrap();
        if ch == '%' {
            out.push_str("%%");
        } else {
            out.push(ch);
        }
        rest = &rest[ch.len_utf8()..];
    }
    out
}

/// Format a unix timestamp in local time using a moment.js style format.
pub fn format_unix(ts: i64, fmt: &str) -> String {
    let dt: DateTime<Local> = Local
        .timestamp_opt(ts, 0)
        .single()
        .unwrap_or_else(|| Local.timestamp_opt(0, 0).unwrap());
    format_datetime(&dt, fmt)
}

pub fn format_datetime<Tz: TimeZone>(dt: &DateTime<Tz>, fmt: &str) -> String
where
    Tz::Offset: std::fmt::Display,
{
    use std::fmt::Write;
    let f = moment_to_strftime(fmt);
    let mut s = String::new();
    // `write!` returns Err on invalid specifiers instead of panicking.
    if write!(s, "{}", dt.format(&f)).is_err() {
        return dt.to_rfc3339();
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn default_format() {
        let dt = Utc.with_ymd_and_hms(2025, 3, 7, 15, 4, 9).unwrap();
        assert_eq!(format_datetime(&dt, "YYYY-MM-DD hh:mm a"), "2025-03-07 03:04 pm");
        assert_eq!(format_datetime(&dt, "YYYY-MM-DD HH:mm:ss"), "2025-03-07 15:04:09");
    }

    #[test]
    fn unpadded_and_names() {
        let dt = Utc.with_ymd_and_hms(2025, 3, 7, 5, 4, 9).unwrap();
        assert_eq!(format_datetime(&dt, "D/M/YY h:m:s A"), "7/3/25 5:4:9 AM");
        assert_eq!(format_datetime(&dt, "dddd, MMMM D"), "Friday, March 7");
        assert_eq!(format_datetime(&dt, "ddd MMM DD"), "Fri Mar 07");
    }

    #[test]
    fn brackets_and_percent_are_literal() {
        let dt = Utc.with_ymd_and_hms(2025, 3, 7, 5, 4, 9).unwrap();
        assert_eq!(format_datetime(&dt, "[Year] YYYY 100%"), "Year 2025 100%");
    }
}
