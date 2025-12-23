use std::sync::{Arc, RwLock};
use crate::agent::rule_applier::apply_rules;
use crate::enforcer::proactive::lock_path;
use crate::enforcer::unlock::unlock_path;
use crate::features::feature_flags::FeatureFlags;
use crate::rules::rule::Rule;

#[derive(Clone)]
pub struct AgentState {
    pub features: Arc<RwLock<FeatureFlags>>,
    pub rules: Arc<RwLock<Vec<Rule>>>,
    pub runtime_locks: Arc<RwLock<Vec<String>>>,
}

impl AgentState {

    // pub fn update_rules(&self, new_rules: Vec<Rule>) {
    //     let mut rules = self.rules.write().unwrap();
    //     *rules = new_rules;
    // }

    pub fn update_rules(&self, new_rules: Vec<Rule>) {
    let mut rules = self.rules.write().unwrap();
    let mut runtime_locks = self.runtime_locks.write().unwrap();
        let old_rules = rules.clone();
        *rules = new_rules.clone();

        // apply_rules(old_rules, new_rules);
          // 🔓 UNLOCK rules turned OFF
        for old in &old_rules {
            let still_enabled = new_rules
                .iter()
                .find(|r| r.rule_id == old.rule_id)
                .map(|r| r.enabled)
                .unwrap_or(false);

            if old.enabled && !still_enabled {
                unlock_path(&old.path, old.recursive);
                runtime_locks.retain(|p| p != &old.path);
            }
        }

        // 🔒 LOCK rules turned ON
        for rule in &new_rules {
            if rule.enabled && !runtime_locks.contains(&rule.path) {
                lock_path(&rule.path, rule.recursive);
                runtime_locks.push(rule.path.clone());
            }
        }
    }
    
    pub fn get_rules(&self) -> Vec<Rule> {
        let rules = self.rules.read().unwrap();
        rules.clone()
    }
}
