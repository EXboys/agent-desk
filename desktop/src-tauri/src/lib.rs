use agent_doctor_core::{
    apply_profile_model, build_repair_preview_from_bundle, execute_repair, list_runtime_backup_ids,
    load_profiles, probe_runtime, restore_runtime_backup, run_doctor, runtime_supports_playbook,
    set_runtime_model, suggest_runtime_repairs, use_profile, ApplyReport, DoctorReport, HermesAdapter,
    HermesProfilePreset, HermesSettings, ProbeStatus, ProfilesDocument, RepairExecuteOptions,
    RepairExecuteReport, RestoreReport, RuntimeModelPreset, RuntimeProbeReport, UseProfileReport,
};
use serde::Serialize;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};
use tauri::{Emitter, Manager};
use tauri_plugin_opener::OpenerExt;

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn publish_doctor_report(app: &tauri::AppHandle, report: &DoctorReport) {
    show_main_window(app);
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.emit("doctor-report", report);
    }
}

#[tauri::command]
fn run_doctor_command() -> DoctorReport {
    run_doctor()
}

#[tauri::command]
fn list_profiles_command() -> ProfilesDocument {
    load_profiles().unwrap_or(ProfilesDocument {
        active: None,
        profiles: Default::default(),
    })
}

#[tauri::command]
fn use_profile_command(name: String) -> Result<UseProfileReport, String> {
    use_profile(&name).map_err(|error| error.to_string())
}

