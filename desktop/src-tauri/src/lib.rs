use agent_doctor_core::{
    apply_profile_model, load_profiles, run_doctor, set_runtime_model, use_profile, ApplyReport,
    DoctorReport, HermesAdapter, HermesProfilePreset, HermesSettings, ProfilesDocument,
    RuntimeModelPreset, UseProfileReport,
};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};
use tauri::{Emitter, Manager};

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
            apply_profile_model_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
