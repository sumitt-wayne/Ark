use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AiConfig {
    pub api_key: String,
    pub model: String,
}

impl Default for AiConfig {
    fn default() -> Self {
        AiConfig {
            api_key: String::new(),
            model: "llama-3.3-70b-versatile".to_string(),
        }
    }
}

pub fn save_config(config: &AiConfig) -> Result<(), String> {
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize AI config: {}", e))?;

    fs::write(".ark/ai_config.json", json)
        .map_err(|e| format!("Failed to write AI config: {}", e))
}

pub fn load_config() -> Result<AiConfig, String> {
    let content = fs::read_to_string(".ark/ai_config.json")
        .map_err(|_| "AI not configured. Run 'ark ai setup' first.".to_string())?;

    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse AI config: {}", e))
}

pub fn is_configured() -> bool {
    if let Ok(config) = load_config() {
        !config.api_key.is_empty()
    } else {
        false
    }
}
