//! Build client + simple probes (HEAD / GET 0-0)
//! - build_client(): reqwest client with sane defaults
//! - head(): send HEAD
//! - get_range0(): GET with Range: bytes=0-0
//! Author: Ali Asadi | Team: Persian Developer Team | Email: persianbsd@gmail.com

use reqwest::{Client, Response, redirect::Policy};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, REFERER, COOKIE, USER_AGENT};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ClientOpts {
    pub referer: Option<String>,
    pub cookie: Option<String>,
    pub ua: Option<String>,
    pub extra_headers: Vec<(String, String)>,
    pub max_redirects: usize,
    pub conn_timeout_secs: u64,
    pub req_timeout_secs: u64,
}

/// build_client: reqwest client with defaults + custom headers
pub fn build_client(opts: &ClientOpts) -> Result<Client, String> {
    let mut h = HeaderMap::new();
    h.insert(ACCEPT, HeaderValue::from_static("*/*"));
    h.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, br, deflate, zstd"));
    if let Some(r) = &opts.referer {
        if let Ok(v) = HeaderValue::from_str(r) { h.insert(REFERER, v); }
    }
    if let Some(c) = &opts.cookie {
        if let Ok(v) = HeaderValue::from_str(c) { h.insert(COOKIE, v); }
    }
    if let Some(ua) = &opts.ua {
        if let Ok(v) = HeaderValue::from_str(ua) { h.insert(USER_AGENT, v); }
    }

    for (k, v) in &opts.extra_headers {
        if let (Ok(name), Ok(val)) = (
            reqwest::header::HeaderName::try_from(k.as_str()),
            HeaderValue::from_str(v),
        ) {
            h.insert(name, val);
        }
    }

    reqwest::Client::builder()
        .default_headers(h)
        .redirect(Policy::limited(opts.max_redirects))
        .connect_timeout(Duration::from_secs(opts.conn_timeout_secs))
        .timeout(Duration::from_secs(opts.req_timeout_secs))
        .use_rustls_tls()
        .gzip(true).brotli(true).zstd(true)
        .build()
        .map_err(|e| e.to_string())
}

/// head: send HEAD (some servers block it)
pub async fn head(client: &Client, url: &str) -> Result<Response, String> {
    client.head(url).send().await.map_err(|e| e.to_string())
}

/// get_range0: GET one byte to reveal Content-Range/Length
pub async fn get_range0(client: &Client, url: &str) -> Result<Response, String> {
    client.get(url)
        .header(reqwest::header::RANGE, "bytes=0-0")
        .send().await
        .map_err(|e| e.to_string())
}
