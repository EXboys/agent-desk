use std::fs;
use std::path::PathBuf;

use crate::adapter::{AdapterDiscovery, RuntimeAdapter, RuntimeProfile};
use crate::adapters::util::{discover_binary, home_join};

pub struct HermesAdapter;

impl RuntimeAdapter for HermesAdapter {
    fn id(&self) -> &'static str {
        "hermes"
    }

    fn display_name(&self) -> &'static str {
        "Hermes Agent"
    }

    fn discover(&self) -> AdapterDiscovery {
        discover_binary("hermes")
    }

    fn config_paths(&self) -> Vec<PathBuf> {
        vec![home_join(".hermes/config.yaml")]
    }

    fn read_profile(&self) -> anyhow::Result<RuntimeProfile> {
        let path = home_join(".hermes/config.yaml");
        if !path.exists() {
            return Ok(RuntimeProfile {
                gateway_url: None,
                key_source: None,
            });
        }

        let raw = fs::read_to_string(&path)?;
        let value: serde_yaml::Value = serde_yaml::from_str(&raw)?;
        let gateway_url = value
            .get("model")
            .and_then(|model| model.get("base_url"))
            .and_then(|v| v.as_str())
            .filter(|url| !url.is_empty())
            .map(str::to_string);

        Ok(RuntimeProfile {
            gateway_url,
            key_source: Some(path.display().to_string()),
        })
    }
}
