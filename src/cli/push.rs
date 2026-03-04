use colored::Colorize;
use crate::core::repo;
use crate::git::git_wrapper;

pub fn run() {
    if !repo::is_initialized() {
        eprintln!("{}", "Error: Not an Ark repository. Run 'ark start' first.".red().bold());
        return;
    }

    if !git_wrapper::is_git_repo() {
        let result = git_wrapper::init();
        if !result.success {
            eprintln!("{} {}", "Error:".red().bold(), result.output);
            return;
        }
    }

    match git_wrapper::get_remote() {
        None => {
            println!("{}", "No remote configured.".yellow().bold());
            println!("  ark remote add <your-github-url>");
            return;
        }
        Some(remote) => {
            println!("  {} {}", "remote:".dimmed(), remote.cyan());
        }
    }

    println!("{}", "Staging changes...".dimmed());
    git_wrapper::add_all();

    println!("{}", "Committing changes...".dimmed());
    let commit = git_wrapper::commit("ark push");
    if !commit.success {
        if commit.output.contains("nothing to commit") {
            println!("{}", "✓ Nothing new to commit.".green());
        } else {
            eprintln!("{} {}", "Error:".red().bold(), commit.output);
            return;
        }
    } else {
        println!("{}", "✓ Changes committed.".green());
    }

    println!("{}", "Pushing to GitHub...".dimmed());
    let push = git_wrapper::push();
    if push.success {
        println!("{}", "✓ Pushed successfully!".green().bold());
    } else {
        eprintln!("{} {}", "Error pushing:".red().bold(), push.output);
    }
}
