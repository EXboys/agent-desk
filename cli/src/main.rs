mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "agent-doctor",
    about = "Diagnose, back up, and repair local AI agent runtimes"
)]
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
    /// Show or update runtime-specific configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Back up, diagnose, and repair a runtime
    Repair {
        /// Runtime id (e.g. openclaw, hermes, claude-code, codex)
        runtime: String,
        /// Execute backup, typed actions, re-probe verification, and write audit metadata
        #[arg(long, conflicts_with = "rollback")]
        apply: bool,
        /// Restore config files from a backup snapshot (latest, or --backup id)
        #[arg(long, conflicts_with = "apply")]
        rollback: bool,
        /// Backup id to restore (with --rollback); default is latest for this runtime
        #[arg(long, requires = "rollback")]
        backup: Option<String>,
        /// Bounded probe → plan → apply → verify loop (pair with --apply to execute fixes)
        #[arg(long = "loop", conflicts_with = "rollback")]
        repair_loop: bool,
        /// Planner for --loop: deterministic (default) or ai (placeholder)
        #[arg(long, default_value = "deterministic")]
        plan: String,
        /// Emit JSON (with --apply or --rollback)
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
enum ProfileAction {
    /// Create example ~/.config/agent-doctor/profiles.yaml
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
    /// Write model settings to a runtime config file
    Set {
        /// Runtime id (e.g. hermes)
        runtime: String,
        #[arg(long)]
        provider: Option<String>,
        #[arg(long)]
        model: Option<String>,
        #[arg(long)]
        base_url: Option<String>,
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
            ConfigAction::Set {
                runtime,
                provider,
                model,
                base_url,
            } => commands::config::set(&runtime, provider, model, base_url)?,
        },
        Commands::Repair {
            runtime,
            apply,
            rollback,
            backup,
            repair_loop,
            plan,
            json,
        } => commands::repair::run(
            &runtime,
            apply,
            rollback,
            backup.as_deref(),
            repair_loop,
            &plan,
            json,
        )?,
        Commands::Setup { url, key } => commands::setup::run(&url, &key)?,
        Commands::Sync => commands::sync::run()?,
        Commands::Policy { action } => match action {
            PolicyAction::Pull => commands::policy::pull()?,
        },
    }
    Ok(())
}
