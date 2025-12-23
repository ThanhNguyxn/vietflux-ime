//! IME integration for Tauri
//! Delegates commands to the global keyboard hook engine

use crate::keyboard;
use vietflux_core::shortcut::Shortcut;

#[tauri::command]
pub fn set_method(method: String) {
    keyboard::set_method(&method);
}

#[tauri::command]
pub fn get_method() -> String {
    keyboard::get_method()
}

#[tauri::command]
pub fn toggle() -> bool {
    keyboard::toggle_ime()
}

#[tauri::command]
pub fn is_enabled() -> bool {
    keyboard::is_enabled()
}

#[tauri::command]
pub fn clear() {
    keyboard::clear();
}

#[tauri::command]
pub fn set_options(auto_capitalize: bool, smart_quotes: bool, spell_check: bool) {
    keyboard::set_options(auto_capitalize, smart_quotes, spell_check);
}

#[tauri::command]
pub fn get_options() -> (bool, bool, bool) {
    keyboard::get_options()
}

// Shortcut Management Commands

#[tauri::command]
pub fn get_shortcuts() -> Vec<Shortcut> {
    keyboard::get_shortcuts()
}

#[tauri::command]
pub fn add_shortcut(trigger: String, expansion: String) -> Vec<Shortcut> {
    keyboard::add_shortcut(&trigger, &expansion);
    keyboard::get_shortcuts()
}

#[tauri::command]
pub fn remove_shortcut(trigger: String) -> Vec<Shortcut> {
    keyboard::remove_shortcut(&trigger);
    keyboard::get_shortcuts()
}

#[tauri::command]
pub fn toggle_shortcut(trigger: String) -> Vec<Shortcut> {
    keyboard::toggle_shortcut(&trigger);
    keyboard::get_shortcuts()
}
