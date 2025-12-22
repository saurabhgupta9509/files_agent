use std::fs;
use std::path::Path;
use crate::enforcer::backup::backup_file;
use crate::enforcer::damage_tracker::record_damage;
use crate::rules::operation::Operation;

pub fn enforce_soft_block(
    path: &str,
    operation: &Operation,
) {

     if record_damage(path) {
         log::error!("[ESCALATION] Mass damage detected on {}", path);
            // Phase 4.2: folder lockdown / process kill
        }

    match operation {

        
        Operation::Delete => {
            log::error!("[DELETE BLOCKED - post fact] {}", path);
            // restore only if backup exists (future improvement)
        }

        Operation::Modify => {
            // Make file read-only
            if let Ok(metadata) = fs::metadata(path) {
                let mut perms = metadata.permissions();
                perms.set_readonly(true);
                let _ = fs::set_permissions(path, perms);
            }
        }

        Operation::Rename | Operation::Move => {
            if let Some(backup) = backup_file(path) {
                log::warn!("[RENAME BACKUP] {:?}", backup);
            }
        }

       
        _ => {}
    }
}
