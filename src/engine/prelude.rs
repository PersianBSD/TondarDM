//! Prelude: import سریع چیزهای متداول

//pub use crate::engine::types::{Meta, RangeReq, JobId, PathPlan};
//pub use crate::engine::error::{DmError, Result};
//pub use crate::engine::consts::*;

//pub type Result<T> = std::result::Result<T, DmError>;
// Keep the prelude minimal and stable.
pub use crate::engine::error::DmError;

/// Project-wide Result alias that uses DmError
pub type Result<T> = std::result::Result<T, DmError>;
