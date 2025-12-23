use std::process::Command;

// pub fn unlock_path(path: &str, recursive: bool) {
//     let mut cmd = Command::new("icacls");

//     cmd.arg(path)
//         .arg("/remove:d")
//         .arg("Everyone")
//         .arg("/inheritance:e");

//     if recursive {
//         cmd.arg("/T");
//     }
//     // let out = cmd.output();
//     match cmd.output() {
//         Ok(_) => log::warn!("🔓 ACL removed (unlocked): {}", path),
//         Err(e) => log::error!("❌ Failed to unlock {}: {}", path, e),
//     }
//     //  log::error!("🔓 UNLOCK CMD OUTPUT: {:?}", out);
// }

pub fn unlock_path(path: &str, _recursive: bool) {
     let output = std::process::Command::new("icacls")
        .arg(path)
        .arg("/inheritance:e")
        .arg("/remove:d")
        .arg("*S-1-1-0")
        .output();

    match output {
        Ok(_) => log::warn!("🔓 RUNTIME LOCK REMOVED: {}", path),
        Err(e) => log::error!("❌ UNLOCK FAILED on {}: {}", path, e),
    }
}

