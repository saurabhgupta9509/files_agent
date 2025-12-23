use std::process::Command;

pub fn lock_folder(path: &str) {
    let _ = Command::new("icacls")
        .arg(path)
        .arg("/deny")
        .arg("Everyone:(W)")
        .arg("/T")
        .output();

    log::error!("🚨 Folder locked: {}", path);
}
