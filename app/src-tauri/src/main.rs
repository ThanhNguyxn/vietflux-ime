#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime,
};

mod ime;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Create system tray menu
            let quit = MenuItem::with_id(app, "quit", "Thoát", true, None::<&str>)?;
            let toggle = MenuItem::with_id(app, "toggle", "Bật/Tắt (Ctrl+Shift)", true, None::<&str>)?;
            let telex = MenuItem::with_id(app, "telex", "Telex", true, None::<&str>)?;
            let vni = MenuItem::with_id(app, "vni", "VNI", true, None::<&str>)?;
            
            let menu = Menu::with_items(app, &[&toggle, &telex, &vni, &quit])?;

            // Create tray icon
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("VietFlux IME - Telex")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "toggle" => {
                        let enabled = ime::toggle();
                        let tray = app.tray_by_id("main").unwrap();
                        let _ = tray.set_tooltip(if enabled { "VietFlux IME - Bật" } else { "VietFlux IME - Tắt" });
                        // In a real app we would also change the icon here
                    }
                    "telex" => {
                        ime::set_method("telex".to_string());
                        let tray = app.tray_by_id("main").unwrap();
                        let _ = tray.set_tooltip("VietFlux IME - Telex");
                    }
                    "vni" => {
                        ime::set_method("vni".to_string());
                        let tray = app.tray_by_id("main").unwrap();
                        let _ = tray.set_tooltip("VietFlux IME - VNI");
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
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ime::process_key,
            ime::set_method,
            ime::get_method,
            ime::toggle,
            ime::clear,
            ime::get_shortcuts,
            ime::add_shortcut,
            ime::remove_shortcut,
            ime::toggle_shortcut,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
