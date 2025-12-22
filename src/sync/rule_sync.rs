use crate::agent::state::AgentState;
use crate::rules::rule::Rule;
use reqwest::Client;
use tokio::time::{sleep, Duration};

pub async fn start_rule_polling(
    agent_state: AgentState,
    server_url: String,
) {
    let client = Client::new();

    loop {
        match client
            .get(format!("{}/agent/rules", server_url))
            .send()
            .await
        {
            Ok(resp) => {
                if let Ok(rules) = resp.json::<Vec<Rule>>().await {
                    agent_state.update_rules(rules);
                    log::info!("Rules updated from server");
                }
            }
            Err(e) => {
                log::error!("Rule sync failed: {}", e);
            }
        }

        sleep(Duration::from_secs(5)).await;
    }
}
