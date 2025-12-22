use serde::{Deserialize, Serialize};
use crate::rules::operation::Operation;
use crate::rules::action::RuleAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub rule_id: String,

    /// Path this rule applies to
    pub path: String,

    /// Apply to subfolders?
    pub recursive: bool,

    /// Operations this rule applies to
    pub operations: Vec<Operation>,

    /// What to do if rule matches
    pub action: RuleAction,

    /// Whether rule is enabled
    pub enabled: bool,
}
