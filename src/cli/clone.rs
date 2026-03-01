use colored::Colorize;
use crate::git::git_wrapper;
use std::path::Path;

pub fn run(url: &str, dir_name: Option<&str>) {
    let folder = match dir_name {
        Some(d) => d.to_string(),
        None => url
            .split('/')
            .last()
            .unwrap_or("repo")
            .replace(".git", ""),
    };

    println!("{} {}", "Cloning into:".dimmed(), folder.cyan().bold());
    println!("{} {}", "URL:".dimmed(), url.dimmed());
    println!();

    if Path::new(&folder).exists() {
        eprintln!("{} Directory '{}' already exists.", "Error:".red().bold(), folder);
        return;
    }

    let result = git_wrapper::clone(url, &folder);

    if !result.success {
        eprintln!("{} {}", "Clone failed:".red().bold(), result.output);
        return;
    }

    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&folder).unwrap();

    match crate::core::repo::init(&folder) {
        Ok(_) => {
            println!("{}", "✓ Cloned successfully!".green().bold());
            println!("{} {}", "✓ Ark initialized in:".green(), folder.cyan());
            println!();
            println!("{} cd {}", "Next:".dimmed(), folder.cyan());
        }
        Err(_) => {
            println!("{}", "✓ Cloned successfully!".green().bold());
            println!("{} {}", "  folder:".dimmed(), folder.cyan());
        }
    }

    std::env::set_current_dir(original_dir).unwrap();
}
