//! IME integration for Tauri

use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Simple IME state (will be replaced with core engine later)
pub struct ImeState {
    enabled: bool,
    method: String,
    buffer: String,
}

impl Default for ImeState {
    fn default() -> Self {
        Self {
            enabled: true,
            method: "telex".to_string(),
            buffer: String::new(),
        }
    }
}

static IME: Mutex<ImeState> = Mutex::new(ImeState {
    enabled: true,
    method: String::new(),
    buffer: String::new(),
});

#[derive(Serialize, Deserialize)]
pub struct ProcessResult {
    pub action: String,
    pub output: String,
    pub backspace: usize,
}

#[tauri::command]
pub fn process_key(key: char, shift: bool) -> ProcessResult {
    let mut ime = IME.lock().unwrap();
    
    if !ime.enabled {
        return ProcessResult {
            action: "passthrough".to_string(),
            output: key.to_string(),
            backspace: 0,
        };
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
    ime.method = method;
}

#[tauri::command]
pub fn get_method() -> String {
    let ime = IME.lock().unwrap();
    ime.method.clone()
}

#[tauri::command]
pub fn toggle() -> bool {
    let mut ime = IME.lock().unwrap();
    ime.enabled = !ime.enabled;
    ime.enabled
}

#[tauri::command]
pub fn clear() {
    let mut ime = IME.lock().unwrap();
    ime.buffer.clear();
}
