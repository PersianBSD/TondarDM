//! CLI utilities: parse args, print metadata, ask confirmation
//! Author: Ali Asadi | Team: Persian Developer Team | Email: persianbsd@gmail.com

use clap::{Parser, ArgAction};
use std::io::{self, Write};
use crate:: util::format::format_size; // ← اضافه


#[derive(Parser, Debug)]
#[command(name = "TondarDM", version, about = "Phase 0: metadata probe")]
pub struct Args {
    /// Download link (HTTP/HTTPS)
    pub url: String,
    /// Optional Referer header
    #[arg(long)]
    pub referer: Option<String>,
    /// Extra header(s): -H "Key: Value" (repeatable)
    #[arg(short = 'H', long = "header", action = ArgAction::Append)]
    pub headers: Vec<String>,
    /// Cookie header (single line)
    #[arg(long)]
    pub cookie: Option<String>,
    /// Override User-Agent
    #[arg(long = "ua")]
    pub ua: Option<String>,
}

pub fn parse_args() -> Args {
    Args::parse()
}

pub fn print_meta(url: &str, name: &str, size: Option<u64>, ranges: bool) {
    println!("URL      : {url}");
    println!("Filename : {name}");
    match size {
        Some(n) => println!("Size     : {}", format_size(n)),
        None    => println!("Size     : (unknown)"),
    }
    let flag = if ranges { "supported" } else { "not supported/unknown" };
    println!("Ranges   : {flag}");
}

pub fn confirm(question: &str) -> bool {
    print!("{question} [y/N]: ");
    let _ = io::stdout().flush();
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_ok() {
        matches!(line.trim().to_ascii_lowercase().as_str(), "y" | "yes")
    } else {
        false
    }
}
