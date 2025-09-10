//! Config: تعریف و ۳ تابع (default/load/save)

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub max_concurrent: usize,
    pub per_host_limit: usize,
    pub rate_limit_global: Option<String>,
    pub default_parts: usize,
    pub resume: bool,
    pub preallocate: bool,
    pub output_dir: String,
}

/// default_config: تنظیمات پیش‌فرض را برمی‌گرداند.
pub fn default_config() -> Config {
    Config {
        max_concurrent: 3,
        per_host_limit: 2,
        rate_limit_global: None,
        default_parts: 4,
        resume: true,
        preallocate: true,
        output_dir: String::from("./Downloads"),
    }
}

/// load_config: کانفیگ را از مسیر بدهٔ TOML می‌خواند (اگر نبود → پیش‌فرض).
pub fn load_config(path: &str) -> Config {
    if Path::new(path).exists() {
        if let Ok(txt) = fs::read_to_string(path) {
            if let Ok(cfg) = toml::from_str::<Config>(&txt) { return cfg; }
        }
    }
    default_config()
}

/// save_config: کانفیگ فعلی را به TOML ذخیره می‌کند.
pub fn save_config(path: &str, cfg: &Config) -> std::io::Result<()> {
    let s = toml::to_string_pretty(cfg).expect("serialize config");
    fs::create_dir_all(Path::new(path).parent().unwrap_or(Path::new(".")))?;
    fs::write(path, s)
}
