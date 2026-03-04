use colored::Colorize;
use crate::core::repo;
use crate::git::git_wrapper;

pub fn run() {
    if !repo::is_initialized() {
        eprintln!("{}", "Error: Not an Ark repository. Run 'ark start' first.".red().bold());
        return;
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

    println!("{}", "Pulling from GitHub...".dimmed());
    let pull = git_wrapper::pull();
    if pull.success {
        println!("{}", "✓ Pull successful.".green().bold());
    } else {
        eprintln!("{} {}", "Error pulling:".red().bold(), pull.output);
    }
}
