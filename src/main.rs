mod cli;
mod core;
mod git;
mod security;

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
        /// Action: new, go, list, delete
        action: String,
        /// Branch name
        name: Option<String>,
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
    }
}
