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

use crate::platform::windows::drives::get_all_drives;
use std::sync::{Arc, RwLock};
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
    };

    // TEMP: add test rule if you want
    // agent_state.update_rules(...);
    
let drives = get_all_drives();
    let filtered_drives: Vec<String> = drives
    .into_iter()
    .filter(|d| !d.eq_ignore_ascii_case("C:\\"))
    .collect();

log::info!("Detected drives (excluding C:): {:?}", filtered_drives);
log::info!("Detected drives: {:?}", filtered_drives);

    // Start file monitor
    monitor::start_watching(
        // vec!["D:\\".to_string() , "E:\\".to_string() , "F:\\".to_string()] ,
        filtered_drives,
        agent_state.clone(),
    );
    log::info!("Watching path");
    tokio::spawn(sync::rule_sync::start_rule_polling(
    agent_state.clone(),
        config.server_url.clone(),
    ));


    // Keep alive
    tokio::signal::ctrl_c().await?;
    Ok(())
}
