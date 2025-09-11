//! TondarDM — HTTP client + light probes
//! فاز ۰: ساخت کلاینت امن، HEAD، و fallback به GET با Range=0-0.
//! نویسنده: Ali Asadi | Persian Developer Team | persianbsd@gmail.com

//! HTTP client + light probes (HEAD / GET range=0-0)
//! Author: Ali Asadi | Team: Persian Developer Team | Email: persianbsd@gmail.com

//! HTTP client + light probes
use reqwest::{Client, Response, redirect::Policy};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, REFERER, COOKIE};
use std::time::Duration;

use crate::engine::prelude::*;
use crate::engine::consts::*;

use crate::ui::cli::Args;

pub fn build_client(args: &Args) -> Result<Client> {
    let mut default_headers = HeaderMap::new();
    default_headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    default_headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, br, deflate, zstd"));
    if let Some(ref r) = args.referer {
        if let Ok(v) = HeaderValue::from_str(r) { default_headers.insert(REFERER, v); }
    }
    if let Some(ref c) = args.cookie {
        if let Ok(v) = HeaderValue::from_str(c) { default_headers.insert(COOKIE, v); }
    }
    // UA را با builder ست می‌کنیم (نه هدر دستی)
    let builder = Client::builder()
        .default_headers(default_headers)
        .user_agent(args.ua.as_deref().unwrap_or(USER_AGENT))
        .gzip(true).brotli(true).zstd(true)
        .redirect(Policy::limited(MAX_REDIRECTS))
        .connect_timeout(Duration::from_secs(CONN_TIMEOUT_SECS))
        .timeout(Duration::from_secs(REQ_TIMEOUT_SECS))
        .use_rustls_tls();

    builder.build().map_err(|e| DmError::Other(e.to_string()))
}

pub async fn send_head(client: &Client, url: &str) -> Result<Response> {
    client.head(url).send().await
        .map_err(|e| DmError::Network(e.to_string()))
}

pub async fn get_range0(client: &Client, url: &str) -> Result<Response> {
    client.get(url)
        .header(reqwest::header::RANGE, "bytes=0-0")
        .send().await
        .map_err(|e| DmError::Network(e.to_string()))
}
