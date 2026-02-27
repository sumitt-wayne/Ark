use colored::Colorize;
use crate::core::{repo, commit};
use std::fs;

pub fn run() {
    if !repo::is_initialized() {
        eprintln!("{}", "Error: Not an Ark repository. Run 'ark start' first.".red().bold());
        return;
    }

    let mut history = commit::load_history();

    if history.is_empty() {
        println!("{}", "Nothing to undo. No saves found.".yellow());
        return;
    }

    if history.len() == 1 {
        println!("{}", "Cannot undo. This is the first save.".yellow());
        return;
    }

    // Remove latest commit from history
    let latest_id = history.pop().unwrap();

    // Load the previous commit
    let previous_id = history.last().unwrap().clone();

    match commit::load_commit(&previous_id) {
        Ok(previous_commit) => {
            // Restore snapshot to previous state
            let snapshot_json = serde_json::to_string_pretty(&previous_commit.files_snapshot)
                .map_err(|e| format!("Failed to serialize snapshot: {}", e));

            match snapshot_json {
                Ok(json) => {
                    fs::write(".ark/snapshots/latest.json", json)
                        .expect("Failed to restore snapshot");

                    // Remove the undone commit file
                    let _ = fs::remove_file(format!(".ark/commits/{}.json", latest_id));

                    // Save updated history
                    let history_json = serde_json::to_string_pretty(&history)
                        .expect("Failed to serialize history");

                    fs::write(".ark/commits/history.json", history_json)
                        .expect("Failed to write history");

                    println!("{}", "Undo successful!".green().bold());
                    println!("  {} {}", "removed:".dimmed(), latest_id.red());
                    println!("  {} {}", "restored to:".dimmed(), previous_id.cyan());
                    println!("  {} {}", "message:".dimmed(), previous_commit.message);
                    println!("  {} {}", "saved at:".dimmed(), previous_commit.timestamp);
                }
                Err(e) => {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                }
            }
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
        }
    }
}
