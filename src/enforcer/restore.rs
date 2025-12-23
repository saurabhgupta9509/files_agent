use std::fs;
use std::path::Path;

pub fn restore_file(backup_path: &str, original_path: &str) {
    let _ = fs::copy(backup_path, original_path);
    log::warn!("♻️ Restored deleted file {}", original_path);
}
