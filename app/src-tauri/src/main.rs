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
                        // Toggle IME
                        println!("Toggle IME");
                    }
                    "telex" => {
                        println!("Switch to Telex");
                    }
                    "vni" => {
                        println!("Switch to VNI");
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
