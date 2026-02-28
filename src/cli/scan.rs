use colored::Colorize;
use crate::core::repo;
use crate::security::scanner;

pub fn run() {
    if !repo::is_initialized() {
        eprintln!("{}", "Error: Not an Ark repository. Run 'ark start' first.".red().bold());
        return;
    }

    println!("{}", "Scanning for secrets and sensitive data...".dimmed());
    println!();

    let results = scanner::scan_files();

    if results.is_empty() {
        println!("{}", "✓ No secrets or sensitive data found.".green().bold());
        return;
    }

    println!("{}", format!("⚠ Found {} potential issue(s):", results.len()).red().bold());
    println!();

    for result in &results {
        println!("  {} {}", "file:".dimmed(), result.file.yellow().bold());
        println!("  {} {}", "line:".dimmed(), result.line_number.to_string().cyan());
        println!("  {} {}", "issue:".dimmed(), result.issue.red());
        println!("  {} {}", "code:".dimmed(), result.line.dimmed());
        println!();
    }

    println!("{}", "⚠ Review these files before syncing to remote.".yellow().bold());
}
