use colored::Colorize;
use crate::core::{repo, tracker, branch};
use crate::core::tracker::Status;

pub fn run() {
    if !repo::is_initialized() {
        eprintln!("{}", "Error: Not an Ark repository. Run 'ark start' first.".red().bold());
        return;
    }

    let current_branch = branch::get_current_branch();
    println!("{} {}", "Branch:".dimmed(), current_branch.cyan().bold());
    println!();

    let changes = tracker::scan_changes();

    if changes.is_empty() {
        println!("{}", "Nothing to report. Everything is up to date.".green());
        return;
    }

    let new: Vec<_> = changes.iter().filter(|f| f.status == Status::New).collect();
    let modified: Vec<_> = changes.iter().filter(|f| f.status == Status::Modified).collect();
    let deleted: Vec<_> = changes.iter().filter(|f| f.status == Status::Deleted).collect();

    println!("{}", "Changes in your project:".bold());
    println!();

    if !new.is_empty() {
        println!("{}", "  New files:".green().bold());
        for f in &new {
            println!("    {} {}", "+".green(), f.path.green());
        }
    }

    if !modified.is_empty() {
        println!("{}", "  Modified files:".yellow().bold());
        for f in &modified {
            println!("    {} {}", "~".yellow(), f.path.yellow());
        }
    }

    if !deleted.is_empty() {
        println!("{}", "  Deleted files:".red().bold());
        for f in &deleted {
            println!("    {} {}", "-".red(), f.path.red());
        }
    }

    println!();
    println!(
        "  {} new  {} modified  {} deleted",
        new.len().to_string().green(),
        modified.len().to_string().yellow(),
        deleted.len().to_string().red()
    );
}
