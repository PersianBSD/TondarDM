//! Author: Ali Asadi | Team: Persian Developer Team | Email: persianbsd@gmail.com
//! /// Normalize known “wrapper” URLs to the real target.

use percent_encoding::percent_decode_str;
use url::Url;

/// Normalize known “wrapper” URLs to the real target.
/// Currently supports: href.li/?<absolute_url>
pub fn normalize_url(input: &str) -> String {
    if let Ok(u) = Url::parse(input) {
        if u.host_str().map(|h| h.eq_ignore_ascii_case("href.li")).unwrap_or(false) {
            // href.li carries the target url in the query – either raw or percent-encoded
            if let Some(q) = u.query() {
                // q may itself start with "https://" (raw) or be encoded.
                let candidate = if q.starts_with("http://") || q.starts_with("https://") {
                    q.to_string()
                } else {
                    percent_decode_str(q).decode_utf8_lossy().to_string()
                };
                if Url::parse(&candidate).is_ok() {
                    return candidate;
                }
            }
        }
    }
    input.to_string()
}
