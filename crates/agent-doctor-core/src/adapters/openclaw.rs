use std::fs;
use std::path::PathBuf;

use crate::adapter::{AdapterDiscovery, RuntimeAdapter, RuntimeProfile};
use crate::adapters::util::{discover_binary, home_join};

pub struct OpenClawAdapter;

impl RuntimeAdapter for OpenClawAdapter {
    fn id(&self) -> &'static str {
        "openclaw"
    }

    fn display_name(&self) -> &'static str {
        "OpenClaw"
    }

    fn discover(&self) -> AdapterDiscovery {
        discover_binary("openclaw")
    }

    fn config_paths(&self) -> Vec<PathBuf> {
        vec![home_join(".openclaw/openclaw.json")]
    }

    fn read_profile(&self) -> anyhow::Result<RuntimeProfile> {
        let path = home_join(".openclaw/openclaw.json");
        if !path.exists() {
            return Ok(RuntimeProfile {
                gateway_url: None,
                key_source: None,
            });
        }

        let raw = fs::read_to_string(&path)?;
        let value: serde_json::Value = serde_json::from_str(&raw)?;
        let gateway_url = value
            .pointer("/gateway/url")
            .or_else(|| value.pointer("/evotown/url"))
            .and_then(|v| v.as_str())
            .map(str::to_string);

        Ok(RuntimeProfile {
            gateway_url,
            key_source: Some(format!("{}", path.display())),
        })
    }
}
