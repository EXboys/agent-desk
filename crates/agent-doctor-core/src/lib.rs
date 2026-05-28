pub mod adapter;
pub mod adapters;
pub mod doctor;
pub mod presets;
pub mod probe;
pub mod profile;
pub mod repair;

pub use adapter::{
    AdapterDiscovery, ApplyReport, RuntimeAdapter, RuntimeModelPreset, RuntimeModelState,
    RuntimeProfile,
};
pub use adapters::{adapter_by_id, all_adapters, CodexAdapter, HermesAdapter, HermesSettings};
pub use doctor::{run_doctor, DoctorReport, RuntimeDoctorResult};
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
    build_repair_preview, build_repair_preview_from_bundle, execute_repair,
    probe_health_summary, suggest_hermes_repairs, suggest_runtime_repairs, AuditReport,
    BackupSnapshot, DiagnosticBundle,
    DiagnosticFact, RedactedFact, RedactionPolicy, Redactor, RepairAction, RepairActionKind,
    RepairExecuteOptions, RepairExecuteReport, RepairPlan, RepairRisk, SensitivityLevel,
    SkippedRepairAction, SnapshotFile, SuggestedRepair,
};
