pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

pub struct Alert {
    pub agent_id: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: String,
}
