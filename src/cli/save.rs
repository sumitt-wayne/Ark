use colored::Colorize;
use crate::core::{repo, tracker, commit};

pub fn run(message: Option<String>) {
    if !repo::is_initialized() {
        eprintln!("{}", "Error: Not an Ark repository. Run 'ark start' first.".red().bold());
        return;
    }

    let changes = tracker::scan_changes();

    if changes.is_empty() {
        println!("{}", "Nothing to save. No changes detected.".yellow());
        return;
    }

    // Build current snapshot from scanned files
    let snapshot = tracker::build_snapshot();

    // Use provided message or generate one
    let msg = match message {
        Some(m) if !m.trim().is_empty() => m,
        _ => generate_message(&changes),
    };

    match commit::save_commit(&msg, snapshot) {
        Ok(id) => {
            println!("{}", "Changes saved successfully!".green().bold());
            println!("  {} {}", "id:".dimmed(), id.cyan());
            println!("  {} {}", "message:".dimmed(), msg.cyan());
            println!("  {} {} new  {} modified  {} deleted",
                "saved:".dimmed(),
                changes.iter().filter(|f| f.status == tracker::Status::New).count().to_string().green(),
                changes.iter().filter(|f| f.status == tracker::Status::Modified).count().to_string().yellow(),
                changes.iter().filter(|f| f.status == tracker::Status::Deleted).count().to_string().red(),
            );
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
        }
    }
}

fn generate_message(changes: &[tracker::FileStatus]) -> String {
    let new = changes.iter().filter(|f| f.status == tracker::Status::New).count();
    let modified = changes.iter().filter(|f| f.status == tracker::Status::Modified).count();
    let deleted = changes.iter().filter(|f| f.status == tracker::Status::Deleted).count();

    let mut parts = Vec::new();
    if new > 0 { parts.push(format!("{} new", new)); }
    if modified > 0 { parts.push(format!("{} modified", modified)); }
    if deleted > 0 { parts.push(format!("{} deleted", deleted)); }

    format!("Auto: {}", parts.join(", "))
}
