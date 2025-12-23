use std::process::Command;

use crate::enforcer::{acl::deny_all_access, exclusions::is_excluded, folder_lock::lock_folder, path_type::{RuleTarget, detect_rule_target}};

// pub fn lock_path(path: &str, recursive: bool) {

//      if is_excluded(path) {
//         log::warn!("⚠️ Skipping excluded path: {}", path);
//             return;
//         }

//     let mut cmd = Command::new("icacls");

//     cmd.arg(path)
//         .arg("/deny")
//         .arg("Everyone:(R,W,D,X)");

//     if recursive {
//         cmd.arg("/T");
//     }

//     match cmd.output() {
//         Ok(_) => log::error!("🔒 PROACTIVE LOCK applied on {}", path),
//         Err(e) => log::error!("❌ Failed to apply ACL on {}: {}", path, e),
//     }
// }

pub fn lock_path(path: &str, recursive: bool) {
    if is_excluded(path) {
        log::warn!("⚠️ Skipping excluded path: {}", path);
        return;
    }

     match detect_rule_target(path) {
        Some(RuleTarget::File) => {
            log::info!("🔒 Locking FILE (rule intent): {}", path);
            deny_all_access(path);
        }

        Some(RuleTarget::Directory) => {
            log::info!("🔒 Locking DIRECTORY (rule intent): {}", path);
            lock_folder(path);
        }

        None => {
            log::error!("❌ Could not determine rule target: {}", path);
        }
    }
}

fn apply_acl(path: &str, recursive: bool) {
    let mut cmd = Command::new("icacls");

    cmd.arg(path).arg("/deny").arg("Everyone:(R,W,D,X)");

    if recursive {
        cmd.arg("/T");
    }

    match cmd.output() {
        Ok(_) => log::info!("🔒 ACL applied on {}", path),
        Err(e) => log::error!("❌ ACL failed on {}: {}", path, e),
    }
}
