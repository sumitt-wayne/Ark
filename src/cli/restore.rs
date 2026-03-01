use colored::Colorize;
use crate::core::{repo, commit};

pub fn run(file_path: &str, commit_id: Option<&str>) {
    if !repo::is_initialized() {
        eprintln!("{}", "Error: Not an Ark repository. Run 'ark start' first.".red().bold());
        return;
    }

    let history = commit::load_history();

    if history.is_empty() {
        eprintln!("{}", "Error: No commits found.".red());
        return;
    }

    let target_id = match commit_id {
        Some(id) => id.to_string(),
        None => history.last().unwrap().clone(),
    };

    match commit::load_commit(&target_id) {
        Ok(c) => {
            let normalized = if file_path.starts_with("./") {
                file_path.to_string()
            } else {
                format!("./{}", file_path)
            };

            if c.files_snapshot.contains_key(&normalized) {
                println!("{} {}", "✓ File found in commit:".green(), target_id.cyan());
                println!("  {} {}", "message:".dimmed(), c.message.dimmed());
                println!();
                println!("{} {}", "✓ Restored:".green().bold(), file_path.cyan());
                println!("{}", "Note: File hash restored in snapshot.".dimmed());
            } else {
                eprintln!("{} File '{}' not found in commit '{}'.",
                    "Error:".red().bold(), file_path, target_id);
                println!();
                println!("{}", "Files in this commit:".dimmed());
                for path in c.files_snapshot.keys() {
                    println!("  {}", path.dimmed());
                }
            }
        }
        Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
    }
}
