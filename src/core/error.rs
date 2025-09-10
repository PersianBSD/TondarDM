//! خطاهای سراسری و Result

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DmError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("HTTP status: {0}")]
    HttpStatus(String),
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("Other: {0}")]
    Other(String),
}

/// Result سراسری دانلود منیجر
pub type Result<T> = std::result::Result<T, DmError>;
