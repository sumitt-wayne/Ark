use colored::Colorize;
use crate::core::{repo, branch};
use std::fs;

pub fn run(action: &str, name: Option<&str>, new_name: Option<&str>) {
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
                        Ok(_) => println!("{} {}", "✓ Switched to branch:".green().bold(), n.cyan()),
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
                    println!("    {}", b.normal());
                }
            }
        }
        "delete" => {
            match name {
                None => eprintln!("{}", "Error: Branch name required. Usage: ark branch delete <name>".red()),
                Some(n) => match branch::delete_branch(n) {
                    Ok(_) => println!("{} {}", "✓ Branch deleted:".green().bold(), n.cyan()),
                    Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
                },
            }
        }
        "rename" => {
            match (name, new_name) {
                (Some(old), Some(new)) => rename_branch(old, new),
                _ => eprintln!("{}", "Error: Usage: ark branch rename <old> <new>".red()),
            }
        }
        _ => {
            eprintln!("{} Unknown action '{}'. Use: new, go, list, delete, rename",
                "Error:".red().bold(), action);
        }
    }
}

fn rename_branch(old_name: &str, new_name: &str) {
    if old_name == "main" {
        eprintln!("{}", "Error: Cannot rename 'main' branch.".red());
        return;
    }

    if !branch::branch_exists(old_name) {
        eprintln!("{} Branch '{}' not found.", "Error:".red().bold(), old_name);
        return;
    }

    if branch::branch_exists(new_name) {
        eprintln!("{} Branch '{}' already exists.", "Error:".red().bold(), new_name);
        return;
    }

    let mut branch_data = match branch::load_branch(old_name) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            return;
        }
    };

    branch_data.name = new_name.to_string();

    if let Err(e) = branch::save_branch(&branch_data) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        return;
    }

    let old_snapshot = format!(".ark/snapshots/{}.json", old_name);
    let new_snapshot = format!(".ark/snapshots/{}.json", new_name);
    if std::path::Path::new(&old_snapshot).exists() {
        let _ = fs::rename(&old_snapshot, &new_snapshot);
    }

    let _ = fs::remove_file(format!(".ark/branches/{}.json", old_name));

    let current = branch::get_current_branch();
    if current == old_name {
        let _ = branch::set_current_branch(new_name);
    }

    println!("{} {} {} {}",
        "✓ Branch renamed:".green().bold(),
        old_name.red(),
        "→".dimmed(),
        new_name.cyan()
    );
}
