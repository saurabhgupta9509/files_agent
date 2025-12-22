use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleAction {
    Allow,    // do nothing
    Monitor, // log only
    Block,    // enforce (Phase 4)
}
