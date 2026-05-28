use agent_doctor_core::{
    build_repair_preview_from_bundle, execute_repair, probe_health_summary, probe_runtime,
    ProbeStatus, RepairExecuteOptions, RepairRisk,
};
use anyhow::Result;

pub fn run(runtime: &str, apply: bool, json: bool) -> Result<()> {
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
        "\nNo files were modified. Run with --apply to execute backup, typed actions, verification, and audit."
    );
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
    println!("Backup: {} ({} file(s))", report.backup.root, report.backup.files.len());
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

    println!("\nAudit: {}", report.audit.id);
    println!("  verification: {}", report.audit.verification_summary);
    println!("  rollback: {}", report.audit.rollback_hint);
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
