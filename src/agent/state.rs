use std::sync::{Arc, RwLock};
use crate::features::feature_flags::FeatureFlags;
use crate::rules::rule::Rule;

#[derive(Clone)]
pub struct AgentState {
    pub features: Arc<RwLock<FeatureFlags>>,
    pub rules: Arc<RwLock<Vec<Rule>>>,
}

impl AgentState {

    pub fn update_rules(&self, new_rules: Vec<Rule>) {
        let mut rules = self.rules.write().unwrap();
        *rules = new_rules;
    }

    pub fn get_rules(&self) -> Vec<Rule> {
        let rules = self.rules.read().unwrap();
        rules.clone()
    }
}
