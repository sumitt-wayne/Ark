use colored::Colorize;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::core::{repo, tracker};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stash {
    pub id: usize,
    pub message: String,
    pub timestamp: String,
    pub snapshot: std::collections::HashMap<String, String>,
}

pub fn run(action: &str, message: Option<&str>) {
    if !repo::is_initialized() {
        eprintln!("{}", "Error: Not an Ark repository. Run 'ark start' first.".red().bold());
        return;
    }

    match action {
        "save"  => stash_save(message),
        "list"  => stash_list(),
        "pop"   => stash_pop(),
        "drop"  => stash_drop(),
        _ => eprintln!("{} Unknown action '{}'. Use: save, list, pop, drop",
            "Error:".red().bold(), action),
    }
}

fn stash_save(message: Option<&str>) {
    let changes = tracker::scan_changes();

    if changes.is_empty() {
        println!("{}", "Nothing to stash. No changes detected.".yellow());
        return;
    }

    fs::create_dir_all(".ark/stash")
        .expect("Failed to create stash directory");

    // Load existing stashes
    let mut stashes = load_stashes();
    let id = stashes.len();

    let snapshot = tracker::build_snapshot();
    let msg = message.unwrap_or("WIP stash").to_string();

    let stash = Stash {
        id,
        message: msg.clone(),
        timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        snapshot,
    };

    stashes.push(stash);
    save_stashes(&stashes);

    println!("{} stash@{}", "✓ Stashed:".green().bold(), id.to_string().cyan());
    println!("  {} {}", "message:".dimmed(), msg.cyan());
    println!("  {} {} changes stashed", "files:".dimmed(), changes.len().to_string().cyan());
}

fn stash_list() {
    let stashes = load_stashes();

    if stashes.is_empty() {
        println!("{}", "No stashes found.".yellow());
        return;
    }

    println!("{}", "Stashes:".bold().underline());
    println!();

    for stash in stashes.iter().rev() {
        println!("  {} {} {}",
            format!("stash@{}", stash.id).cyan().bold(),
            "→".dimmed(),
            stash.message
        );
        println!("    {} {}", "saved at:".dimmed(), stash.timestamp.dimmed());
        println!();
    }
}

fn stash_pop() {
    let mut stashes = load_stashes();

    if stashes.is_empty() {
        println!("{}", "No stashes found.".yellow());
        return;
    }

    let stash = stashes.pop().unwrap();

    // Restore snapshot
    let current_branch = crate::core::branch::get_current_branch();
    let snapshot_json = serde_json::to_string_pretty(&stash.snapshot)
        .expect("Failed to serialize snapshot");

    fs::write(
        format!(".ark/snapshots/{}.json", current_branch),
        snapshot_json,
    ).expect("Failed to restore snapshot");

    save_stashes(&stashes);

    println!("{} stash@{}", "✓ Popped:".green().bold(), stash.id.to_string().cyan());
    println!("  {} {}", "message:".dimmed(), stash.message);
    println!("{}", "Stash applied and removed.".dimmed());
}

fn stash_drop() {
    let mut stashes = load_stashes();

    if stashes.is_empty() {
        println!("{}", "No stashes found.".yellow());
        return;
    }

    let stash = stashes.pop().unwrap();
    save_stashes(&stashes);

    println!("{} stash@{}", "✓ Dropped:".green().bold(), stash.id.to_string().cyan());
    println!("  {} {}", "message:".dimmed(), stash.message);
}

fn load_stashes() -> Vec<Stash> {
    let path = Path::new(".ark/stash/stashes.json");
    if !path.exists() {
        return Vec::new();
    }

    let content = fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

fn save_stashes(stashes: &Vec<Stash>) {
    let json = serde_json::to_string_pretty(stashes)
        .expect("Failed to serialize stashes");

    fs::create_dir_all(".ark/stash").expect("Failed to create stash dir");
    fs::write(".ark/stash/stashes.json", json)
        .expect("Failed to write stashes");
}
