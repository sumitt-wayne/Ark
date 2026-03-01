use colored::Colorize;
use std::io::{self, Write};
use std::fs;
use crate::core::{repo, tracker, commit};
use crate::ai::{groq, config};

pub fn run(action: &str) {
    if !repo::is_initialized() {
        eprintln!("{}", "Error: Not an Ark repository. Run 'ark start' first.".red().bold());
        return;
    }

    match action {
        "setup"   => ai_setup(),
        "commit"  => ai_commit(),
        "review"  => ai_review(),
        "fix"     => ai_fix(),
        "auto"    => ai_auto(),
        "explain" => ai_explain(),
        _ => eprintln!("{} Unknown action '{}'. Use: setup, commit, review, fix, auto, explain",
            "Error:".red().bold(), action),
    }
}

fn ai_setup() {
    println!("{}", "Ark AI Setup".bold().underline());
    println!("{}", "Enter your Groq API key (from console.groq.com):".dimmed());
    print!("  API Key: ");
    io::stdout().flush().unwrap();

    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key).unwrap();
    let api_key = api_key.trim().to_string();

    if api_key.is_empty() {
        eprintln!("{}", "Error: API key cannot be empty.".red());
        return;
    }

    // Encrypt before saving
    let encrypted = config::encrypt_key(&api_key);

    let ai_config = config::AiConfig {
        api_key: encrypted,
        model: "llama-3.3-70b-versatile".to_string(),
    };

    match config::save_config(&ai_config) {
        Ok(_) => {
            println!("{}", "✓ AI configured successfully!".green().bold());
            println!("  {} {}", "model:".dimmed(), ai_config.model.cyan());
            println!("{}", "You can now use 'ark ai commit', 'ark ai review', etc.".dimmed());
        }
        Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
    }
}

fn ai_commit() {
    if !config::is_configured() {
        eprintln!("{}", "Error: AI not configured. Run 'ark ai setup' first.".red());
        return;
    }

    let changes = tracker::scan_changes();

    if changes.is_empty() {
        println!("{}", "No changes to commit.".yellow());
        return;
    }

    let change_summary: Vec<String> = changes.iter().map(|f| {
        let status = match f.status {
            tracker::Status::New      => "added",
            tracker::Status::Modified => "modified",
            tracker::Status::Deleted  => "deleted",
            tracker::Status::Unchanged => "unchanged",
        };
        format!("{}: {}", status, f.path)
    }).collect();

    let prompt = format!(
        "Generate a concise git commit message (one line, max 72 chars) following conventional commits format (feat:, fix:, chore:, etc.) for these changes:\n{}\n\nRespond with ONLY the commit message, no explanation.",
        change_summary.join("\n")
    );

    println!("{}", "Generating commit message...".dimmed());

    match groq::generate(&prompt) {
        Ok(message) => {
            println!("{} {}", "Suggested:".green().bold(), message.cyan());
            println!();

            let snapshot = tracker::build_snapshot();
            match commit::save_commit(&message, snapshot) {
                Ok(id) => {
                    println!("{}", "Changes saved successfully!".green().bold());
                    println!("  {} {}", "id:".dimmed(), id.cyan());
                    println!("  {} {}", "message:".dimmed(), message.cyan());
                }
                Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
            }
        }
        Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
    }
}

fn ai_review() {
    if !config::is_configured() {
        eprintln!("{}", "Error: AI not configured. Run 'ark ai setup' first.".red());
        return;
    }

    let changes = tracker::scan_changes();

    if changes.is_empty() {
        println!("{}", "No changes to review.".yellow());
        return;
    }

    let change_summary: Vec<String> = changes.iter().map(|f| {
        let status = match f.status {
            tracker::Status::New      => "new file",
            tracker::Status::Modified => "modified",
            tracker::Status::Deleted  => "deleted",
            tracker::Status::Unchanged => "unchanged",
        };
        format!("{}: {}", status, f.path)
    }).collect();

    let prompt = format!(
        "As a senior code reviewer, review these file changes and provide 3 concise, actionable suggestions:\n{}\n\nBe specific and practical. Format as numbered list.",
        change_summary.join("\n")
    );

    println!("{}", "Reviewing changes...".dimmed());

    match groq::generate(&prompt) {
        Ok(review) => {
            println!("{}", "AI Code Review:".green().bold());
            println!();
            println!("{}", review);
        }
        Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
    }
}

