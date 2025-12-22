use std::fs;
use std::path::{Path, PathBuf};
use chrono::Utc;

pub fn backup_file(path: &str) -> Option<PathBuf> {
    let src = Path::new(path);
    if !src.exists() || src.is_dir() {
        return None;
    }

    let timestamp = Utc::now().timestamp();
    let file_name = src.file_name()?.to_string_lossy();

    let backup_dir = Path::new("C:\\ProgramData\\EnterpriseGuard\\backup");
    let _ = fs::create_dir_all(backup_dir);

    let backup_path = backup_dir.join(format!(
        "{}_{}",
        timestamp,
        file_name
    ));

    match fs::copy(src, &backup_path) {
        Ok(_) => Some(backup_path),
        Err(_) => None,
    }
}
