use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileStatus {
    pub path: String,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    New,
    Modified,
    Deleted,
    Unchanged,
}

pub fn load_snapshot() -> HashMap<String, String> {
    let snapshot_path = ".ark/snapshots/latest.json";

    if !Path::new(snapshot_path).exists() {
        return HashMap::new();
    }

    let content = fs::read_to_string(snapshot_path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

/// Build a fresh snapshot of current directory state
pub fn build_snapshot() -> HashMap<String, String> {
    let mut files = HashMap::new();
    scan_dir(Path::new("."), &mut files);
    files
}

pub fn scan_changes() -> Vec<FileStatus> {
    let snapshot = load_snapshot();
    let current_files = build_snapshot();
    let mut results = Vec::new();

    for (path, hash) in &current_files {
        match snapshot.get(path) {
            None => results.push(FileStatus {
                path: path.clone(),
                status: Status::New,
            }),
            Some(old_hash) => {
                if old_hash != hash {
                    results.push(FileStatus {
                        path: path.clone(),
                        status: Status::Modified,
                    });
                }
            }
        }
    }

    for path in snapshot.keys() {
        if !current_files.contains_key(path) {
            results.push(FileStatus {
                path: path.clone(),
                status: Status::Deleted,
            });
        }
    }

    results.sort_by(|a, b| a.path.cmp(&b.path));
    results
}

fn scan_dir(dir: &Path, files: &mut HashMap<String, String>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let path_str = path.to_string_lossy().to_string();

        if should_ignore(&path_str) {
            continue;
        }

        if path.is_dir() {
            scan_dir(&path, files);
        } else if path.is_file() {
            let hash = hash_file(&path);
            files.insert(path_str, hash);
        }
    }
}

fn should_ignore(path: &str) -> bool {
    let ignore_list = [".ark", "target", ".git", ".env"];
    ignore_list.iter().any(|i| path.contains(i))
}

fn hash_file(path: &Path) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let content = fs::read(path).unwrap_or_default();
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish().to_string()
}
