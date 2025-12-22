#[derive(Debug)]
pub struct Config {
    pub server_url: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            server_url: "http://localhost:8080".to_string(),
        })
    }
}
