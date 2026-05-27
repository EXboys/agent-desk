mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "agent-desk", about = "Manage desktop AI agents on one machine")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Discover installed runtimes, config paths, and gateway wiring
    Doctor {
        /// Emit JSON instead of human-readable output
        #[arg(long)]
        json: bool,
    },
    /// Apply company profile (not yet implemented)
    Setup {
        #[arg(long)]
        url: String,
        #[arg(long)]
        key: String,
    },
    /// Pull private SkillHub bundle (not yet implemented)
    Sync,
    /// Cache policies from control plane (not yet implemented)
    Policy {
        #[command(subcommand)]
        action: PolicyAction,
    },
}

#[derive(Subcommand)]
enum PolicyAction {
    Pull,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Doctor { json } => commands::doctor::run(json)?,
        Commands::Setup { url, key } => commands::setup::run(&url, &key)?,
        Commands::Sync => commands::sync::run()?,
        Commands::Policy { action } => match action {
            PolicyAction::Pull => commands::policy::pull()?,
        },
    }
    Ok(())
}
