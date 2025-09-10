//! Metadata helpers: filename / size / range
//! نویسنده: Ali Asadi | Persian Developer Team | persianbsd@gmail.com

use content_disposition::{parse_content_disposition, ParsedContentDisposition};
use reqwest::header::{HeaderMap, CONTENT_DISPOSITION, CONTENT_LENGTH, ACCEPT_RANGES, CONTENT_RANGE};
use url::Url;

/// infer_filename: نام فایل را از Content-Disposition یا از آخرین سگمنت URL حدس می‌زند.
pub fn infer_filename(url: &str, headers: &HeaderMap) -> String {
    // 1) تلاش از Content-Disposition
    if let Some(v) = headers.get(CONTENT_DISPOSITION) {
        if let Ok(s) = v.to_str() {
            // نسخه 0.4: پارسر تابع سادۀ زیر را می‌دهد
            let parsed: ParsedContentDisposition = parse_content_disposition(s);
            if let Some((fname, _charset)) = parsed.filename() {
                let t = fname.trim().trim_matches('"').to_string();
                if !t.is_empty() {
                    return t;
                }
            }
        }
    }
    // 2) fallback از URL
    Url::parse(url)
        .ok()
        .and_then(|u| u.path_segments()?.last().map(|s| s.to_string()))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "download.bin".to_string())
}

/// parse_size: اندازه فایل را از Content-Length یا Content-Range استخراج می‌کند.
pub fn parse_size(headers: &HeaderMap) -> Option<u64> {
    if let Some(n) = headers.get(CONTENT_LENGTH).and_then(|v| v.to_str().ok()).and_then(|s| s.parse::<u64>().ok()) {
        return Some(n);
    }
    if let Some(cr) = headers.get(CONTENT_RANGE).and_then(|v| v.to_str().ok()) {
        // مثال: "bytes 0-0/12345" → total = 12345
        if let Some(total) = cr.rsplit('/').next().and_then(|t| t.parse::<u64>().ok()) {
            return Some(total);
        }
    }
    None
}

/// supports_range: پشتیبانی سرور از Range را چک می‌کند.
pub fn supports_range(headers: &HeaderMap) -> bool {
    if let Some(v) = headers.get(ACCEPT_RANGES) {
        if let Ok(s) = v.to_str() {
            return s.to_ascii_lowercase().contains("bytes");
        }
    }
    false
}
