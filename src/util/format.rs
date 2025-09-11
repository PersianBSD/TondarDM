//! Utility: human-readable size formatting (KB, MB, GB)
//! Author: Ali Asadi | Team: Persian Developer Team | Email: persianbsd@gmail.com
//!
//! - format_size(): format u64 bytes into smart units

/// format_size: converts bytes into KB / MB / GB smartly
/// - < 1 MB → show in KB
/// - < 1000 MB → show in MB
/// - ≥ 1000 MB → show in GB
pub fn format_size(bytes: u64) -> String {
    let kb = 1024.0;
    let mb = kb * 1024.0;
    let gb = mb * 1024.0;

    if bytes < mb as u64 {
        // show in KB
        format!("{:.2} KB", bytes as f64 / kb)
    } else if bytes < (1000.0 * mb) as u64 {
        // show in MB
        format!("{:.2} MB", bytes as f64 / mb)
    } else {
        // show in GB
        format!("{:.2} GB", bytes as f64 / gb)
    }
}
