use agent_desk_core::{adapter_by_id, set_runtime_model, show_config, RuntimeModelPreset};
use anyhow::{Context, Result};

pub fn show(runtime: &str, json: bool) -> Result<()> {
    if json {
        println!("{}", show_config(runtime)?);
        return Ok(());
    }

    let adapter =
        adapter_by_id(runtime).ok_or_else(|| anyhow::anyhow!("unknown runtime '{runtime}'"))?;
    let model = adapter
        .read_model()?
        .ok_or_else(|| anyhow::anyhow!("{runtime} does not expose model settings yet"))?;

    println!("{} model settings\n", adapter.display_name());
    if let Some(provider) = &model.provider {
        println!("  provider: {provider}");
    }
    if let Some(model_name) = &model.model {
        println!("  model:    {model_name}");
    }
    if let Some(base_url) = &model.base_url {
        println!("  base_url: {base_url}");
    }

    Ok(())
}

pub fn set(
    runtime: &str,
    provider: Option<String>,
    model: Option<String>,
    base_url: Option<String>,
) -> Result<()> {
    let adapter =
        adapter_by_id(runtime).ok_or_else(|| anyhow::anyhow!("unknown runtime '{runtime}'"))?;
    let current = adapter
        .read_model()?
        .ok_or_else(|| anyhow::anyhow!("{runtime} does not expose model settings yet"))?;

    let preset = RuntimeModelPreset {
        provider: provider
            .or(current.provider)
            .context("provider is required")?,
        model: model.or(current.model).context("model is required")?,
        base_url: base_url
            .or(current.base_url)
            .context("base_url is required")?,
    };

    let report = set_runtime_model(runtime, preset, None)?;
    println!("✓ {} updated", report.runtime_id);
    println!("  config: {}", report.config_path);
    if let Some(backup) = report.backup_path {
        println!("  backup: {backup}");
    }
    println!("  hint:   {}", report.restart_hint);
    Ok(())
}
