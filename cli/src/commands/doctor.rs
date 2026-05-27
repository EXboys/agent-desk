use agent_desk_core::run_doctor;
use anyhow::Result;

pub fn run(json: bool) -> Result<()> {
    let report = run_doctor();

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("Agent Desk — doctor\n");

    if let Some(path) = &report.profile_env_path {
        let status = if report.profile_env_exists {
            "found"
        } else {
            "missing"
        };
        println!("Company profile: {status} ({path})");
    } else {
        println!("Company profile: unknown config directory");
    }

    if let Some(active) = &report.active_preset {
        println!("Active preset: {active}");
    } else {
        println!("Active preset: (none) — run `agent-desk profile init`");
    }

    println!();
    for runtime in &report.runtimes {
        let status = if runtime.installed {
            "installed"
        } else {
            "not installed"
        };
        println!("{} ({}) — {status}", runtime.display_name, runtime.id);
        if let Some(version) = &runtime.version {
            println!("  version: {version}");
        }
        if let Some(path) = &runtime.binary_path {
            println!("  binary: {path}");
        }
        for config_path in &runtime.config_paths {
            println!("  config: {config_path}");
        }
        if let Some(url) = &runtime.profile.gateway_url {
            println!("  gateway: {url}");
        }
        println!();
    }

    Ok(())
}
