use agent_desk_core::{
    init_example_profiles, load_profiles, profiles_path, use_profile, UseProfileReport,
};
use anyhow::Result;

pub fn list() -> Result<()> {
    let doc = load_profiles()?;
    let path = profiles_path()?;

    if doc.profiles.is_empty() {
        println!("No profiles configured.");
        println!("Create one with: agent-desk profile init");
        println!("Expected file: {}", path.display());
        return Ok(());
    }

    println!("Profiles ({})", path.display());
    if let Some(active) = &doc.active {
        println!("Active: {active}\n");
    } else {
        println!("Active: (none)\n");
    }

    for (name, entry) in &doc.profiles {
        let marker = if doc.active.as_deref() == Some(name.as_str()) {
            "*"
        } else {
            " "
        };
        let hermes = entry
            .hermes
            .as_ref()
            .map(|h| {
                if h.provider == "ollama" {
                    format!("hermes: {} / {} @ {}", h.provider, h.model, h.base_url)
                } else {
                    format!("hermes: {} / {}", h.provider, h.model)
                }
            })
            .unwrap_or_else(|| "hermes: (not set)".to_string());
        println!("{marker} {name} — {hermes}");
    }

    Ok(())
}

pub fn init() -> Result<()> {
    let path = init_example_profiles()?;
    println!("Created example profiles at {}", path.display());
    println!("\nTry:");
    println!("  agent-desk profile list");
    println!("  agent-desk profile use local    # Ollama @ 127.0.0.1:11434, no API key");
    println!("  agent-desk profile use work");
    Ok(())
}

pub fn activate(name: &str) -> Result<()> {
    let report = use_profile(name)?;
    print_report(&report);
    Ok(())
}

pub fn print_report(report: &UseProfileReport) {
    println!("Activated profile: {}\n", report.profile);

    for item in &report.applied {
        println!("✓ {} updated", item.runtime_id);
        println!("  config: {}", item.config_path);
        if let Some(backup) = &item.backup_path {
            println!("  backup: {backup}");
        }
        println!("  hint:   {}", item.restart_hint);
        println!();
    }

    for item in &report.skipped {
        println!("• skipped: {item}");
    }
}
