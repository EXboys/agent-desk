use super::backends::{hermes_gateway_profiles, run_hermes, run_openclaw};
use super::WorkspaceEntry;
use crate::adapters::util::find_binary;

#[derive(Debug, Clone, serde::Serialize)]
pub struct GatewayRestartReport {
    pub runtime_id: &'static str,
    pub attempted: bool,
    pub success: bool,
    pub detail: String,
}

pub fn restart_workspace_gateways(entry: &WorkspaceEntry) -> Vec<GatewayRestartReport> {
    vec![restart_hermes_gateway(entry), restart_openclaw_gateway()]
}

fn restart_hermes_gateway(entry: &WorkspaceEntry) -> GatewayRestartReport {
    if find_binary("hermes").is_none() {
        return GatewayRestartReport {
            runtime_id: "hermes",
            attempted: false,
            success: false,
            detail: "hermes binary not found — restart gateway manually after switching profile"
                .into(),
        };
    }

    let running = hermes_gateway_profiles();
    if running.is_empty() {
        return GatewayRestartReport {
            runtime_id: "hermes",
            attempted: false,
            success: false,
            detail: "no Hermes gateway lock detected — start gateway under this profile if needed"
                .into(),
        };
    }

    if running
        .iter()
        .all(|profile| profile == &entry.hermes_profile)
    {
        return GatewayRestartReport {
            runtime_id: "hermes",
            attempted: false,
            success: true,
            detail: format!(
                "Hermes gateway already on profile '{}'",
                entry.hermes_profile
            ),
        };
    }

    if let Ok(capture) = run_hermes(&["gateway", "restart"]) {
        if capture.success {
            let aligned = hermes_gateway_profiles()
                .iter()
                .all(|profile| profile == &entry.hermes_profile);
            return GatewayRestartReport {
                runtime_id: "hermes",
                attempted: true,
                success: aligned,
                detail: if aligned {
                    format!(
                        "Hermes gateway restarted for profile '{}'",
                        entry.hermes_profile
                    )
                } else {
                    format!(
                        "`hermes gateway restart` ran but gateway still reports: {}",
                        running.join(", ")
                    )
                },
            };
        }
    }

    let stop_ok = run_hermes(&["gateway", "stop"])
        .map(|capture| capture.success)
        .unwrap_or(false);
    let start_ok = run_hermes(&["gateway", "start"])
        .map(|capture| capture.success)
        .unwrap_or(false);
    let aligned = hermes_gateway_profiles()
        .iter()
        .all(|profile| profile == &entry.hermes_profile);

    GatewayRestartReport {
        runtime_id: "hermes",
        attempted: stop_ok || start_ok,
        success: aligned,
        detail: if aligned {
            "Hermes gateway stop/start completed".into()
        } else {
            format!(
                "could not align Hermes gateway to profile '{}' — try: hermes profile use {profile} && hermes gateway restart",
                entry.hermes_profile,
                profile = entry.hermes_profile
            )
        },
    }
}

fn restart_openclaw_gateway() -> GatewayRestartReport {
    if find_binary("openclaw").is_none() {
        return GatewayRestartReport {
            runtime_id: "openclaw",
            attempted: false,
            success: false,
            detail: "openclaw binary not found".into(),
        };
    }

    for args in [["gateway", "restart"], ["gateway", "reload"]] {
        if let Ok(capture) = run_openclaw(&args) {
            if capture.success {
                return GatewayRestartReport {
                    runtime_id: "openclaw",
                    attempted: true,
                    success: true,
                    detail: format!("ran `openclaw {}`", args.join(" ")),
                };
            }
        }
    }

    GatewayRestartReport {
        runtime_id: "openclaw",
        attempted: true,
        success: false,
        detail: "OpenClaw gateway restart not confirmed — run `openclaw gateway restart` if routing is stale"
            .into(),
    }
}

pub fn gateway_restart_hint(entry: &WorkspaceEntry) -> Option<String> {
    let running = hermes_gateway_profiles();
    if running
        .iter()
        .any(|profile| profile != &entry.hermes_profile)
    {
        Some(format!(
            "Hermes gateway running under {} — rerun with --restart-gateways",
            running.join(", ")
        ))
    } else {
        None
    }
}
