//! Inspect: probe a URL and print metadata (HEAD or GET bytes=0-0)
//! Author: Ali Asadi | Team: Persian Developer Team | Email: persianbsd@gmail.com
//! Public API (≤3 pub functions rule observed):
//! - pub enum ProbeMode
//! - pub struct MetaInfo
//! - pub fn probe_url(...)
//! - pub fn print_table(...)

use reqwest::{Client, StatusCode};
use reqwest::header::{
    HeaderMap, CONTENT_DISPOSITION, CONTENT_LENGTH, ACCEPT_RANGES, CONTENT_RANGE, ETAG, LAST_MODIFIED,
};
use content_disposition::{parse_content_disposition, ParsedContentDisposition};
use url::Url;

#[derive(Debug, Clone, Copy)]
pub enum ProbeMode {
    /// Try HEAD; if it fails or is non-success → fallback GET 0-0
    Auto,
    /// Force HEAD
    Head,
    /// Force GET with Range: bytes=0-0
    GetRange0,
}

#[derive(Debug, Clone)]
pub struct MetaInfo {
    pub final_url: String,
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub filename: String,
    pub size: Option<u64>,
    pub accept_ranges: bool,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

/// probe_url: performs HEAD or GET 0-0 (based on mode) and returns metadata of the *final* response
pub async fn probe_url(client: &Client, url: &str, mode: ProbeMode) -> Result<MetaInfo, String> {
    let resp = match mode {
        ProbeMode::Head => super::request::head(client, url).await?,
        ProbeMode::GetRange0 => super::request::get_range0(client, url).await?,
        ProbeMode::Auto => match super::request::head(client, url).await {
            Ok(r) if r.status().is_success() => r,
            Ok(_) | Err(_) => super::request::get_range0(client, url).await?,
        },
    };

    let status = resp.status();
    let final_url = resp.url().to_string();
    let headers = resp.headers().clone();

    let filename = infer_filename(&final_url, &headers);
    let size = parse_size(&headers);
    let accept_ranges = supports_range(&headers);
    let etag = headers
        .get(ETAG)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let last_modified = headers
        .get(LAST_MODIFIED)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    Ok(MetaInfo {
        final_url,
        status,
        headers,
        filename,
        size,
        accept_ranges,
        etag,
        last_modified,
    })
}

/// print_table: pretty-print key info and all headers
pub fn print_table(meta: &MetaInfo) {
    println!("Status   : {}", meta.status);
    println!("FinalURL : {}", meta.final_url);
    println!("Filename : {}", meta.filename);
    match meta.size {
        Some(n) => println!("Size     : {} bytes", n),
        None => println!("Size     : (unknown)"),
    }
    println!(
        "Ranges   : {}",
        if meta.accept_ranges {
            "supported"
        } else {
            "not supported/unknown"
        }
    );
    if let Some(e) = &meta.etag {
        println!("ETag     : {e}");
    }
    if let Some(lm) = &meta.last_modified {
        println!("Last-Mod : {lm}");
    }

    println!("\n+--------------------------+------------------------------------------+");
    println!("| {:24} | {:40} |", "Header", "Value");
    println!("+--------------------------+------------------------------------------+");
    for (k, v) in meta.headers.iter() {
        let key = k.as_str();
        let val = v.to_str().unwrap_or("<non-UTF8>");
        println!("| {:24} | {:40} |", key, truncate(val, 40));
    }
    println!("+--------------------------+------------------------------------------+");
}

// ---------- private helpers ----------

fn infer_filename(url: &str, headers: &HeaderMap) -> String {
    if let Some(v) = headers
        .get(CONTENT_DISPOSITION)
        .and_then(|h| h.to_str().ok())
    {
        let parsed: ParsedContentDisposition = parse_content_disposition(v);
        if let Some((fname, _)) = parsed.filename() {
            let t = fname.trim().trim_matches('"').to_string();
            if !t.is_empty() {
                return t;
            }
        }
    }
    Url::parse(url)
        .ok()
        .and_then(|u| u.path_segments()?.last().map(|s| s.to_string()))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "download.bin".to_string())
}

fn parse_size(headers: &HeaderMap) -> Option<u64> {
    if let Some(n) = headers
        .get(CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
    {
        return Some(n);
    }
    if let Some(cr) = headers.get(CONTENT_RANGE).and_then(|v| v.to_str().ok()) {
        if let Some(total) = cr.rsplit('/').next().and_then(|t| t.parse::<u64>().ok()) {
            return Some(total);
        }
    }
    None
}

fn supports_range(headers: &HeaderMap) -> bool {
    if let Some(v) = headers
        .get(ACCEPT_RANGES)
        .and_then(|h| h.to_str().ok())
    {
        return v.to_ascii_lowercase().contains("bytes");
    }
    false
}

fn truncate(s: &str, n: usize) -> String {
    if s.len() <= n {
        s.to_string()
    } else {
        format!("{}…", &s[..n.saturating_sub(1)])
    }
}
