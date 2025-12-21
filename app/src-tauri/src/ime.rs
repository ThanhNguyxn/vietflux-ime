//! IME integration for Tauri

use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Shortcut {
    pub trigger: String,
    pub expansion: String,
    pub enabled: bool,
}

// Simple IME state (will be replaced with core engine later)
pub struct ImeState {
    enabled: bool,
    method: String,
    buffer: String,
    shortcuts: Vec<Shortcut>,
}

impl Default for ImeState {
    fn default() -> Self {
        Self {
            enabled: true,
            method: "telex".to_string(),
            buffer: String::new(),
            shortcuts: vec![
                Shortcut { trigger: "vn".to_string(), expansion: "Việt Nam".to_string(), enabled: true },
                Shortcut { trigger: "hn".to_string(), expansion: "Hà Nội".to_string(), enabled: true },
                Shortcut { trigger: "hcm".to_string(), expansion: "Hồ Chí Minh".to_string(), enabled: true },
            ],
        }
    }
}

static IME: Mutex<ImeState> = Mutex::new(ImeState {
    enabled: true,
    method: String::new(),
    buffer: String::new(),
    shortcuts: Vec::new(), // Will be initialized properly via Default if we used Lazy, but here we use explicit init
});

// Initialize the static with default values properly
// In a real app we'd use lazy_static or OnceLock, but for this simple example let's just use a function to ensure defaults if empty
fn ensure_defaults(ime: &mut ImeState) {
    if ime.method.is_empty() {
        *ime = ImeState::default();
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProcessResult {
    pub action: String,
    pub output: String,
    pub backspace: usize,
}

#[tauri::command]
pub fn process_key(key: char, _shift: bool) -> ProcessResult {
    let mut ime = IME.lock().unwrap();
    ensure_defaults(&mut ime);
    
    if !ime.enabled {
        return ProcessResult {
            action: "passthrough".to_string(),
            output: key.to_string(),
            backspace: 0,
        };
    }
    
    // Check shortcuts first (simplified logic)
    // In a real engine, this would be part of the composition loop
    // Here we just check if the buffer + key matches a shortcut trigger
    let current_input = format!("{}{}", ime.buffer, key);
    for shortcut in &ime.shortcuts {
        if shortcut.enabled && shortcut.trigger == current_input {
            ime.buffer.clear();
            return ProcessResult {
                action: "commit".to_string(),
                output: shortcut.expansion.clone(),
                backspace: shortcut.trigger.len() - 1, // Backspace previous chars
            };
        }
    }
    
    // Simple Telex processing (placeholder - will use core engine)
    let output = match (ime.method.as_str(), key) {
        ("telex", 's') => {
            // Apply acute tone to last vowel in buffer
            ime.buffer.push(key);
            key.to_string()
        }
        _ => {
            ime.buffer.push(key);
            key.to_string()
        }
    };
    
    ProcessResult {
        action: "commit".to_string(),
        output,
        backspace: 0,
    }
}

#[tauri::command]
pub fn set_method(method: String) {
    let mut ime = IME.lock().unwrap();
    ensure_defaults(&mut ime);
    ime.method = method;
}

#[tauri::command]
pub fn get_method() -> String {
    let mut ime = IME.lock().unwrap();
    ensure_defaults(&mut ime);
    ime.method.clone()
}

#[tauri::command]
pub fn toggle() -> bool {
    let mut ime = IME.lock().unwrap();
    ensure_defaults(&mut ime);
    ime.enabled = !ime.enabled;
    ime.enabled
}

#[tauri::command]
pub fn clear() {
    let mut ime = IME.lock().unwrap();
    ime.buffer.clear();
}

// Shortcut Management Commands

#[tauri::command]
pub fn get_shortcuts() -> Vec<Shortcut> {
    let mut ime = IME.lock().unwrap();
    ensure_defaults(&mut ime);
    ime.shortcuts.clone()
}

#[tauri::command]
pub fn add_shortcut(trigger: String, expansion: String) -> Vec<Shortcut> {
    let mut ime = IME.lock().unwrap();
    ensure_defaults(&mut ime);
    ime.shortcuts.push(Shortcut {
        trigger,
        expansion,
        enabled: true,
    });
    ime.shortcuts.clone()
}

#[tauri::command]
pub fn remove_shortcut(index: usize) -> Vec<Shortcut> {
    let mut ime = IME.lock().unwrap();
    ensure_defaults(&mut ime);
    if index < ime.shortcuts.len() {
        ime.shortcuts.remove(index);
    }
    ime.shortcuts.clone()
}

#[tauri::command]
pub fn toggle_shortcut(index: usize) -> Vec<Shortcut> {
    let mut ime = IME.lock().unwrap();
    ensure_defaults(&mut ime);
    if index < ime.shortcuts.len() {
        ime.shortcuts[index].enabled = !ime.shortcuts[index].enabled;
    }
    ime.shortcuts.clone()
}
