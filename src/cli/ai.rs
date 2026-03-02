use colored::Colorize;
use std::io::{self, Write};
use std::fs;
use crate::core::{repo, tracker, commit, branch};
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
        "diff"    => ai_diff(),
        "suggest" => ai_suggest(),
        _ => {
            eprintln!("{} Unknown action '{}'.", "Error:".red().bold(), action);
            println!();
            println!("{}", "Available commands:".bold());
            println!("  {} {}", "ark ai setup".cyan(),   "→ Configure AI API key");
            println!("  {} {}", "ark ai commit".cyan(),  "→ Generate smart commit message");
            println!("  {} {}", "ark ai review".cyan(),  "→ Review your changes");
            println!("  {} {}", "ark ai fix".cyan(),     "→ Get fix suggestions");
            println!("  {} {}", "ark ai auto".cyan(),    "→ Auto save + push");
            println!("  {} {}", "ark ai explain".cyan(), "→ Explain project history");
            println!("  {} {}", "ark ai diff".cyan(),    "→ Explain current changes");
            println!("  {} {}", "ark ai suggest".cyan(), "→ Get next step suggestions");
        }
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

    let encrypted = config::encrypt_key(&api_key);

    let ai_config = config::AiConfig {
        api_key: encrypted,
        model: "llama-3.3-70b-versatile".to_string(),
    };

    match config::save_config(&ai_config) {
        Ok(_) => {
            println!("{}", "✓ AI configured successfully!".green().bold());
            println!("  {} {}", "model:".dimmed(), ai_config.model.cyan());
            println!();
            println!("{}", "Available commands:".dimmed());
            println!("  ark ai commit  → smart commit message");
            println!("  ark ai review  → code review");
            println!("  ark ai fix     → fix suggestions");
            println!("  ark ai auto    → auto save + push");
            println!("  ark ai diff    → explain changes");
            println!("  ark ai suggest → next step suggestions");
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

    // Read actual file content for better commit messages
    let mut file_contents = Vec::new();
    for f in &changes {
        let status = match f.status {
            tracker::Status::New      => "added",
            tracker::Status::Modified => "modified",
            tracker::Status::Deleted  => "deleted",
            tracker::Status::Unchanged => "unchanged",
        };

        if f.status == tracker::Status::Deleted {
            file_contents.push(format!("[{}] {}", status, f.path));
        } else if let Ok(content) = fs::read_to_string(&f.path) {
            let preview = content.chars().take(300).collect::<String>();
            file_contents.push(format!("[{}] {}\n{}", status, f.path, preview));
        } else {
            file_contents.push(format!("[{}] {}", status, f.path));
        }
    }

    let prompt = format!(
        "Generate a concise git commit message (one line, max 72 chars) following conventional commits format (feat:, fix:, chore:, docs:, refactor:, test:) based on these file changes:\n\n{}\n\nRespond with ONLY the commit message. No explanation, no quotes.",
        file_contents.join("\n\n")
    );

    println!("{}", "⚡ Generating commit message...".dimmed());

    match groq::generate(&prompt) {
        Ok(message) => {
            println!();
            println!("  {} {}", "→".green().bold(), message.cyan().bold());
            println!();

            let snapshot = tracker::build_snapshot();
            match commit::save_commit(&message, snapshot) {
                Ok(id) => {
                    println!("{}", "✓ Changes saved!".green().bold());
                    println!("  {} {}", "id:".dimmed(), id.dimmed());
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

    let mut file_contents = Vec::new();
    for f in &changes {
        let status = match f.status {
            tracker::Status::New      => "new file",
            tracker::Status::Modified => "modified",
            tracker::Status::Deleted  => "deleted",
            tracker::Status::Unchanged => "unchanged",
        };

        if let Ok(content) = fs::read_to_string(&f.path) {
            let preview = content.chars().take(400).collect::<String>();
            file_contents.push(format!("[{}] {}\n```\n{}\n```", status, f.path, preview));
        } else {
            file_contents.push(format!("[{}] {}", status, f.path));
        }
    }

    let prompt = format!(
        "As a senior code reviewer, review these changes and give 3 specific, actionable suggestions. Be concise:\n\n{}\n\nFormat as numbered list. Focus on code quality, bugs, and improvements.",
        file_contents.join("\n\n")
    );

    println!("{}", "⚡ Reviewing changes...".dimmed());

    match groq::generate(&prompt) {
        Ok(review) => {
            println!();
            println!("{}", "Code Review:".green().bold().underline());
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

    let mut file_contents = Vec::new();
    for f in &changes {
        if let Ok(content) = fs::read_to_string(&f.path) {
            let status = match f.status {
                tracker::Status::New      => "new file",
                tracker::Status::Modified => "modified",
                tracker::Status::Deleted  => "deleted",
                tracker::Status::Unchanged => "unchanged",
            };
            let preview = content.chars().take(500).collect::<String>();
            file_contents.push(format!("[{}] {}\n```\n{}\n```", status, f.path, preview));
        }
    }

    let prompt = format!(
        "Analyze these code changes and suggest specific fixes or improvements:\n\n{}\n\nBe specific and practical. Format as numbered list.",
        file_contents.join("\n\n")
    );

    println!("{}", "⚡ Analyzing for fixes...".dimmed());

    match groq::generate(&prompt) {
        Ok(fix) => {
            println!();
            println!("{}", "Fix Suggestions:".green().bold().underline());
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

    println!("{}", "⚡ AI Auto Mode".bold());
    println!();

    // Show changes summary
    let new_count = changes.iter().filter(|f| f.status == tracker::Status::New).count();
    let mod_count = changes.iter().filter(|f| f.status == tracker::Status::Modified).count();
    let del_count = changes.iter().filter(|f| f.status == tracker::Status::Deleted).count();
    println!("  {} {} new  {} modified  {} deleted",
        "Changes:".dimmed(),
        new_count.to_string().green(),
        mod_count.to_string().yellow(),
        del_count.to_string().red()
    );
    println!();

    // Read file content for better message
    let mut file_contents = Vec::new();
    for f in &changes {
        let status = match f.status {
            tracker::Status::New      => "added",
            tracker::Status::Modified => "modified",
            tracker::Status::Deleted  => "deleted",
            tracker::Status::Unchanged => "unchanged",
        };
        if let Ok(content) = fs::read_to_string(&f.path) {
            let preview = content.chars().take(300).collect::<String>();
            file_contents.push(format!("[{}] {}\n{}", status, f.path, preview));
        } else {
            file_contents.push(format!("[{}] {}", status, f.path));
        }
    }

    let prompt = format!(
        "Generate a concise git commit message (one line, max 72 chars) following conventional commits format for these changes:\n\n{}\n\nRespond with ONLY the commit message. No explanation, no quotes.",
        file_contents.join("\n\n")
    );

    println!("{}", "  Generating message...".dimmed());

    match groq::generate(&prompt) {
        Ok(message) => {
            println!("  {} {}", "→".green().bold(), message.cyan().bold());
            println!();

            print!("{}", "  Save and push? (y/n): ".yellow().bold());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim().to_lowercase() != "y" {
                println!("{}", "  Cancelled.".yellow());
                return;
            }

            println!();
            let snapshot = tracker::build_snapshot();
            match commit::save_commit(&message, snapshot) {
                Ok(id) => {
                    println!("  {} {}", "✓ Saved:".green(), id.dimmed());
                    crate::cli::sync::run();
                }
                Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
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

    let current = branch::get_current_branch();
    let recent: Vec<String> = history.iter().rev().take(5).filter_map(|id| {
        commit::load_commit(id).ok().map(|c| {
            format!("- {} ({})", c.message, c.timestamp)
        })
    }).collect();

    let prompt = format!(
        "Explain what this project has been working on based on these recent commits from branch '{}':\n{}\n\nBe concise and clear. Explain in simple terms what was built or changed.",
        current,
        recent.join("\n")
    );

    println!("{}", "⚡ Analyzing project history...".dimmed());

    match groq::generate(&prompt) {
        Ok(explanation) => {
            println!();
            println!("{}", "Project Summary:".green().bold().underline());
            println!();
            println!("{}", explanation);
        }
        Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
    }
}

fn ai_diff() {
    if !config::is_configured() {
        eprintln!("{}", "Error: AI not configured. Run 'ark ai setup' first.".red());
        return;
    }

    let changes = tracker::scan_changes();

    if changes.is_empty() {
        println!("{}", "No changes to explain.".yellow());
        return;
    }

    let mut file_contents = Vec::new();
    for f in &changes {
        let status = match f.status {
            tracker::Status::New      => "new file",
            tracker::Status::Modified => "modified",
            tracker::Status::Deleted  => "deleted",
            tracker::Status::Unchanged => "unchanged",
        };

        if let Ok(content) = fs::read_to_string(&f.path) {
            let preview = content.chars().take(400).collect::<String>();
            file_contents.push(format!("[{}] {}\n```\n{}\n```", status, f.path, preview));
        } else {
            file_contents.push(format!("[{}] {}", status, f.path));
        }
    }

    let prompt = format!(
        "Explain these code changes in simple, clear language. What was changed and why might these changes have been made?\n\n{}\n\nBe concise. Use simple language a junior developer would understand.",
        file_contents.join("\n\n")
    );

    println!("{}", "⚡ Explaining changes...".dimmed());

    match groq::generate(&prompt) {
        Ok(explanation) => {
            println!();
            println!("{}", "Change Explanation:".green().bold().underline());
            println!();
            println!("{}", explanation);
        }
        Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
    }
}

fn ai_suggest() {
    if !config::is_configured() {
        eprintln!("{}", "Error: AI not configured. Run 'ark ai setup' first.".red());
        return;
    }

    let changes = tracker::scan_changes();
    let history = commit::load_history();
    let current_branch = branch::get_current_branch();

    // Build context
    let changes_summary: Vec<String> = changes.iter().map(|f| {
        let status = match f.status {
            tracker::Status::New      => "new",
            tracker::Status::Modified => "modified",
            tracker::Status::Deleted  => "deleted",
            tracker::Status::Unchanged => "unchanged",
        };
        format!("{}: {}", status, f.path)
    }).collect();

    let recent_commits: Vec<String> = history.iter().rev().take(3).filter_map(|id| {
        commit::load_commit(id).ok().map(|c| c.message)
    }).collect();

    let prompt = format!(
        "Based on this project context, suggest the next 3 best actions a developer should take:\n\nCurrent branch: {}\nUncommitted changes: {}\nRecent commits:\n{}\n\nYou MUST suggest ONLY ark commands (ark save, ark ai commit, ark ai auto, ark sync, ark branch, ark diff, etc.). NEVER suggest git commands. Format as numbered list with the exact ark command to run.",
        current_branch,
        if changes.is_empty() { "none".to_string() } else { changes_summary.join(", ") },
        if recent_commits.is_empty() { "none".to_string() } else { recent_commits.join("\n") }
    );

    println!("{}", "⚡ Analyzing project state...".dimmed());

    match groq::generate(&prompt) {
        Ok(suggestion) => {
            println!();
            println!("{}", "Suggested Next Steps:".green().bold().underline());
            println!();
            println!("{}", suggestion);
        }
        Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
    }
}
