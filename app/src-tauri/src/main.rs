#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    Manager,
};

use vietflux_app_lib::{ime, keyboard};

// Global tray reference for updates
static TRAY: Mutex<Option<TrayIcon>> = Mutex::new(None);

#[tauri::command]
fn quit_app(_app: tauri::AppHandle) {
    keyboard::stop_hook();
    std::process::exit(0);
}

#[tauri::command]
fn hide_window(window: tauri::Window) {
    window.hide().unwrap();
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Create system tray menu
            let quit = MenuItem::with_id(app, "quit", "Thoát", true, None::<&str>)?;
            let toggle =
                MenuItem::with_id(app, "toggle", "Bật/Tắt (Ctrl+Shift)", true, None::<&str>)?;
            let telex = MenuItem::with_id(app, "telex", "Telex", true, None::<&str>)?;
            let vni = MenuItem::with_id(app, "vni", "VNI", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&toggle, &telex, &vni, &quit])?;

            // Get icon from default window icon
            let icon = app.default_window_icon().unwrap().clone();

            // Create tray icon
            let tray = TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .tooltip("VietFlux IME - Telex")
                .on_menu_event(|_app, event| {
                    println!("Menu event: {:?}", event.id.as_ref());
                    match event.id.as_ref() {
                        "quit" => {
                            println!("Quit clicked");
                            keyboard::stop_hook();
                            std::process::exit(0);
                        }
                        "toggle" => {
                            println!("Toggle clicked");
                            let enabled = keyboard::toggle_ime();
                            println!("IME enabled: {}", enabled);
                            if let Ok(guard) = TRAY.lock() {
                                if let Some(ref tray) = *guard {
                                    let _ = tray.set_tooltip(if enabled {
                                        Some("VietFlux IME - Bật")
                                    } else {
                                        Some("VietFlux IME - Tắt")
                                    });
                                }
                            }
                        }
                        "telex" => {
                            println!("Telex clicked");
                            keyboard::set_method("telex");
                            if let Ok(guard) = TRAY.lock() {
                                if let Some(ref tray) = *guard {
                                    let _ = tray.set_tooltip(Some("VietFlux IME - Telex"));
                                }
                            }
                        }
                        "vni" => {
                            println!("VNI clicked");
                            keyboard::set_method("vni");
                            if let Ok(guard) = TRAY.lock() {
                                if let Some(ref tray) = *guard {
                                    let _ = tray.set_tooltip(Some("VietFlux IME - VNI"));
                                }
                            }
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        println!("Tray left clicked");
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // Store tray reference for updates
            *TRAY.lock().unwrap() = Some(tray);

            // Start keyboard hook
            keyboard::start_hook();
            println!("Keyboard hook started");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ime::set_method,
            ime::get_method,
            ime::toggle,
            ime::is_enabled,
            ime::clear,
            ime::set_options,
            ime::get_options,
            ime::get_shortcuts,
            ime::add_shortcut,
            ime::remove_shortcut,
            ime::toggle_shortcut,
            quit_app,
            hide_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
