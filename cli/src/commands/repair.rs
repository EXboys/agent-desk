use agent_doctor_core::{build_repair_preview, RepairRisk};
use anyhow::Result;

pub fn run(runtime: &str) -> Result<()> {
    let plan = build_repair_preview(runtime);

    println!("Agent Doctor — safe repair preview\n");
    println!("Runtime: {}", plan.runtime_id);
    println!("Summary: {}\n", plan.summary);

    println!("Redacted diagnostic facts:");
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
