//! Single-part download with resume & progress (uses final URL + If-Range)
//! Author: Ali Asadi | Team: Persian Developer Team | Email: persianbsd@gmail.com

//! Single-part download with resume & progress (final URL + If-Range)

use crate::engine::prelude::*;
use crate::iox::file as iox;

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Client, Response, StatusCode};
use tokio::fs::File;

use tokio::io::{AsyncWriteExt, AsyncSeekExt};
use std::cmp::min;
use std::io::SeekFrom;


pub async fn download_single(
    client: &Client,
    url: &str,
    filename: &str,
    size: Option<u64>,
    ranges_supported: bool,
    etag: Option<&str>,
    last_modified: Option<&str>,
) -> Result<()> {
    let (mut file, existing) = iox::open_for_resume(filename).await?;

    // اگر سایز کل را می‌دانیم و فایل از آن بزرگتر شده، کوچک کن
    let mut start_offset = match size {
        Some(total) if existing > total => {
            // فایل را به total ببُر تا خراب نشود
            file.set_len(total).await.map_err(DmError::Io)?;
            total
        }
        _ => existing,
    };

    // اگر سرور رنج ندارد ولی فایل چیزی دارد → از صفر
    if !ranges_supported && start_offset > 0 {
        file.set_len(0).await.map_err(DmError::Io)?;
        start_offset = 0;
    }

    // *** preallocate را موقتاً خاموش کنیم تا اندازهٔ فایل معیار اشتباه نشود ***
    // iox::preallocate_if_needed(&file, size, start_offset, false).await?;

    // اگر total معلوم است و offset == total → فایل کامل است
    if let Some(total) = size {
        if start_offset >= total {
            eprintln!("Already complete (local {} bytes, total {}). Skipping.", start_offset, total);
            return Ok(());
        }
    }

    // درخواست
    let mut resp = make_request(client, url, ranges_supported, start_offset, etag, last_modified).await?;

    // اگر 416 گرفتیم و total را می‌دانیم، رفتار درست:
    if resp.status() == StatusCode::RANGE_NOT_SATISFIABLE {
        if let Some(total) = size {
            if start_offset >= total {
                eprintln!("Server says 416 but file already complete. Skipping.");
                return Ok(());
            } else {
                // برخی CDNها از offset فعلی خوششان نمی‌آید؛ یک تلاش از offset-1 بکنیم
                // سپس اولین بایت دریافتی را دور می‌ریزیم.
                let backoff = start_offset.saturating_sub(1);
                eprintln!("416 at offset {start_offset}; retrying from {backoff}…");
                resp = make_request(client, url, ranges_supported, backoff, etag, last_modified).await?;
                // در استریم، اولین بایت را نادیده می‌گیریم؛ پس کرسر را روی start_offset قرار بده
                file.seek(SeekFrom::Start(start_offset)).await.map_err(DmError::Io)?;
                return run_stream_to_file(file, resp, size, start_offset).await;
            }
        }
    }

    // اعتبارسنجی رزوم (پذیرش 206 یا 200 با remainder)
    if start_offset > 0 && ranges_supported {
        use reqwest::header::CONTENT_RANGE;
        let st = resp.status();
        let has_cr = resp.headers().get(CONTENT_RANGE).is_some();
        let cl = resp.content_length();
        let total = size;

        let resume_ok = match st {
            StatusCode::PARTIAL_CONTENT => true,
            StatusCode::OK => {
                if has_cr {
                    true
                } else if let (Some(len), Some(t)) = (cl, total) {
                    len + start_offset == t
                } else { false }
            }
            _ => false,
        };

        if !resume_ok {
            eprintln!("Server did not clearly return resume; restarting from zero.");
            file.set_len(0).await.map_err(DmError::Io)?;
            start_offset = 0;
            resp = make_request(client, url, false, 0, None, None).await?;
        }
    }

    // **نکتهٔ حیاتی**: قبل از نوشتن، حتماً به start_offset seek کن
    file.seek(SeekFrom::Start(start_offset)).await.map_err(DmError::Io)?;

    run_stream_to_file(file, resp, size, start_offset).await
}

// async fn make_request(
//     client: &Client,
//     url: &str,
//     ranges_supported: bool,
//     start_offset: u64,
//     etag: Option<&str>,
//     last_modified: Option<&str>,
// ) -> Result<Response> {
//     use reqwest::header::{IF_RANGE, RANGE};
//     let mut req = client.get(url);
//     if ranges_supported && start_offset > 0 {
//         req = req.header(RANGE, format!("bytes={}-", start_offset));
//         if let Some(tag) = etag {
//             req = req.header(IF_RANGE, tag);
//         } else if let Some(lm) = last_modified {
//             req = req.header(IF_RANGE, lm);
//         }
//     }
//     req.send().await.map_err(|e| DmError::Network(e.to_string()))
// }

async fn make_request(
    client: &Client,
    url: &str,
    ranges_supported: bool,
    start_offset: u64,
    etag: Option<&str>,
    last_modified: Option<&str>,
) -> Result<Response> {
    use reqwest::header::{IF_RANGE, RANGE};
    let mut req = client.get(url);

    if ranges_supported && start_offset > 0 {
        req = req.header(RANGE, format!("bytes={}-", start_offset));

        // If-Range only when explicitly provided (via CLI flag in main)
        if let Some(tag) = etag {
            req = req.header(IF_RANGE, tag);
        } else if let Some(lm) = last_modified {
            req = req.header(IF_RANGE, lm);
        }
    }

    let resp = req.send().await.map_err(|e| DmError::Network(e.to_string()))?;

    // DEBUG: یک بار وضعیت رزوم را چاپ کن تا علت restart مشخص شود
    #[cfg(debug_assertions)]
    eprintln!(
        "DEBUG resume: status={:?} CR={:?} CL={:?} off={}",
        resp.status(),
        resp.headers().get(reqwest::header::CONTENT_RANGE),
        resp.content_length(),
        start_offset
    );

    Ok(resp)
}



async fn run_stream_to_file(mut file: File, resp: Response, total_size: Option<u64>, start_offset: u64) -> Result<()> {
    let expected = total_size.unwrap_or_else(|| {
        let len = resp.content_length().unwrap_or(0);
        start_offset + len
    });

    let pb = ProgressBar::new(expected);
    pb.set_position(start_offset);
    pb.set_style(
        ProgressStyle::with_template("{bar:40.cyan/blue} {bytes}/{total_bytes} ({bytes_per_sec}) ETA {eta}")
            .unwrap()
            .progress_chars("##-"),
    );

    let mut stream = resp.bytes_stream();
    let mut written = start_offset;

    while let Some(next) = stream.next().await {
        let chunk = next.map_err(|e| DmError::Network(e.to_string()))?;
        file.write_all(&chunk).await.map_err(DmError::Io)?;
        written += chunk.len() as u64;
        pb.set_position(written);
    }

    pb.finish_with_message("Done");
    iox::finalize_sync(&mut file).await?;
    Ok(())
}
