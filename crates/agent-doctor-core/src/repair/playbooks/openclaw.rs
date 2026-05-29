use crate::lifecycle::{run_openclaw_lifecycle, OpenClawLifecycleAction};
use crate::probe::{ProbeStatus, RuntimeProbeReport};
use crate::repair::{SkippedRepairAction, SuggestedRepair};

use super::should_run;
use super::PlaybookApplyResult;

pub fn suggest_openclaw_repairs(probe: &RuntimeProbeReport) -> Vec<SuggestedRepair> {
    let mut items = Vec::new();

    for check in &probe.checks {
        if check.id == "binary.exists" && check.status == ProbeStatus::Fail {
            items.push(SuggestedRepair {
                id: "fix-openclaw-install".to_string(),
                title: "Install OpenClaw".to_string(),
                description: "Run the official OpenClaw installer (openclaw.ai/install.sh) \
                    with --no-onboard. Requires network access."
                    .to_string(),
                auto_fixable: true,
            });
        }
    }

    items
}

pub fn apply_openclaw_playbook(probe: &RuntimeProbeReport) -> anyhow::Result<PlaybookApplyResult> {
    apply_openclaw_playbook_filtered(probe, None)
}

pub fn apply_openclaw_playbook_filtered(
    probe: &RuntimeProbeReport,
    only_ids: Option<&[String]>,
) -> anyhow::Result<PlaybookApplyResult> {
    let mut result = PlaybookApplyResult::default();

    if should_run("fix-openclaw-install", only_ids) && openclaw_needs_install(probe) {
        match run_openclaw_lifecycle(OpenClawLifecycleAction::Install) {
            Ok(()) => result.executed.push("fix-openclaw-install".to_string()),
            Err(error) => result.skipped.push(SkippedRepairAction {
                id: "fix-openclaw-install".to_string(),
                reason: error.to_string(),
            }),
        }
    }

    Ok(result)
}

fn openclaw_needs_install(probe: &RuntimeProbeReport) -> bool {
    probe
        .checks
        .iter()
        .any(|check| check.id == "binary.exists" && check.status == ProbeStatus::Fail)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probe::{ProbeCheck, ProbeSeverity};

    #[test]
    fn suggest_install_when_binary_missing() {
        let probe = RuntimeProbeReport {
            runtime_id: "openclaw".to_string(),
            display_name: "OpenClaw".to_string(),
            binary_name: "openclaw".to_string(),
            checks: vec![ProbeCheck::new(
                "binary.exists",
                "Binary on PATH",
                ProbeStatus::Fail,
                ProbeSeverity::Error,
                "openclaw not found",
                crate::repair::SensitivityLevel::Public,
            )],
            facts: Vec::new(),
        };

        let items = suggest_openclaw_repairs(&probe);
        assert!(items.iter().any(|item| item.id == "fix-openclaw-install"));
    }
}
