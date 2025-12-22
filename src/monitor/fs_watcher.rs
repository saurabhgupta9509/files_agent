use notify::{Watcher, RecursiveMode, EventKind};
use std::sync::mpsc::channel;
use std::path::Path;
use crate::agent::state::AgentState;
use crate::monitor::event_mapper::{FsAction, map_event};
use crate::rules::matcher::match_rule;
use crate::rules::action::RuleAction;
use crate::rules::operation::{self, Operation};

pub fn start_watching(paths: Vec<String> , state: AgentState) {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(tx).expect("Failed to create watcher");

    // for path in paths {
    //     watcher.watch(Path::new(&path), RecursiveMode::Recursive).unwrap();
    // }

      for p in paths {
        let path = Path::new(&p);

        if !path.exists() {
            log::error!("❌ Watch path does not exist: {}", p);
            continue;
        }

        if !path.is_dir() && !path.is_file() {
            log::error!("❌ Path is not a file or directory: {}", p);
            continue;
        }

        match watcher.watch(path, RecursiveMode::Recursive) {
            Ok(_) => log::info!("✅ Watching path: {}", p),
            Err(e) => log::error!("❌ Failed to watch {}: {:?}", p, e),
        }
    }
    
    for res in rx {
        match res {
            Ok(event) => handle_event(event , state.clone()),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
pub fn handle_event(event: notify::Event, state: AgentState) {
    let features = state.features.read().unwrap();

    if !features.fs_monitoring {
        return;
    }
    drop(features);


    let action = map_event(&event);
    if action.is_none() {
        return;
    }

   let fs_action = action.unwrap();
    let operation: Operation = fs_action.into();

   let path = event.paths.get(0).map(|p| p.to_string_lossy().to_string());
        if path.is_none() {
            return;
        }

    let path = path.unwrap();



     handle_decision(path, operation, state);
    // send event

}

  pub async fn handle_fs_event(
    event_path: String,
    operation: Operation,
    is_directory: bool,
    agent_state: AgentState,
) {
    // 1️⃣ Feature flags check
    let features = agent_state.features.read().unwrap();
    if !features.fs_monitoring {
        return;
    }
    drop(features);

    // 2️⃣ Get rules snapshot
    let rules = agent_state.get_rules();

    // 3️⃣ Match rule
    let matched_rule = match_rule(&event_path, &operation, &rules);

    // 4️⃣ Decision
    match matched_rule {
        Some(rule) => {
            match rule.action {
                RuleAction::Allow => {
                    log::info!(
                        "[ALLOW] {} {:?}",
                        event_path,
                        operation
                    );
                }

                RuleAction::Monitor => {
                    log::warn!(
                        "[MONITOR] Rule={} {} {:?}",
                        rule.rule_id,
                        event_path,
                        operation
                    );
                }

                RuleAction::Block => {
                    log::error!(
                        "[BLOCK - not enforced yet] Rule={} {} {:?}",
                        rule.rule_id,
                        event_path,
                        operation
                    );
                    // 🚫 Actual blocking comes in Phase 4
                }
            }
        }

        None => {
            // No rule matched → default allow
            log::info!(
                "[DEFAULT ALLOW] {} {:?}",
                event_path,
                operation
            );
        }
    }
}

pub fn handle_decision(
    event_path: String,
    operation: Operation,
    agent_state: AgentState,
) {
    let features = agent_state.features.read().unwrap();
    if !features.fs_monitoring {
        return;
    }
    drop(features);

    let rules = agent_state.get_rules();
    let matched_rule = match_rule(&event_path, &operation, &rules);

    match matched_rule {
        Some(rule) => match rule.action {
            RuleAction::Allow => {
                log::info!("[ALLOW] {} {:?}", event_path, operation);
            }
            RuleAction::Monitor => {
                log::warn!(
                    "[MONITOR] Rule={} {} {:?}",
                    rule.rule_id, event_path, operation
                );
            }
            RuleAction::Block => {
                log::error!(
                    "[BLOCK - future] Rule={} {} {:?}",
                    rule.rule_id, event_path, operation
                );
            }
        },
        None => {
            log::info!("[DEFAULT ALLOW] {} {:?}", event_path, operation);
        }
    }
}
