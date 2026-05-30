use anyhow::{Context, Result};

use super::backends::{bind_claude_code, bind_codex, bind_hermes, bind_openclaw};
use super::gateway::restart_workspace_gateways;
use super::snapshot::{apply_workspace_snapshot, save_workspace_snapshot};
use super::{
    load_workspaces, save_workspaces, workspace_data_root, workspace_doctor, write_active_env,
    WorkspaceCheckStatus, WorkspaceDoctorReport, WorkspaceEntry,
};

#[derive(Debug, Clone, serde::Serialize)]
pub struct WorkspaceFixAction {
    pub id: String,
    pub title: String,
    pub applied: bool,
    pub detail: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct WorkspaceFixReport {
    pub active: Option<String>,
    pub actions: Vec<WorkspaceFixAction>,
}

pub fn workspace_fix(dry_run: bool, restart_gateways: bool) -> Result<WorkspaceFixReport> {
    let doc = load_workspaces()?;
    let Some(active_name) = doc.active.clone() else {
        return Ok(WorkspaceFixReport {
            active: None,
            actions: vec![WorkspaceFixAction {
                id: "workspace.active.missing".into(),
                title: "No active workspace".into(),
                applied: false,
                detail: "Run `agent-doctor workspace init` then `workspace use <name>`.".into(),
            }],
        });
    };

    let Some(entry) = doc.workspaces.get(&active_name).cloned() else {
        return Ok(WorkspaceFixReport {
            active: Some(active_name),
            actions: vec![WorkspaceFixAction {
                id: "workspace.active.invalid".into(),
                title: "Active workspace entry missing".into(),
                applied: false,
                detail: "Re-register or pick a valid workspace with `workspace use`.".into(),
            }],
        });
    };

    let doctor = workspace_doctor()?;
    let mut actions = plan_fixes(&active_name, &entry, &doctor);

    if !dry_run {
        for action in &mut actions {
            if action.applied {
                continue;
            }
            match action.id.as_str() {
                "workspace.hermes.profile" => {
                    bind_hermes(&entry.hermes_profile, &entry.path)?;
                    action.applied = true;
                    action.detail = format!("Activated Hermes profile '{}'", entry.hermes_profile);
                }
                "workspace.openclaw.workspace" => {
                    bind_openclaw(&entry.openclaw_agent_id, &entry.openclaw_workspace)?;
                    action.applied = true;
                    action.detail = format!(
                        "Updated openclaw.json workspace for agent '{}'",
                        entry.openclaw_agent_id
                    );
                }
                "workspace.codex.home" | "workspace.codex.global_memory" => {
                    write_active_env(&active_name, &entry)?;
                    bind_codex(&entry.codex_home)?;
                    action.applied = true;
                    action.detail = format!(
                        "Refreshed active-workspace.env (CODEX_HOME={})",
                        entry.codex_home.display()
                    );
                }
                "workspace.claude.project_mcp" => {
                    let data_root = workspace_data_root(&active_name)?;
                    let report = apply_workspace_snapshot(&entry, &data_root)?;
                    if report.mcp_applied {
                        action.applied = true;
                        action.detail = "Restored project .mcp.json from workspace snapshot".into();
                    } else {
                        bind_claude_code(&entry.path)?;
                        save_workspace_snapshot(&entry, &data_root)?;
                        action.applied = true;
                        action.detail =
                            "Ensured .claude/ scaffold and refreshed MCP snapshot template".into();
                    }
                }
                "workspace.hermes.gateway_mismatch" if restart_gateways => {
                    let reports = restart_workspace_gateways(&entry);
                    action.applied = reports.iter().any(|report| report.success);
                    action.detail = reports
                        .into_iter()
                        .map(|report| format!("{}: {}", report.runtime_id, report.detail))
                        .collect::<Vec<_>>()
                        .join("; ");
                }
                _ => {}
            }
        }
    }

    Ok(WorkspaceFixReport {
        active: Some(active_name),
        actions,
    })
}

fn plan_fixes(
    active_name: &str,
    entry: &WorkspaceEntry,
    doctor: &WorkspaceDoctorReport,
) -> Vec<WorkspaceFixAction> {
    let mut actions = Vec::new();

    for check in &doctor.checks {
        if check.status == WorkspaceCheckStatus::Pass {
            continue;
        }

        let fixable = matches!(
            check.id.as_str(),
            "workspace.hermes.profile"
                | "workspace.openclaw.workspace"
                | "workspace.codex.home"
                | "workspace.codex.global_memory"
                | "workspace.claude.project_mcp"
                | "workspace.hermes.gateway_mismatch"
        );

        if !fixable {
            continue;
        }

        actions.push(WorkspaceFixAction {
            id: check.id.clone(),
            title: check.title.clone(),
            applied: false,
            detail: if check.id == "workspace.claude.project_mcp" {
                format!(
                    "Will restore .mcp.json from ~/.config/agent-doctor/workspaces/{active_name}/snapshots/"
                )
            } else if check.id == "workspace.hermes.gateway_mismatch" {
                "Will attempt Hermes/OpenClaw gateway restart (pass --restart-gateways)".into()
            } else {
                check.detail.clone()
            },
        });
    }

    if actions.is_empty() {
        actions.push(WorkspaceFixAction {
            id: "workspace.fix.nothing".into(),
            title: "No auto-fixable issues".into(),
            applied: false,
            detail: "Run `workspace doctor` for manual hints (cwd mismatch, gateway restart, global MCP)."
                .into(),
        });
    }

    let _ = entry;
    actions
}

pub fn remove_workspace(name: &str, purge_data: bool) -> Result<()> {
    let mut doc = load_workspaces()?;
    if !doc.workspaces.contains_key(name) {
        anyhow::bail!("workspace '{name}' not found");
    }

    doc.workspaces.remove(name);
    if doc.active.as_deref() == Some(name) {
        doc.active = doc.workspaces.keys().next().cloned();
    }
    save_workspaces(&doc)?;

    if purge_data {
        let data_root = workspace_data_root(name)?;
        if data_root.exists() {
            std::fs::remove_dir_all(&data_root)
                .with_context(|| format!("purge {}", data_root.display()))?;
        }
    }

    Ok(())
}
