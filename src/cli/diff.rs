use colored::Colorize;
use crate::core::{repo, tracker, commit};

pub fn run(commit_id: Option<&str>) {
    if !repo::is_initialized() {
        eprintln!("{}", "Error: Not an Ark repository. Run 'ark start' first.".red().bold());
        return;
    }

    match commit_id {
        None => show_current_diff(),
        Some(id) => show_commit_diff(id),
    }
}

// Show diff between current state and last commit
fn show_current_diff() {
    let changes = tracker::scan_changes();

    if changes.is_empty() {
        println!("{}", "No changes detected.".green());
        return;
    }

    println!("{}", "Current Changes:".bold().underline());
    println!();

    for f in &changes {
        match f.status {
            tracker::Status::New => {
                println!("{} {}", "+".green().bold(), f.path.green());

                // Show file content preview
                if let Ok(content) = std::fs::read_to_string(&f.path) {
                    for line in content.lines().take(10) {
                        println!("  {} {}", "+".green(), line.green());
                    }
                    if content.lines().count() > 10 {
                        println!("  {}", "... (more lines)".dimmed());
                    }
                }
                println!();
            }
            tracker::Status::Modified => {
                println!("{} {}", "~".yellow().bold(), f.path.yellow());
                println!();
            }
            tracker::Status::Deleted => {
                println!("{} {}", "-".red().bold(), f.path.red());
                println!();
            }
            tracker::Status::Unchanged => {}
        }
    }

    let new = changes.iter().filter(|f| f.status == tracker::Status::New).count();
    let modified = changes.iter().filter(|f| f.status == tracker::Status::Modified).count();
    let deleted = changes.iter().filter(|f| f.status == tracker::Status::Deleted).count();

    println!("{} {} new  {} modified  {} deleted",
        "Summary:".bold(),
        new.to_string().green(),
        modified.to_string().yellow(),
        deleted.to_string().red()
    );
}

// Show diff for a specific commit
fn show_commit_diff(id: &str) {
    match commit::load_commit(id) {
        Ok(c) => {
            println!("{} {}", "Commit:".bold(), c.id.cyan());
            println!("{} {}", "Message:".dimmed(), c.message);
            println!("{} {}", "Date:".dimmed(), c.timestamp);
            println!("{} {}", "Branch:".dimmed(), c.branch);
            println!();
            println!("{}", "Files in this commit:".bold().underline());
            println!();

            for (path, _) in &c.files_snapshot {
                println!("  {} {}", "●".cyan(), path);
            }

            println!();
            println!("{} {} files tracked", "Total:".dimmed(), c.files_snapshot.len().to_string().cyan());
        }
        Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
    }
}
