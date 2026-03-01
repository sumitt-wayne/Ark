mod cli;
mod core;
mod git;
mod security;
mod ai;

use clap::{Parser, Subcommand};
use core::repo;

#[derive(Parser)]
#[command(
    name = "ark",
    about = "Ark - Simple, secure version control",
    version = "0.1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start tracking a project
    Start,
    /// Save your changes
    Save {
        message: Option<String>,
    },
    /// Check current status
    Check,
    /// View history
    History,
    /// Sync with remote
    Sync,
    /// Undo last save
    Undo,
    /// Scan for secrets
    Scan,
    /// Show project info
    Info,
    /// Manage branches
    Branch {
        action: String,
        name: Option<String>,
    },
    /// AI powered features
    Ai {
        action: String,
    },
    /// Manage remote repository
    Remote {
        action: String,
        url: Option<String>,
    },
    /// Show changes/diff
    Diff {
        commit_id: Option<String>,
    },
    /// Merge a branch into current branch
    Merge {
        branch: String,
    },
    /// Clone a remote repository
    Clone {
        url: String,
        dir: Option<String>,
    },
    /// Manage version tags
    Tag {
        action: String,
        name: Option<String>,
        message: Option<String>,
    },
    /// Temporarily save changes
    Stash {
        action: String,
        message: Option<String>,
    },
    /// Restore a file from a commit
    Restore {
        /// File path to restore
        file: String,
        /// Optional commit ID
        commit_id: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start => cli::start::run(),
        Commands::Save { message } => cli::save::run(message),
        Commands::Check => cli::check::run(),
        Commands::History => cli::history::run(),
        Commands::Sync => cli::sync::run(),
        Commands::Undo => cli::undo::run(),
        Commands::Scan => cli::scan::run(),
        Commands::Info => {
            match repo::load_config() {
                Ok(config) => {
                    println!("Project: {}", config.project_name);
                    println!("Version: {}", config.version);
                    println!("Created: {}", config.created_at);
                }
                Err(e) => eprintln!("{}", e),
            }
        }
        Commands::Branch { action, name } => {
            cli::branch::run(&action, name.as_deref());
        }
        Commands::Ai { action } => {
            cli::ai::run(&action);
        }
        Commands::Remote { action, url } => {
            cli::remote::run(&action, url.as_deref());
        }
        Commands::Diff { commit_id } => {
            cli::diff::run(commit_id.as_deref());
        }
        Commands::Merge { branch } => {
            cli::merge::run(&branch);
        }
        Commands::Clone { url, dir } => {
            cli::clone::run(&url, dir.as_deref());
        }
        Commands::Tag { action, name, message } => {
            cli::tag::run(&action, name.as_deref(), message.as_deref());
        }
        Commands::Stash { action, message } => {
            cli::stash::run(&action, message.as_deref());
        }
        Commands::Restore { file, commit_id } => {
            cli::restore::run(&file, commit_id.as_deref());
        }
    }
}
