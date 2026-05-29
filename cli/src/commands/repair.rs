use agent_doctor_core::{
    build_repair_preview_from_bundle, execute_repair, list_runtime_backup_ids,
    probe_health_summary, probe_runtime, restore_runtime_backup, runtime_supports_playbook,
    suggest_runtime_repairs, ProbeStatus, RepairExecuteOptions, RepairRisk,
};
use anyhow::Result;

pub fn run(
    runtime: &str,
    apply: bool,
    rollback: bool,
    backup: Option<&str>,
    json: bool,
) -> Result<()> {
    if rollback {
        return run_rollback(runtime, backup, json);
    }
    if apply {
        return run_execute(runtime, json);
    }
    run_preview(runtime)
}

fn run_preview(runtime: &str) -> Result<()> {
    let report = probe_runtime(runtime)?;
    let plan = build_repair_preview_from_bundle(report.to_diagnostic_bundle());

    println!("Agent Doctor — runtime probe and safe repair preview\n");
    println!("Runtime: {}", plan.runtime_id);
    println!("Summary: {}\n", plan.summary);

    if runtime_supports_playbook(runtime) {
        let backups = list_runtime_backup_ids(runtime)?;
        if !backups.is_empty() {
            println!("Recent backups (for --rollback):");
            for id in backups.iter().take(5) {
                println!("  - {id}");
            }
            println!();
        }
    }

    let suggested = suggest_runtime_repairs(runtime, &report);
    if !suggested.is_empty() {
        println!("Suggested fixes (use --apply to run auto-fixable items):");
        for item in &suggested {
            let mode = if item.auto_fixable {
                "auto-fixable"
            } else {
                "manual"
            };
            println!("  - {} [{mode}]", item.title);
            println!("    {}", item.description);
        }
        println!();
    }

    println!("Rule-based probe checks:");
    for check in &report.checks {
        println!(
            "  - {}: {} — {}",
            check.title,
            status_label(check.status),
            check.message
        );
        for detail in &check.details {
            println!("    detail: {detail}");
        }
    }

    println!("\nRedacted diagnostic facts:");
    for fact in &plan.redacted_facts {
        let marker = if fact.redacted { "redacted" } else { "visible" };
        println!("  - {}: {} ({marker})", fact.key, fact.value);
    }

    println!("\nPlanned repair phases:");
    for action in &plan.actions {
        let risk = match action.risk {
            RepairRisk::Low => "low",
            RepairRisk::Medium => "medium",
            RepairRisk::High => "high",
        };
        let confirmation = if action.requires_confirmation {
            "requires confirmation"
        } else {
            "automatic"
        };
        println!("  - {} [{} · {}]", action.title, risk, confirmation);
        println!("    {}", action.description);
        if !action.touches.is_empty() {
            println!("    touches: {}", action.touches.join(", "));
        }
    }

    println!(
        "\nNo files were modified. Run with --apply to execute backup, rule fixes, verification, and audit."
    );
    println!("Rollback: agent-doctor repair {runtime} --rollback [--backup <id>]");
    Ok(())
}

fn run_execute(runtime: &str, json: bool) -> Result<()> {
    let report = execute_repair(
        runtime,
        &RepairExecuteOptions {
            apply_confirmed_writes: true,
        },
    )?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("Agent Doctor — repair execute\n");
    println!("Runtime: {}", report.runtime_id);
    println!(
        "Backup: {} ({} file(s))",
        report.backup.root,
        report.backup.files.len()
    );
    for file in &report.backup.files {
        println!("  - {} -> {}", file.original_path, file.snapshot_path);
    }

    println!(
        "\nHealth: {} -> {}",
        probe_health_summary(&report.before_probe),
        probe_health_summary(&report.after_probe)
    );

    println!("\nExecuted actions:");
    if report.executed_action_ids.is_empty() {
        println!("  (none)");
    } else {
        for id in &report.executed_action_ids {
            println!("  - {id}");
        }
    }

    if report.skipped_actions.is_empty() {
        println!("\nNo rule fixes were required (config backup completed).");
    } else {
        println!("\nSkipped actions:");
        for item in &report.skipped_actions {
            println!("  - {}: {}", item.id, item.reason);
        }
    }

    if let Some(guide) = &report.guide_path {
        println!("\nAPI key guide: {guide}");
        println!("  Open this file for setup steps. Secrets are not auto-filled.");
    }

    println!("\nAudit: {}", report.audit.id);
    println!("  verification: {}", report.audit.verification_summary);
    println!(
        "  rollback: agent-doctor repair {runtime} --rollback --backup {}",
        report.backup.id
    );
    Ok(())
}

fn run_rollback(runtime: &str, backup: Option<&str>, json: bool) -> Result<()> {
    let report = restore_runtime_backup(runtime, backup)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("Agent Doctor — restore from backup\n");
    println!("Runtime: {}", report.runtime_id);
    println!("Backup: {} ({})", report.backup_id, report.backup_root);
    println!("\nRestored files:");
    for path in &report.restored_files {
        println!("  - {path}");
    }
    Ok(())
}

fn status_label(status: ProbeStatus) -> &'static str {
    match status {
        ProbeStatus::Pass => "pass",
        ProbeStatus::Warn => "warn",
        ProbeStatus::Fail => "fail",
        ProbeStatus::NotApplicable => "n/a",
        ProbeStatus::NotChecked => "not checked",
    }
}
