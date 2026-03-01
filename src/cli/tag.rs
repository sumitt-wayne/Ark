use colored::Colorize;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::core::repo;
use crate::core::commit;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    pub commit_id: String,
    pub message: String,
    pub created_at: String,
}

pub fn run(action: &str, name: Option<&str>, message: Option<&str>) {
    if !repo::is_initialized() {
        eprintln!("{}", "Error: Not an Ark repository. Run 'ark start' first.".red().bold());
        return;
    }

    match action {
        "new" => create_tag(name, message),
        "list" => list_tags(),
        "delete" => delete_tag(name),
        _ => eprintln!("{} Unknown action '{}'. Use: new, list, delete",
            "Error:".red().bold(), action),
    }
}

fn create_tag(name: Option<&str>, message: Option<&str>) {
    let tag_name = match name {
        Some(n) => n,
        None => {
            eprintln!("{}", "Error: Tag name required. Usage: ark tag new <name>".red());
            return;
        }
    };

    // Get latest commit
    let history = commit::load_history();
    if history.is_empty() {
        eprintln!("{}", "Error: No commits found. Save changes first.".red());
        return;
    }

    let latest_id = history.last().unwrap().clone();

    fs::create_dir_all(".ark/tags")
        .expect("Failed to create tags directory");

    let tag_path = format!(".ark/tags/{}.json", tag_name);

    if Path::new(&tag_path).exists() {
        eprintln!("{} Tag '{}' already exists.", "Error:".red().bold(), tag_name);
        return;
    }

    let tag = Tag {
        name: tag_name.to_string(),
        commit_id: latest_id.clone(),
        message: message.unwrap_or("").to_string(),
        created_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    let json = serde_json::to_string_pretty(&tag)
        .expect("Failed to serialize tag");

    fs::write(&tag_path, json).expect("Failed to write tag");

    println!("{} {}", "✓ Tag created:".green().bold(), tag_name.cyan());
    println!("  {} {}", "commit:".dimmed(), latest_id.cyan());
    if !tag.message.is_empty() {
        println!("  {} {}", "message:".dimmed(), tag.message);
    }
}

fn list_tags() {
    let tags_dir = Path::new(".ark/tags");

    if !tags_dir.exists() {
        println!("{}", "No tags found.".yellow());
        return;
    }

    let mut tags: Vec<Tag> = Vec::new();

    if let Ok(entries) = fs::read_dir(tags_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(tag) = serde_json::from_str::<Tag>(&content) {
                        tags.push(tag);
                    }
                }
            }
        }
    }

    if tags.is_empty() {
        println!("{}", "No tags found.".yellow());
        return;
    }

    println!("{}", "Tags:".bold().underline());
    println!();

    for tag in &tags {
        println!("  {} {}", "▶".cyan(), tag.name.cyan().bold());
        println!("    {} {}", "commit:".dimmed(), tag.commit_id.dimmed());
        println!("    {} {}", "created:".dimmed(), tag.created_at.dimmed());
        if !tag.message.is_empty() {
            println!("    {} {}", "message:".dimmed(), tag.message);
        }
        println!();
    }
}

fn delete_tag(name: Option<&str>) {
    let tag_name = match name {
        Some(n) => n,
        None => {
            eprintln!("{}", "Error: Tag name required. Usage: ark tag delete <name>".red());
            return;
        }
    };

    let tag_path = format!(".ark/tags/{}.json", tag_name);

    if !Path::new(&tag_path).exists() {
        eprintln!("{} Tag '{}' not found.", "Error:".red().bold(), tag_name);
        return;
    }

    fs::remove_file(&tag_path).expect("Failed to delete tag");
    println!("{} {}", "✓ Tag deleted:".green().bold(), tag_name.cyan());
}