#[tauri::command]
fn get_hermes_model_command() -> Result<HermesSettings, String> {
    HermesAdapter
        .read_settings()
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn set_hermes_model_command(
    provider: String,
    model: String,
    base_url: String,
    api_key: Option<String>,
) -> Result<ApplyReport, String> {
    set_runtime_model(
        "hermes",
        RuntimeModelPreset {
            provider,
            model,
            base_url,
        },
        api_key.as_deref(),
    )
    .map_err(|error| error.to_string())
}

#[tauri::command]
fn apply_profile_model_command(
    profile: String,
    provider: String,
    model: String,
    base_url: String,
) -> Result<ApplyReport, String> {
    apply_profile_model(
        &profile,
        HermesProfilePreset {
            provider,
            model,
            base_url,
        },
    )
    .map_err(|error| error.to_string())
}

#[tauri::command]
fn run_repair_preview_command(runtime: String) -> Result<RepairPreviewResponse, String> {
    let report = probe_runtime(&runtime).map_err(|error| error.to_string())?;
    Ok(build_repair_preview_response(report, None))
}

#[tauri::command]
fn run_repair_execute_command(runtime: String) -> Result<RepairPreviewResponse, String> {
    let result = execute_repair(
        &runtime,
        &RepairExecuteOptions {
            apply_confirmed_writes: true,
        },
    )
    .map_err(|error| error.to_string())?;
    let execute = RepairExecuteSummary::from(&result);
    Ok(build_repair_preview_response(
        result.after_probe,
        Some(execute),
    ))
}

#[tauri::command]
fn run_repair_rollback_command(
    runtime: String,
    backup: Option<String>,
) -> Result<RestoreSummary, String> {
    let report =
        restore_runtime_backup(&runtime, backup.as_deref()).map_err(|error| error.to_string())?;
    Ok(RestoreSummary::from(&report))
}

#[tauri::command]
fn open_path_command(path: String, app: tauri::AppHandle) -> Result<(), String> {
    app.opener()
        .open_path(path, None::<&str>)
        .map_err(|error| error.to_string())
}

fn build_repair_preview_response(
    report: RuntimeProbeReport,
    last_execute: Option<RepairExecuteSummary>,
) -> RepairPreviewResponse {
    let plan = build_repair_preview_from_bundle(report.to_diagnostic_bundle());
    let suggested = suggest_runtime_repairs(&report.runtime_id, &report);
    let can_apply_repair = runtime_supports_playbook(&report.runtime_id)
        || suggested.iter().any(|item| item.auto_fixable);
    let backup_ids = list_runtime_backup_ids(&report.runtime_id).unwrap_or_default();
    let mut summary = RepairPreviewSummary::default();
    let checks = report
        .checks
        .into_iter()
        .map(|check| {
            match check.status {
                ProbeStatus::Pass => summary.pass += 1,
                ProbeStatus::Warn => summary.warn += 1,
                ProbeStatus::Fail => summary.fail += 1,
                ProbeStatus::NotApplicable => summary.not_applicable += 1,
                ProbeStatus::NotChecked => summary.not_checked += 1,
            }
            RepairPreviewCheck {
                title: check.title,
                status: probe_status_label(check.status).to_string(),
                message: check.message,
                details: check.details,
            }
        })
        .collect();

    RepairPreviewResponse {
        runtime_id: report.runtime_id,
        display_name: report.display_name,
        summary,
        checks,
        plan_summary: plan.summary,
        suggested_repairs: suggested
            .into_iter()
            .map(|item| SuggestedRepairItem {
                id: item.id,
                title: item.title,
                description: item.description,
                auto_fixable: item.auto_fixable,
            })
            .collect(),
        can_apply_repair,
        backup_ids,
        last_execute,
    }
}

#[derive(Debug, Default, Serialize)]
struct RepairPreviewSummary {
    pass: usize,
    warn: usize,
    fail: usize,
    not_applicable: usize,
    not_checked: usize,
}

#[derive(Debug, Serialize)]
struct RepairPreviewCheck {
    title: String,
    status: String,
    message: String,
    details: Vec<String>,
}

#[derive(Debug, Serialize)]
struct SuggestedRepairItem {
    id: String,
    title: String,
    description: String,
    auto_fixable: bool,
}

#[derive(Debug, Serialize)]
struct SkippedRepairItem {
    id: String,
    reason: String,
}

#[derive(Debug, Serialize)]
struct RepairExecuteSummary {
    backup_id: String,
    backup_root: String,
    executed: Vec<String>,
    skipped: Vec<SkippedRepairItem>,
    verification_summary: String,
    rollback_hint: String,
    guide_path: Option<String>,
}

impl From<&RepairExecuteReport> for RepairExecuteSummary {
    fn from(report: &RepairExecuteReport) -> Self {
        Self {
            backup_id: report.backup.id.clone(),
            backup_root: report.backup.root.clone(),
            executed: report.executed_action_ids.clone(),
            skipped: report
                .skipped_actions
                .iter()
                .map(|item| SkippedRepairItem {
                    id: item.id.clone(),
                    reason: item.reason.clone(),
                })
                .collect(),
            verification_summary: report.audit.verification_summary.clone(),
            rollback_hint: report.audit.rollback_hint.clone(),
            guide_path: report.guide_path.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
struct RestoreSummary {
    backup_id: String,
    backup_root: String,
    restored_files: Vec<String>,
}

impl From<&RestoreReport> for RestoreSummary {
    fn from(report: &RestoreReport) -> Self {
        Self {
            backup_id: report.backup_id.clone(),
            backup_root: report.backup_root.clone(),
            restored_files: report.restored_files.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
struct RepairPreviewResponse {
    runtime_id: String,
    display_name: String,
    summary: RepairPreviewSummary,
    checks: Vec<RepairPreviewCheck>,
    plan_summary: String,
    suggested_repairs: Vec<SuggestedRepairItem>,
    can_apply_repair: bool,
    backup_ids: Vec<String>,
    last_execute: Option<RepairExecuteSummary>,
}

fn probe_status_label(status: ProbeStatus) -> &'static str {
    match status {
        ProbeStatus::Pass => "pass",
        ProbeStatus::Warn => "warn",
        ProbeStatus::Fail => "fail",
        ProbeStatus::NotApplicable => "n/a",
        ProbeStatus::NotChecked => "not checked",
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            use tauri::menu::{Menu, MenuItem};
            use tauri::tray::TrayIconBuilder;

            let show_i = MenuItem::with_id(app, "show", "Show Agent Doctor", true, None::<&str>)?;
            let doctor_i = MenuItem::with_id(app, "doctor", "Run doctor", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &doctor_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .tooltip("Agent Doctor")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => show_main_window(app),
                    "doctor" => {
                        let report = run_doctor();
                        publish_doctor_report(app, &report);
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_main_window(tray.app_handle());
                    }
                })
                .build(app)?;

            show_main_window(app.handle());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            run_doctor_command,
            list_profiles_command,
            use_profile_command,
            get_hermes_model_command,
            set_hermes_model_command,
            apply_profile_model_command,
            run_repair_preview_command,
            run_repair_execute_command,
            run_repair_rollback_command,
            open_path_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
