use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Branch {
    pub name: String,
    pub commit_ids: Vec<String>,
}

pub fn get_current_branch() -> String {
    fs::read_to_string(".ark/HEAD")
        .unwrap_or_else(|_| "main".to_string())
        .trim()
        .to_string()
}

pub fn set_current_branch(name: &str) -> Result<(), String> {
    fs::write(".ark/HEAD", name)
        .map_err(|e| format!("Failed to update HEAD: {}", e))
}

pub fn branch_exists(name: &str) -> bool {
    Path::new(&format!(".ark/branches/{}.json", name)).exists()
}

pub fn create_branch(name: &str) -> Result<(), String> {
    fs::create_dir_all(".ark/branches")
        .map_err(|e| format!("Failed to create branches directory: {}", e))?;

    if branch_exists(name) {
        return Err(format!("Branch '{}' already exists.", name));
    }

    // New branch inherits current branch commits
    let current = get_current_branch();
    let current_commits = load_branch(&current)
        .map(|b| b.commit_ids)
        .unwrap_or_default();

    let branch = Branch {
        name: name.to_string(),
        commit_ids: current_commits,
    };

    save_branch(&branch)
}

pub fn save_branch(branch: &Branch) -> Result<(), String> {
    let json = serde_json::to_string_pretty(branch)
        .map_err(|e| format!("Failed to serialize branch: {}", e))?;

    fs::write(format!(".ark/branches/{}.json", branch.name), json)
        .map_err(|e| format!("Failed to write branch: {}", e))
}

pub fn load_branch(name: &str) -> Result<Branch, String> {
    let content = fs::read_to_string(format!(".ark/branches/{}.json", name))
        .map_err(|_| format!("Branch '{}' not found.", name))?;

    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse branch: {}", e))
}

pub fn list_branches() -> Vec<String> {
    let dir = Path::new(".ark/branches");

    if !dir.exists() {
        return vec![];
    }

    let mut branches = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                if let Some(stem) = path.file_stem() {
                    branches.push(stem.to_string_lossy().to_string());
                }
            }
        }
    }

    branches.sort();
    branches
}

pub fn delete_branch(name: &str) -> Result<(), String> {
    if name == "main" {
        return Err("Cannot delete 'main' branch.".to_string());
    }

    if get_current_branch() == name {
        return Err(format!("Cannot delete current branch '{}'. Switch to another branch first.", name));
    }

    if !branch_exists(name) {
        return Err(format!("Branch '{}' not found.", name));
    }

    fs::remove_file(format!(".ark/branches/{}.json", name))
        .map_err(|e| format!("Failed to delete branch: {}", e))
}
