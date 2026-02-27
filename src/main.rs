mod cli;
mod core;
mod git;
mod security;

use clap::{Parser, Subcommand};

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
        /// Optional commit message
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
    }
}
