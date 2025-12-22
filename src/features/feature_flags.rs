use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub fs_monitoring: bool,
    pub log_events: bool,

    pub detect_create: bool,
    pub detect_modify: bool,
    pub detect_delete: bool,
    pub detect_rename: bool,

    // future
    pub block_delete: bool,
}


impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            fs_monitoring: true,
            log_events: true,
            detect_create: true,
            detect_modify: true,
            detect_delete: true,
            detect_rename: true,
            block_delete: false,
        }
    }
}
