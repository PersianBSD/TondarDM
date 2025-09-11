//! Probe binary: send request and dump headers/meta (standalone tester)
//! Author: Ali Asadi | Team: Persian Developer Team | Email: persianbsd@gmail.com

use clap::Parser;
use tondar_dm::net::{inspect, request};

#[derive(Parser, Debug)]
#[command(name = "probe", about = "Simple header/meta probe")]
struct Args {
    /// URL to inspect
    url: String,
    /// Optional Referer
    #[arg(long)]
    referer: Option<String>,
    /// Optional Cookie line
    #[arg(long)]
    cookie: Option<String>,
    /// User-Agent override
    #[arg(long = "ua")]
    ua: Option<String>,
    /// Extra header(s): -H "Key: Value" (repeatable)
    #[arg(short = 'H', long = "header")]
    headers: Vec<String>,
    /// Force GET with Range: bytes=0-0 (instead of HEAD)
    #[arg(long = "force-range")]
    force_range: bool,
}

fn parse_extra_headers(v: &[String]) -> Vec<(String, String)> {
    v.iter()
        .filter_map(|line| {
            line.split_once(':')
                .map(|(k, rest)| (k.trim().to_string(), rest.trim().to_string()))
        })
        .collect()
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let opts = request::ClientOpts {
        referer: args.referer,
        cookie: args.cookie,
        ua: args.ua.or_else(|| Some("TondarProbe/0.1".into())),
        extra_headers: parse_extra_headers(&args.headers),
        max_redirects: 10,
        conn_timeout_secs: 20,
        req_timeout_secs: 300,
    };

    let client = match request::build_client(&opts) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Build client error: {e}");
            return;
        }
    };

    let mode = if args.force_range {
        inspect::ProbeMode::GetRange0
    } else {
        inspect::ProbeMode::Auto
    };

    match inspect::probe_url(&client, &args.url, mode).await {
        Ok(meta) => inspect::print_table(&meta),
        Err(e) => eprintln!("Probe error: {e}"),
    }
}
