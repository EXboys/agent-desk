use agent_desk_core::{adapter_by_id, show_config};
use anyhow::Result;

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
