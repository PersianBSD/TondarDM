//! TondarDM — HTTP client + light probes
//! فاز ۰: ساخت کلاینت امن، HEAD، و fallback به GET با Range=0-0.
//! نویسنده: Ali Asadi | Persian Developer Team | persianbsd@gmail.com

//! HTTP client + light probes (HEAD / GET range=0-0)
//! Author: Ali Asadi | Team: Persian Developer Team | Email: persianbsd@gmail.com

use reqwest::{Client, Response, redirect::Policy};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, REFERER, COOKIE};
use std::time::Duration;
use crate::core::prelude::*;
use crate::ui::cli::Args;

fn build_default_headers(args: &Args) -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(ACCEPT, HeaderValue::from_static("*/*"));
    h.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, br, deflate, zstd"));
    if let Some(ref r) = args.referer {
        if let Ok(v) = HeaderValue::from_str(r) { h.insert(REFERER, v); }
    }
    if let Some(ref c) = args.cookie {
        if let Ok(v) = HeaderValue::from_str(c) { h.insert(COOKIE, v); }
    }
    if let Some(ref ua) = args.ua {
        if let Ok(v) = HeaderValue::from_str(ua) { h.insert(USER_AGENT, v); }
    }
    // parse -H "Key: Value"
    for line in &args.headers {
        if let Some((k,v)) = line.split_once(':') {
            let key = k.trim().to_string();
            let val = v.trim();
            if let Ok(name) = reqwest::header::HeaderName::try_from(key) {
                if let Ok(hv) = HeaderValue::from_str(val) {
                    h.insert(name, hv);
                }
            }
        }
    }
    h
}

/// build_client: reqwest client with timeouts, redirects and rustls
pub fn build_client(args: &Args) -> Result<Client> {
    let default_headers = build_default_headers(args);
    let builder = Client::builder()
        .default_headers(default_headers)
        .user_agent(args.ua.as_deref().unwrap_or(USER_AGENT))
        .gzip(true)
        .brotli(true)
        .zstd(true)
        .redirect(Policy::limited(MAX_REDIRECTS))
        .connect_timeout(Duration::from_secs(CONN_TIMEOUT_SECS))
        .timeout(Duration::from_secs(REQ_TIMEOUT_SECS))
        .use_rustls_tls();

    builder.build().map_err(|e| DmError::Other(e.to_string()))
}

/// send_head: HEAD for metadata
pub async fn send_head(client: &Client, url: &str) -> Result<Response> {
    client.head(url).send().await
        .map_err(|e| DmError::Network(e.to_string()))
}

/// get_range0: fallback GET bytes=0-0 for metadata
pub async fn get_range0(client: &Client, url: &str) -> Result<Response> {
    client.get(url)
        .header(reqwest::header::RANGE, "bytes=0-0")
        .send().await
        .map_err(|e| DmError::Network(e.to_string()))
}
