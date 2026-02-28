use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use chrono::Local;
use crate::core::branch;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArkConfig {
    pub version: String,
    pub created_at: String,
    pub project_name: String,
}

pub fn init(project_name: &str) -> Result<(), String> {
    let ark_dir = Path::new(".ark");

    // Prevent re-initialization
    if ark_dir.exists() {
        return Err("Ark repository already exists in this directory.".to_string());
    }

    fs::create_dir(ark_dir)
        .map_err(|e| format!("Failed to create .ark directory: {}", e))?;

    fs::create_dir(".ark/commits")
        .map_err(|e| format!("Failed to create commits directory: {}", e))?;

    fs::create_dir(".ark/snapshots")
        .map_err(|e| format!("Failed to create snapshots directory: {}", e))?;

    fs::create_dir(".ark/branches")
        .map_err(|e| format!("Failed to create branches directory: {}", e))?;

    // Create default main branch
    branch::create_branch("main")?;

    // Set HEAD to main
    branch::set_current_branch("main")?;

    let config = ArkConfig {
        version: "0.1.0".to_string(),
        created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        project_name: project_name.to_string(),
    };

    let config_json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(".ark/config.json", config_json)
        .map_err(|e| format!("Failed to write config: {}", e))?;

    Ok(())
}

pub fn is_initialized() -> bool {
    Path::new(".ark").exists() && Path::new(".ark/config.json").exists()
}

pub fn load_config() -> Result<ArkConfig, String> {
    let content = fs::read_to_string(".ark/config.json")
        .map_err(|_| "Failed to read config. Is this an Ark repository?".to_string())?;

    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config: {}", e))
}
