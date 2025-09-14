//! State file i/o (save/load simple resume info).
//! Author: Ali Asadi | Team: Persian Developer Team | Email: persianbsd@gmail.com

use crate::engine::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct DlState {
    pub url: String,
    pub filename: String,
    pub total: Option<u64>,
    pub written: u64,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

fn state_path(filename: &str) -> String {
    format!("{filename}.state")
}

pub async fn save_state(s: &DlState) -> Result<()> {
    let path = state_path(&s.filename);
    let json = serde_json::to_vec_pretty(s).map_err(|e| DmError::Other(e.to_string()))?;
    let mut f = fs::File::create(&path).await.map_err(DmError::Io)?;
    f.write_all(&json).await.map_err(DmError::Io)?;
    f.flush().await.map_err(DmError::Io)?;
    Ok(())
}

pub async fn remove_state(filename: &str) -> Result<()> {
    let path = state_path(filename);
    match fs::remove_file(&path).await {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(DmError::Io(e)),
    }
}
