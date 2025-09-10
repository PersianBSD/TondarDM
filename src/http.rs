//! HTTP client + light probes
//! نویسنده: Ali Asadi | Persian Developer Team | persianbsd@gmail.com

use reqwest::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING};
use crate::core::prelude::*;

pub fn build_client() -> Result<Client> {
    // هدرهای پیش‌فرض ملایم (بعضی سرورها روی Accept/Encoding حساس‌اند)
    let mut default_headers = HeaderMap::new();
    default_headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    default_headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, br, deflate, zstd"));

    let c = Client::builder()
        .default_headers(default_headers)
        .user_agent(USER_AGENT)
        .gzip(true)
        .brotli(true)
        .zstd(true)
        // .http2_prior_knowledge() // اگر می‌خواهی فقط-H2 باشه این رو فعال کن، بدون آرگومان
        .use_rustls_tls()
        .build()
        .map_err(|e| DmError::Other(e.to_string()))?;
    Ok(c)
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
