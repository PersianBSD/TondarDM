//! TondarDM — Phase 2 integrated with probe (final URL + If-Range)

mod ui { pub mod cli; }
mod http { pub mod client; }
mod download { pub mod single; }
mod iox { pub mod file; }
mod net { pub mod request; pub mod inspect; }
mod engine {pub mod error; pub mod config; pub mod consts; pub mod prelude; pub mod  types;}
mod util {pub mod format;}

use crate::engine::prelude::{Result, DmError};   // ← مهم
use crate::net::inspect::{self, ProbeMode};

#[tokio::main]
async fn main() -> Result<()> {
    let args = ui::cli::parse_args();
    let client = http::client::build_client(&args)?;

    let meta = inspect::probe_url(&client, &args.url, ProbeMode::Auto)
        .await
        .map_err(|e| DmError::Other(format!("probe failed: {e}")))?;

    ui::cli::print_meta(&meta.final_url, &meta.filename, meta.size, meta.accept_ranges);

    if !ui::cli::confirm("Start download now?") {
        println!("Aborted by user.");
        return Ok(());
    }

    println!("Starting single download...");
    download::single::download_single(
        &client,
        &meta.final_url,
        &meta.filename,
        meta.size,
        meta.accept_ranges,
        meta.etag.as_deref(),
        meta.last_modified.as_deref(),
    ).await?;

    println!("✅ Download completed: {}", meta.filename);
    Ok(())
}