pub mod adapter;
pub mod adapters;
pub mod doctor;
pub mod presets;
pub mod profile;

pub use adapter::{
    AdapterDiscovery, ApplyReport, RuntimeAdapter, RuntimeModelPreset, RuntimeModelState,
    RuntimeProfile,
};
pub use adapters::{adapter_by_id, all_adapters, HermesAdapter, HermesSettings};
pub use doctor::{run_doctor, DoctorReport, RuntimeDoctorResult};
pub use presets::{
    default_local_hermes_preset, init_example_profiles, load_profiles, merge_builtin_profiles,
    profiles_path, set_runtime_model, show_config, use_profile, HermesProfilePreset, ProfileEntry,
    ProfilesDocument, UseProfileReport,
};
