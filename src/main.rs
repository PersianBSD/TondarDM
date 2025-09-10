//! TondarDM — Phase 0: Metadata Probe (with robust fallback & headers)
//! Author: Ali Asadi | Team: Persian Developer Team | Email: persianbsd@gmail.com

mod ui { pub mod cli; }
mod http { pub mod client; }
mod probe { pub mod meta; }
mod core;

use crate::core::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let args = ui::cli::parse_args();
    let client = http::client::build_client(&args)?;

    // Try HEAD first, then fallback to GET 0-0
    let resp = match http::client::send_head(&client, &args.url).await {
        Ok(r) if r.status().is_success() => r,
        Ok(r) => {
            eprintln!("HEAD returned status: {} → falling back to GET range=0-0", r.status());
            http::client::get_range0(&client, &args.url).await?
        }
        Err(e) => {
            eprintln!("HEAD request failed: {e} → falling back to GET range=0-0");
            http::client::get_range0(&client, &args.url).await?
        }
    };

    // Extract metadata from whichever response we have
    let headers = resp.headers().clone();
    let filename = probe::meta::infer_filename(&args.url, &headers);
    let size     = probe::meta::parse_size(&headers);
    let ranges   = probe::meta::supports_range(&headers);

    ui::cli::print_meta(&args.url, &filename, size, ranges);

    if !ui::cli::confirm("Start download now? (implemented in next phase)") {
        println!("Aborted by user.");
        return Ok(());
    }

    println!("🎯 Phase 1: single download + resume + progress is next.");
    Ok(())
}
