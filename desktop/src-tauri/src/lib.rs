use agent_desk_core::{
    load_profiles, run_doctor, use_profile, DoctorReport, ProfilesDocument, UseProfileReport,
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            use tauri::menu::{Menu, MenuItem};
            use tauri::tray::TrayIconBuilder;

            let show_i = MenuItem::with_id(app, "show", "Show Agent Desk", true, None::<&str>)?;
            let doctor_i = MenuItem::with_id(app, "doctor", "Run doctor", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &doctor_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .tooltip("Agent Desk")
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
            use_profile_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
