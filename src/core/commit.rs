use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::Local;
use crate::core::branch;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub branch: String,
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
    let current_branch = branch::get_current_branch();

    let commit = Commit {
        id: id.clone(),
        message: message.to_string(),
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        branch: current_branch.clone(),
        files_snapshot: snapshot.clone(),
    };

    // Save commit metadata
    let commit_json = serde_json::to_string_pretty(&commit)
        .map_err(|e| format!("Failed to serialize commit: {}", e))?;

    fs::write(format!(".ark/commits/{}.json", id), commit_json)
        .map_err(|e| format!("Failed to write commit: {}", e))?;

    // Update latest snapshot for current branch
    let snapshot_json = serde_json::to_string_pretty(&snapshot)
        .map_err(|e| format!("Failed to serialize snapshot: {}", e))?;

    fs::write(
        format!(".ark/snapshots/{}.json", current_branch),
        snapshot_json,
    ).map_err(|e| format!("Failed to write snapshot: {}", e))?;

    // Append commit id to current branch history
    let mut branch_data = branch::load_branch(&current_branch)
        .map_err(|e| format!("Failed to load branch: {}", e))?;

    branch_data.commit_ids.push(id.clone());
    branch::save_branch(&branch_data)?;

    Ok(id)
}

pub fn load_history() -> Vec<String> {
    let current_branch = branch::get_current_branch();

    branch::load_branch(&current_branch)
        .map(|b| b.commit_ids)
        .unwrap_or_default()
}

pub fn load_commit(id: &str) -> Result<Commit, String> {
    let content = fs::read_to_string(format!(".ark/commits/{}.json", id))
        .map_err(|_| format!("Commit '{}' not found.", id))?;

    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse commit: {}", e))
}
