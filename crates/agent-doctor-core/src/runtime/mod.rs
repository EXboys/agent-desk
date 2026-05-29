mod registry;

pub(crate) use registry::ConfigFormat;

pub use registry::{
    adapter_by_id, all_adapters, all_runtime_ids, apply_runtime_playbook, descriptor_by_id,
    run_runtime_lifecycle, runtime_supports_lifecycle, runtime_supports_playbook,
    suggest_runtime_repairs, RuntimeDescriptor, RuntimeLifecycleAction, RuntimeProbeSpec,
};
