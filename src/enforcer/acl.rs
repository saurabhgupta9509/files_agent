use std::process::Command;

pub fn deny_all_access(path: &str) {
    let output = Command::new("icacls")
        .arg(path)
        .arg("/deny")
        .arg("Everyone:(R,W,X)")
        .output();

    match output {
        Ok(_) => log::error!("🔒 Access denied on {}", path),
        Err(e) => log::error!("ACL failed on {}: {}", path, e),
    }
}
