use colored::Colorize;
use crate::core::{repo, branch};

pub fn run(action: &str, name: Option<&str>) {
    if !repo::is_initialized() {
        eprintln!("{}", "Error: Not an Ark repository. Run 'ark start' first.".red().bold());
        return;
    }

    match action {
        "new" => {
            match name {
                None => eprintln!("{}", "Error: Branch name required. Usage: ark branch new <name>".red()),
                Some(n) => match branch::create_branch(n) {
                    Ok(_) => {
                        println!("{} {}", "✓ Branch created:".green().bold(), n.cyan());
                        println!("{}", "  Use 'ark branch go <name>' to switch.".dimmed());
                    }
                    Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
                },
            }
        }
        "go" => {
            match name {
                None => eprintln!("{}", "Error: Branch name required. Usage: ark branch go <name>".red()),
                Some(n) => {
                    if !branch::branch_exists(n) {
                        eprintln!("{} Branch '{}' not found.", "Error:".red().bold(), n);
                        return;
                    }
                    match branch::set_current_branch(n) {
                        Ok(_) => {
                            println!("{} {}", "✓ Switched to branch:".green().bold(), n.cyan());
                        }
                        Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
                    }
                }
            }
        }
        "list" => {
            let branches = branch::list_branches();
            let current = branch::get_current_branch();

            if branches.is_empty() {
                println!("{}", "No branches found.".yellow());
                return;
            }

            println!("{}", "Branches:".bold());
            for b in &branches {
                if b == &current {
                    println!("  {} {}", "→".green().bold(), b.green().bold());
                } else {
                    println!("  {} {}", " ".normal(), b.normal());
                }
            }
        }
        "delete" => {
            match name {
                None => eprintln!("{}", "Error: Branch name required. Usage: ark branch delete <name>".red()),
                Some(n) => match branch::delete_branch(n) {
                    Ok(_) => {
                        println!("{} {}", "✓ Branch deleted:".green().bold(), n.cyan());
                    }
                    Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
                },
            }
        }
        _ => {
            eprintln!("{} Unknown action '{}'. Use: new, go, list, delete", "Error:".red().bold(), action);
        }
    }
}
