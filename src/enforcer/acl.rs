use std::process::Command;

pub fn deny_all_access(path: &str) {
    let output = std::process::Command::new("icacls")
        .arg(path)
        // Disable inheritance completely
        .arg("/inheritance:d")
        // Remove ALL grants
        .arg("/remove:g")
        .arg("*S-1-1-0") // Everyone SID
        // DENY EVERYTHING to Everyone (including Admins)
        .arg("/deny")
        .arg("*S-1-1-0:(F)")
        .output();

    match output {
        Ok(o) => log::error!(
            "🔒 HARD RUNTIME LOCK (NO ONE) on {} | status={:?}",
            path,
            o.status
        ),
        Err(e) => log::error!("❌ HARD LOCK FAILED on {}: {}", path, e),
    }
}
