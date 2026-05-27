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
    /// List, create, and switch local model presets
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
    },
    /// Show runtime-specific configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
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
enum ProfileAction {
    /// Create example ~/.config/agent-desk/profiles.yaml
    Init,
    /// List configured presets
    List,
    /// Activate a preset and apply it to installed runtimes
    Use {
        /// Profile name (e.g. work, personal)
        name: String,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show current model settings for a runtime
    Show {
        /// Runtime id (e.g. hermes, openclaw, claude-code)
        runtime: String,
        /// Emit JSON
        #[arg(long)]
        json: bool,
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
        Commands::Profile { action } => match action {
            ProfileAction::Init => commands::profile::init()?,
            ProfileAction::List => commands::profile::list()?,
            ProfileAction::Use { name } => commands::profile::activate(&name)?,
        },
        Commands::Config { action } => match action {
            ConfigAction::Show { runtime, json } => commands::config::show(&runtime, json)?,
        },
        Commands::Setup { url, key } => commands::setup::run(&url, &key)?,
        Commands::Sync => commands::sync::run()?,
        Commands::Policy { action } => match action {
            PolicyAction::Pull => commands::policy::pull()?,
        },
    }
    Ok(())
}
