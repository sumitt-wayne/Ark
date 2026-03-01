use colored::Colorize;
use crate::git::git_wrapper;

pub fn run(action: &str, url: Option<&str>) {
    match action {
        "add" => {
            match url {
                None => {
                    eprintln!("{}", "Error: URL required. Usage: ark remote add <url>".red());
                }
                Some(u) => {
                    // Check if remote already exists
                    if git_wrapper::get_remote().is_some() {
                        eprintln!("{}", "Error: Remote already exists.".red());
                        eprintln!("  To update: git remote set-url origin <url>");
                        return;
                    }

                    // Init git if not already
                    if !git_wrapper::is_git_repo() {
                        git_wrapper::init();
                    }

                    let result = git_wrapper::set_remote(u);
                    if result.success {
                        println!("{} {}", "✓ Remote added:".green().bold(), u.cyan());
                        println!("{}", "You can now use 'ark sync' to push changes.".dimmed());
                    } else {
                        eprintln!("{} {}", "Error:".red().bold(), result.output);
                    }
                }
            }
        }
        "show" => {
            match git_wrapper::get_remote() {
                Some(remote) => {
                    println!("{} {}", "Remote:".dimmed(), remote.cyan());
                }
                None => {
                    println!("{}", "No remote configured.".yellow());
                    println!("{}", "Add one with: ark remote add <url>".dimmed());
                }
            }
        }
        _ => {
            eprintln!("{} Unknown action '{}'. Use: add, show",
                "Error:".red().bold(), action);
        }
    }
}
