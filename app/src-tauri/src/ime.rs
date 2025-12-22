//! IME integration for Tauri

use std::sync::Mutex;
use vietflux_core::{shortcut::Shortcut, Engine};

// Global IME Engine instance
static ENGINE: Mutex<Option<Engine>> = Mutex::new(None);

// Helper to get engine instance, initializing if needed
fn with_engine<F, R>(f: F) -> R
where
    F: FnOnce(&mut Engine) -> R,
{
    let mut engine_guard = ENGINE.lock().unwrap();
    if engine_guard.is_none() {
        *engine_guard = Some(Engine::new());
    }
    f(engine_guard.as_mut().unwrap())
}

#[derive(serde::Serialize)]
pub struct ProcessResult {
    pub action: String,
    pub output: String,
    pub backspace: usize,
}

#[tauri::command]
pub fn process_key(key: char, _shift: bool) -> ProcessResult {
    with_engine(|engine| {
        let result = engine.process_key(key, _shift);

        let action_str = match result.action {
            vietflux_core::engine::Action::Commit => "commit",
            vietflux_core::engine::Action::Update => "update",
            vietflux_core::engine::Action::Passthrough => "passthrough",
            vietflux_core::engine::Action::Restore => "restore",
        };

        ProcessResult {
            action: action_str.to_string(),
            output: result.output,
            backspace: result.backspace,
        }
    })
}

#[tauri::command]
pub fn set_method(method: String) {
    with_engine(|engine| {
        engine.set_method(&method);
    })
}

#[tauri::command]
pub fn get_method() -> String {
    with_engine(|engine| engine.get_method().to_string())
}

#[tauri::command]
pub fn toggle() -> bool {
    with_engine(|engine| {
        engine.toggle();
        engine.is_enabled()
    })
}

#[tauri::command]
pub fn clear() {
    with_engine(|engine| {
        engine.clear();
    })
}

// Shortcut Management Commands

#[tauri::command]
pub fn get_shortcuts() -> Vec<Shortcut> {
    with_engine(|engine| engine.get_shortcuts())
}

#[tauri::command]
pub fn add_shortcut(trigger: String, expansion: String) -> Vec<Shortcut> {
    with_engine(|engine| {
        engine.add_shortcut(&trigger, &expansion);
        engine.get_shortcuts()
    })
}

#[tauri::command]
pub fn remove_shortcut(trigger: String) -> Vec<Shortcut> {
    with_engine(|engine| {
        engine.remove_shortcut(&trigger);
        engine.get_shortcuts()
    })
}

#[tauri::command]
pub fn toggle_shortcut(trigger: String) -> Vec<Shortcut> {
    with_engine(|engine| {
        engine.toggle_shortcut(&trigger);
        engine.get_shortcuts()
    })
}
