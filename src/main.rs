// module declarations (OK at top)
mod config;
mod sync;
mod rules;
mod monitor;
mod agent;
mod features;
mod transport;
mod models;
mod enforcer;
mod platform;
mod detection;
use crate::platform::windows::drives::get_all_drives;
use crate::rules::action::RuleAction;
use crate::rules::operation::Operation;
use crate::enforcer::proactive::lock_path;
use crate::rules::rule::Rule;
use std::sync::{ Arc, RwLock };
use log::info;
use crate::agent::state::AgentState;
use crate::features::feature_flags::FeatureFlags;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Enterprise Guard Agent starting...");
    let config = config::config::Config::load()?;
    // ✅ Runtime initialization MUST be inside main
    let agent_state = AgentState {
        features: Arc::new(RwLock::new(FeatureFlags::default())),
        rules: Arc::new(RwLock::new(Vec::new())),
        runtime_locks: Arc::new(RwLock::new(Vec::new())),
    };
    {
    let agent_state = agent_state.clone();
        ctrlc::set_handler(move || {
            log::warn!("🔻 Agent stopping — removing runtime locks");

            let locks = agent_state.runtime_locks.read().unwrap();
            log::error!("🧠 Runtime locks: {:?}", *locks);
            for path in locks.iter() {
                // unlock everything this agent locked at runtime
                crate::enforcer::unlock::unlock_path(path, true);
            }

            std::process::exit(0);
        })?;
    }
    // let rules = agent_state.get_rules();
    // for rule in rules {
    //     if rule.enabled && rule.action == RuleAction::Block {
    //         lock_path(&rule.path, rule.recursive);
    //     }
    // }

    // TEMP: add test rule if you want
    // agent_state.update_rules(...);

    agent_state.update_rules(
        vec![Rule {
            rule_id: "block-modify".to_string(),
            path: "D:\\Agentstarts\\Agentstarts.txt".to_string(),
            recursive: true,
            operations: vec![],
            action: RuleAction::Block,
            enabled: true,
        }]
    );

    let drives = get_all_drives();
    let filtered_drives: Vec<String> = drives
        .into_iter()
        .filter(|d| !d.eq_ignore_ascii_case(""))
        .collect();

    log::info!("Detected drives (excluding C:): {:?}", filtered_drives);
    log::info!("Detected drives: {:?}", filtered_drives);

    // Start file monitor
    monitor::start_watching(
        vec!["D:\\".to_string(), "E:\\".to_string(), "F:\\".to_string()],
        // filtered_drives,
        agent_state.clone()
    );
    log::info!("Watching path");
    tokio::spawn(
        sync::rule_sync::start_rule_polling(agent_state.clone(), config.server_url.clone())
    );
     loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
    // Keep alive
    // tokio::signal::ctrl_c().await?;
    Ok(())
}
