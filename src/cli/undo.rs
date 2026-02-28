use colored::Colorize;
use crate::core::{repo, commit, branch};
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

    let latest_id = history.pop().unwrap();
    let previous_id = history.last().unwrap().clone();
    let current_branch = branch::get_current_branch();

    match commit::load_commit(&previous_id) {
        Ok(previous_commit) => {
            let snapshot_json = serde_json::to_string_pretty(&previous_commit.files_snapshot)
                .map_err(|e| format!("Failed to serialize snapshot: {}", e));

            match snapshot_json {
                Ok(json) => {
                    // Restore branch snapshot
                    fs::write(
                        format!(".ark/snapshots/{}.json", current_branch),
                        json,
                    ).expect("Failed to restore snapshot");

                    // Remove undone commit file
                    let _ = fs::remove_file(format!(".ark/commits/{}.json", latest_id));

                    // Update branch commit history
                    let mut branch_data = branch::load_branch(&current_branch)
                        .expect("Failed to load branch");

                    branch_data.commit_ids = history;
                    branch::save_branch(&branch_data)
                        .expect("Failed to save branch");

                    println!("{}", "Undo successful!".green().bold());
                    println!("  {} {}", "removed:".dimmed(), latest_id.red());
                    println!("  {} {}", "restored to:".dimmed(), previous_id.cyan());
                    println!("  {} {}", "message:".dimmed(), previous_commit.message);
                    println!("  {} {}", "saved at:".dimmed(), previous_commit.timestamp);
                }
                Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
            }
        }
        Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
    }
}
