use std::collections::HashMap;
use crate::rules::rule::Rule;
use crate::rules::action::RuleAction;
use crate::enforcer::proactive::lock_path;
use crate::enforcer::unlock::unlock_path;

pub fn apply_rules(
    old_rules: Vec<Rule>,
    new_rules: Vec<Rule>,
) {
    let old: HashMap<String, Rule> =
        old_rules.into_iter().map(|r| (r.rule_id.clone(), r)).collect();

    let new: HashMap<String, Rule> =
        new_rules.iter().map(|r| (r.rule_id.clone(), r.clone())).collect();

    // 🔒 Apply new or enabled rules
    for rule in new.values() {
        let was_enabled = old.get(&rule.rule_id).map(|r| r.enabled).unwrap_or(false);

        if rule.enabled && !was_enabled && rule.action == RuleAction::Block {
            lock_path(&rule.path, rule.recursive);
        }
    }

    // 🔓 Remove disabled rules
    for rule in old.values() {
        let is_enabled = new.get(&rule.rule_id).map(|r| r.enabled).unwrap_or(false);

        if rule.enabled && !is_enabled && rule.action == RuleAction::Block {
            unlock_path(&rule.path, rule.recursive);
        }
    }
}
