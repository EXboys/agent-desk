pub mod hermes;

pub use hermes::{apply_hermes_playbook, suggest_hermes_repairs};

use crate::probe::RuntimeProbeReport;
use crate::repair::SuggestedRepair;

pub fn suggest_runtime_repairs(runtime_id: &str, probe: &RuntimeProbeReport) -> Vec<SuggestedRepair> {
    match runtime_id {
        "hermes" => suggest_hermes_repairs(probe),
        _ => Vec::new(),
    }
}
