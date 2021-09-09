use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct WebhookPayload {
    text: String,
}

impl WebhookPayload {
    pub fn create(message: &str) -> Result<String, String> {
        let p = WebhookPayload {
            text: message.to_string(),
        };
        serde_json::to_string(&p).map_err(|e| format!("failed to create webhook payload: {}", e))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigFile {
    pub webhook_urls: HashMap<String, String>,
}

impl ConfigFile {
    pub fn load() -> Result<ConfigFile, String> {
        let path = match dirs::home_dir().map(|p| p.join(".pesn.json")) {
            None => return Err("failed to resolve config file path".to_string()),
            Some(p) => p,
        };

        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("failed to read config file: {}", e))?;
        serde_json::from_str(&text).map_err(|e| format!("failed to parse config file: {}", e))
    }
}
#[derive(Debug)]
pub struct Configuration {
    pub webhook_url: String,
    pub pid: i32,
    pub interval_seconds: u64,
    pub memo: Option<String>
}
