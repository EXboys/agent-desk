use agent_doctor_core::{build_repair_preview_from_bundle, probe_runtime, ProbeStatus, RepairRisk};
use anyhow::Result;

pub fn run(runtime: &str) -> Result<()> {
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
        "\nNo files were read or modified. Real repair execution will require a backup snapshot and explicit confirmation."
    );
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
