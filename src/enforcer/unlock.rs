use std::process::Command;

pub fn unlock_path(path: &str, recursive: bool) {
    let mut cmd = Command::new("icacls");

    cmd.arg(path)
        .arg("/remove:d")
        .arg("Everyone")
        .arg("/inheritance:e");

    if recursive {
        cmd.arg("/T");
    }
    let out = cmd.output();
    // match cmd.output() {
    //     Ok(_) => log::warn!("🔓 ACL removed (unlocked): {}", path),
    //     Err(e) => log::error!("❌ Failed to unlock {}: {}", path, e),
    // }
     log::error!("🔓 UNLOCK CMD OUTPUT: {:?}", out);
}
