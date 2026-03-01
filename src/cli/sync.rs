use colored::Colorize;
use crate::core::repo;
use crate::git::git_wrapper;

pub fn run() {
    if !repo::is_initialized() {
        eprintln!("{}", "Error: Not an Ark repository. Run 'ark start' first.".red().bold());
        return;
    }

    if !git_wrapper::is_git_repo() {
        println!("{}", "Initializing Git backend...".dimmed());
        let result = git_wrapper::init();
        if !result.success {
            eprintln!("{} {}", "Error:".red().bold(), result.output);
            return;
        }
        println!("{}", "✓ Git initialized.".green());
    }

    // Check remote — suggest ark remote add if not found
    match git_wrapper::get_remote() {
        None => {
            println!("{}", "No remote configured.".yellow().bold());
            println!("{}", "Add a remote first:".dimmed());
            println!("  ark remote add <your-github-url>");
            println!();
            println!("{}", "Example:".dimmed());
            println!("  ark remote add https://github.com/username/repo.git");
            return;
        }
        Some(remote) => {
            println!("  {} {}", "remote:".dimmed(), remote.cyan());
        }
    }

    // Pull
    println!("{}", "Pulling latest changes...".dimmed());
    let pull = git_wrapper::pull();
    if pull.success {
        println!("{}", "✓ Pull successful.".green());
    } else {
        println!("{} {}", "⚠ Pull warning:".yellow(), pull.output.dimmed());
    }

    // Stage
    println!("{}", "Staging changes...".dimmed());
    let add = git_wrapper::add_all();
    if !add.success {
        eprintln!("{} {}", "Error staging files:".red().bold(), add.output);
        return;
    }

    // Commit
    println!("{}", "Committing changes...".dimmed());
    let commit = git_wrapper::commit("ark sync");
    if !commit.success {
        if commit.output.contains("nothing to commit") {
            println!("{}", "✓ Nothing new to commit.".green());
        } else {
            eprintln!("{} {}", "Error committing:".red().bold(), commit.output);
            return;
        }
    } else {
        println!("{}", "✓ Changes committed.".green());
    }

    // Push
    println!("{}", "Pushing to remote...".dimmed());
    let push = git_wrapper::push();
    if push.success {
        println!("{}", "✓ Pushed successfully!".green().bold());
    } else {
        eprintln!("{} {}", "Error pushing:".red().bold(), push.output);
    }
}
