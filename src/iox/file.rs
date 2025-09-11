//! I/O helpers for single download (open/resume, preallocate, finalize)
//! Author: Ali Asadi | Team: Persian Developer Team | Email: persianbsd@gmail.com
//!
//! - open_for_resume(): open/create file and return current size
//! - preallocate_if_needed(): optional preallocate to total size
//! - finalize_sync(): fsync to ensure durability

//! I/O helpers (open/resume, preallocate, finalize)

use crate::engine::prelude::*;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use std::io::SeekFrom;

pub async fn open_for_resume(path: &str) -> Result<(File, u64)> {
    let f = OpenOptions::new()
        .create(true).write(true).read(true)
        .open(path).await
        .map_err(DmError::Io)?;

    let meta = f.metadata().await.map_err(DmError::Io)?;
    let len = meta.len();

    // مهم: اینجا دیگر seek به len نمی‌کنیم. موقع نوشتن، صراحتاً seek می‌کنیم.
    Ok((f, len))
}


pub async fn preallocate_if_needed(file: &File, total_size: Option<u64>, existing_len: u64, preallocate: bool) -> Result<()> {
    if preallocate {
        if let Some(sz) = total_size {
            if sz > existing_len {
                file.set_len(sz).await.map_err(DmError::Io)?;
            }
        }
    }
    Ok(())
}

pub async fn finalize_sync(file: &mut File) -> Result<()> {
    file.flush().await.map_err(DmError::Io)?;
    file.sync_all().await.map_err(DmError::Io)?;
    Ok(())
}