fn ai_fix() {
    if !config::is_configured() {
        eprintln!("{}", "Error: AI not configured. Run 'ark ai setup' first.".red());
        return;
    }

    let changes = tracker::scan_changes();

    if changes.is_empty() {
        println!("{}", "No changes detected.".yellow());
        return;
    }

    // Read actual file content for better suggestions
    let mut file_contents = Vec::new();
    for f in &changes {
        if let Ok(content) = fs::read_to_string(&f.path) {
            let status = match f.status {
                tracker::Status::New      => "new file",
                tracker::Status::Modified => "modified",
                tracker::Status::Deleted  => "deleted",
                tracker::Status::Unchanged => "unchanged",
            };
            // Limit content to 500 chars per file
            let preview = content.chars().take(500).collect::<String>();
            file_contents.push(format!("[{}] {}\n```\n{}\n```", status, f.path, preview));
        }
    }

    let prompt = format!(
        "Analyze these file changes and suggest specific fixes or improvements:\n{}\n\nBe specific, practical, and concise. Format as numbered list.",
        file_contents.join("\n\n")
    );

    println!("{}", "Analyzing for fixes...".dimmed());

    match groq::generate(&prompt) {
        Ok(fix) => {
            println!("{}", "AI Suggestions:".green().bold());
            println!();
            println!("{}", fix);
        }
        Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
    }
}

fn ai_auto() {
    if !config::is_configured() {
        eprintln!("{}", "Error: AI not configured. Run 'ark ai setup' first.".red());
        return;
    }

    let changes = tracker::scan_changes();

    if changes.is_empty() {
        println!("{}", "No changes detected. Nothing to do.".yellow());
        return;
    }

    println!("{}", "Running AI auto mode...".dimmed());
    println!();

    let change_summary: Vec<String> = changes.iter().map(|f| {
        let status = match f.status {
            tracker::Status::New      => "added",
            tracker::Status::Modified => "modified",
            tracker::Status::Deleted  => "deleted",
            tracker::Status::Unchanged => "unchanged",
        };
        format!("{}: {}", status, f.path)
    }).collect();

    let prompt = format!(
        "Generate a concise git commit message (one line, max 72 chars) following conventional commits format for these changes:\n{}\n\nRespond with ONLY the commit message.",
        change_summary.join("\n")
    );

    match groq::generate(&prompt) {
        Ok(message) => {
            println!("{} {}", "Message:".green().bold(), message.cyan());
            println!();

            // Confirmation step
            print!("{}", "Save and push? (y/n): ".yellow().bold());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim().to_lowercase() != "y" {
                println!("{}", "Cancelled.".yellow());
                return;
            }

            // Save
            let snapshot = tracker::build_snapshot();
            match commit::save_commit(&message, snapshot) {
                Ok(id) => {
                    println!("{} {}", "✓ Saved:".green(), id.cyan());

                    // Sync
                    println!("{}", "✓ Syncing...".green());
                    crate::cli::sync::run();
                }
                Err(e) => eprintln!("{} {}", "Error saving:".red().bold(), e),
            }
        }
        Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
    }
}

fn ai_explain() {
    if !config::is_configured() {
        eprintln!("{}", "Error: AI not configured. Run 'ark ai setup' first.".red());
        return;
    }

    let history = commit::load_history();

    if history.is_empty() {
        println!("{}", "No commits to explain.".yellow());
        return;
    }

    let recent: Vec<String> = history.iter().rev().take(5).filter_map(|id| {
        commit::load_commit(id).ok().map(|c| {
            format!("- {} ({})", c.message, c.timestamp)
        })
    }).collect();

    let prompt = format!(
        "Explain what this project has been working on based on these recent commits:\n{}\n\nBe concise and clear, explain in simple terms.",
        recent.join("\n")
    );

    println!("{}", "Analyzing project history...".dimmed());

    match groq::generate(&prompt) {
        Ok(explanation) => {
            println!("{}", "Project Summary:".green().bold());
            println!();
            println!("{}", explanation);
        }
        Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
    }
}
