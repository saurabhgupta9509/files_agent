use std::process::Command;

use crate::enforcer::exclusions::is_excluded;

pub fn lock_path(path: &str, recursive: bool) {

     if is_excluded(path) {
        log::warn!("⚠️ Skipping excluded path: {}", path);
            return;
        }

    let mut cmd = Command::new("icacls");

    cmd.arg(path)
        .arg("/deny")
        .arg("Everyone:(R,W,D,X)");

    if recursive {
        cmd.arg("/T");
    }

    match cmd.output() {
        Ok(_) => log::error!("🔒 PROACTIVE LOCK applied on {}", path),
        Err(e) => log::error!("❌ Failed to apply ACL on {}: {}", path, e),
    }
}

