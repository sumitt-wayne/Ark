use std::env;
use colored::Colorize;
use crate::core::repo;

pub fn run() {
    // Use current directory name as project name
    let project_name = env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "unknown".to_string());

    match repo::init(&project_name) {
        Ok(_) => {
            println!("{}", "Ark repository initialized successfully!".green().bold());
            println!("  {} .ark/", "created".cyan());
            println!("  {} .ark/commits/", "created".cyan());
            println!("  {} .ark/snapshots/", "created".cyan());
            println!("  {} .ark/config.json", "created".cyan());
            println!("\n{}", "You can now use 'ark save' to save your changes.".dimmed());
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
        }
    }
}
