use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::Local;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub files_snapshot: HashMap<String, String>,
}

pub fn generate_id() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    Local::now().timestamp_nanos_opt().unwrap_or(0).hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

pub fn save_commit(message: &str, snapshot: HashMap<String, String>) -> Result<String, String> {
    let id = generate_id();

    let commit = Commit {
        id: id.clone(),
        message: message.to_string(),
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        files_snapshot: snapshot.clone(),
    };

    // Save commit metadata
    let commit_json = serde_json::to_string_pretty(&commit)
        .map_err(|e| format!("Failed to serialize commit: {}", e))?;

    fs::write(format!(".ark/commits/{}.json", id), commit_json)
        .map_err(|e| format!("Failed to write commit: {}", e))?;

    // Update latest snapshot for future change detection
    let snapshot_json = serde_json::to_string_pretty(&snapshot)
        .map_err(|e| format!("Failed to serialize snapshot: {}", e))?;

    fs::write(".ark/snapshots/latest.json", snapshot_json)
        .map_err(|e| format!("Failed to write snapshot: {}", e))?;

    // Append commit id to history index
    let mut history = load_history();
    history.push(id.clone());

    let history_json = serde_json::to_string_pretty(&history)
        .map_err(|e| format!("Failed to serialize history: {}", e))?;

    fs::write(".ark/commits/history.json", history_json)
        .map_err(|e| format!("Failed to write history: {}", e))?;

    Ok(id)
}

pub fn load_history() -> Vec<String> {
    let content = fs::read_to_string(".ark/commits/history.json").unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

pub fn load_commit(id: &str) -> Result<Commit, String> {
    let content = fs::read_to_string(format!(".ark/commits/{}.json", id))
        .map_err(|_| format!("Commit '{}' not found.", id))?;

    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse commit: {}", e))
}
