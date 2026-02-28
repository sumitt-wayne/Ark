use colored::Colorize;
use crate::core::{repo, commit, branch};

pub fn run() {
    if !repo::is_initialized() {
        eprintln!("{}", "Error: Not an Ark repository. Run 'ark start' first.".red().bold());
        return;
    }

    let current_branch = branch::get_current_branch();
    println!("{} {}", "Branch:".dimmed(), current_branch.cyan().bold());
    println!();

    let history = commit::load_history();

    if history.is_empty() {
        println!("{}", "No saves found. Use 'ark save' to save your changes.".yellow());
        return;
    }

    println!("{}", "Ark History".bold().underline());
    println!();

    for (index, id) in history.iter().rev().enumerate() {
        match commit::load_commit(id) {
            Ok(c) => {
                let marker = if index == 0 {
                    "latest".green().bold()
                } else {
                    "".normal()
                };

                println!("  {} {}  {}", "●".cyan(), c.id.cyan().bold(), marker);
                println!("  {} {}", "message:".dimmed(), c.message);
                println!("  {} {}", "saved at:".dimmed(), c.timestamp);
                println!("  {} {} files tracked", "files:".dimmed(), c.files_snapshot.len().to_string().yellow());
                println!();
            }
            Err(e) => {
                eprintln!("{} {}", "Error loading commit:".red(), e);
            }
        }
    }

    println!("{} total saves", history.len().to_string().cyan().bold());
}
