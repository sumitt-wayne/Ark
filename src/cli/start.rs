use std::env;
use std::io::{self, Write};
use colored::Colorize;
use crate::core::repo;
use crate::git::git_wrapper;

pub fn run() {
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
            println!();

            // Ask for remote setup
            print!("{}", "Do you want to add a GitHub remote? (y/n): ".dimmed());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim().to_lowercase() == "y" {
                print!("{}", "Enter remote URL: ".dimmed());
                io::stdout().flush().unwrap();

                let mut url = String::new();
                io::stdin().read_line(&mut url).unwrap();
                let url = url.trim();

                if !url.is_empty() {
                    if !git_wrapper::is_git_repo() {
                        git_wrapper::init();
                    }

                    let result = git_wrapper::set_remote(url);
                    if result.success {
                        println!("{} {}", "✓ Remote added:".green().bold(), url.cyan());
                    } else {
                        eprintln!("{} {}", "Error adding remote:".red(), result.output);
                    }
                }
            }

            println!();
            println!("{}", "You can now use 'ark save' to save your changes.".dimmed());
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
        }
    }
}
