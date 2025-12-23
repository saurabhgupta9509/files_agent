use std::fs;
use std::path::{Path, PathBuf};
use chrono::Utc;

pub fn backup_file(path: &str) -> Option<PathBuf> {
    let src = Path::new(path);
    if !src.exists() || src.is_dir() {
        return None;
    }

    let backup_dir = "C:\\ProgramData\\EnterpriseGuard\\backup";
    let _ = fs::create_dir_all(backup_dir);

    let filename = src.file_name()?.to_string_lossy();
    let backup_path = format!(
        "{}\\{}_{}",
        backup_dir,
        Utc::now().timestamp(),
        filename
    );

    fs::copy(src, &backup_path).ok()?;
    Some(PathBuf::from(backup_path))
}