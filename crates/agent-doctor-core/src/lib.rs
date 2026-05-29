pub mod adapter;
pub mod adapters;
pub mod doctor;
pub mod lifecycle;
pub mod presets;
pub mod probe;
pub mod profile;
pub mod repair;
pub mod runtime;

pub use adapter::{
    AdapterDiscovery, ApplyReport, RuntimeAdapter, RuntimeModelPreset, RuntimeModelState,
    RuntimeProfile,
};
pub use adapters::{CodexAdapter, HermesAdapter, HermesSettings, OpenClawAdapter};
pub use runtime::{adapter_by_id, all_adapters};
pub use doctor::{run_doctor, DoctorReport, RuntimeDoctorResult};
pub use lifecycle::{hermes_shell_command, run_hermes_lifecycle, HermesLifecycleAction};
pub use presets::{
    apply_profile_model, default_local_hermes_preset, default_work_models, effective_models,
    init_example_profiles, load_profiles, merge_builtin_profiles, profiles_path, set_runtime_model,
    show_config, use_profile, HermesProfilePreset, ProfileEntry, ProfilesDocument,
    UseProfileReport,
};
pub use probe::{
    probe_all_runtimes, probe_runtime, ProbeCheck, ProbeSeverity, ProbeStatus, RuntimeProbeReport,
};
pub use repair::{
    apply_hermes_playbook, build_repair_preview, build_repair_preview_from_bundle, execute_repair,
    list_runtime_backup_ids, probe_health_summary, restore_runtime_backup, suggest_hermes_repairs,
    AuditReport, BackupSnapshot, DiagnosticBundle, DiagnosticFact, PlaybookApplyResult,
    RedactedFact, RedactionPolicy, Redactor, RepairAction, RepairActionKind, RepairExecuteOptions,
    RepairExecuteReport, RepairPlan, RepairRisk, RestoreReport, SensitivityLevel,
    SkippedRepairAction, SnapshotFile, SuggestedRepair,
};
pub use runtime::{
    all_runtime_ids, apply_runtime_playbook, descriptor_by_id, run_runtime_lifecycle,
    runtime_supports_lifecycle, runtime_supports_playbook, suggest_runtime_repairs,
    RuntimeDescriptor, RuntimeLifecycleAction, RuntimeProbeSpec,
};
