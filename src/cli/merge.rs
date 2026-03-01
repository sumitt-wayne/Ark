use colored::Colorize;
use crate::core::{repo, branch, commit, tracker};

pub fn run(branch_name: &str) {
    if !repo::is_initialized() {
        eprintln!("{}", "Error: Not an Ark repository. Run 'ark start' first.".red().bold());
        return;
    }

    let current = branch::get_current_branch();

    // Cannot merge branch into itself
    if current == branch_name {
        eprintln!("{} Cannot merge branch into itself.", "Error:".red().bold());
        return;
    }

    // Check if branch exists
    if !branch::branch_exists(branch_name) {
        eprintln!("{} Branch '{}' not found.", "Error:".red().bold(), branch_name);
        return;
    }

    println!("{} {} {} {}",
        "Merging".dimmed(),
        branch_name.cyan().bold(),
        "into".dimmed(),
        current.green().bold()
    );
    println!();

    // Load source branch commits
    let source_branch = match branch::load_branch(branch_name) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            return;
        }
    };

    if source_branch.commit_ids.is_empty() {
        println!("{}", "Nothing to merge. Source branch has no commits.".yellow());
        return;
    }

    // Get latest snapshot from source branch
    let source_snapshot_path = format!(".ark/snapshots/{}.json", branch_name);
    let source_snapshot: std::collections::HashMap<String, String> = 
        std::fs::read_to_string(&source_snapshot_path)
            .ok()
            .and_then(|c| serde_json::from_str(&c).ok())
            .unwrap_or_default();

    if source_snapshot.is_empty() {
        println!("{}", "Nothing to merge. Source branch has no snapshot.".yellow());
        return;
    }

    // Check for conflicts with current changes
    let current_changes = tracker::scan_changes();
    if !current_changes.is_empty() {
        println!("{}", "⚠ Warning: You have uncommitted changes.".yellow().bold());
        println!("{}", "  Save your changes first: ark save".dimmed());
        println!();
    }

    // Create merge commit message
    let merge_message = format!("merge: {} into {}", branch_name, current);

    // Save merge commit with source snapshot
    match commit::save_commit(&merge_message, source_snapshot) {
        Ok(id) => {
            println!("{}", "✓ Merge successful!".green().bold());
            println!("  {} {}", "commit:".dimmed(), id.cyan());
            println!("  {} {}", "message:".dimmed(), merge_message.cyan());
            println!();
            println!("{} {}",
                "Tip:".dimmed(),
                "Run 'ark check' to see merged changes.".dimmed()
            );
        }
        Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
    }
}
