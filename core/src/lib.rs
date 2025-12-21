//! VietFlux IME Core Engine
//!
//! High-performance Vietnamese Input Method Engine with WebAssembly support.
//! Inspired by UniKey and gonhanh.org with modern Rust implementation.
//!
//! # Features
//! - Telex and VNI input methods
//! - Fast syllable validation
//! - Smart diacritic placement
//! - Zero-copy buffer management
//!
//! # Usage (JavaScript/WASM)
//! ```javascript
//! import init, { VietFlux } from 'vietflux-core';
//! 
//! await init();
//! const ime = new VietFlux();
//! ime.set_method('telex');
//! 
//! const result = ime.process_key('a');
//! console.log(result.output); // 'a'
//! 
//! const result2 = ime.process_key('a');
//! console.log(result2.output); // 'â' (aa -> â in Telex)
//! ```

pub mod buffer;
pub mod chars;
pub mod engine;
pub mod methods;
pub mod shortcuts;
pub mod transform;
pub mod validation;

use wasm_bindgen::prelude::*;
use engine::Engine;

/// Main VietFlux IME instance exposed to JavaScript
#[wasm_bindgen]
pub struct VietFlux {
    engine: Engine,
}

#[wasm_bindgen]
impl VietFlux {
    /// Create a new VietFlux IME instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
        }
    }

    /// Set input method: "telex" or "vni"
    #[wasm_bindgen]
    pub fn set_method(&mut self, method: &str) {
        self.engine.set_method(method);
    }

    /// Get current input method name
    #[wasm_bindgen]
    pub fn get_method(&self) -> String {
        self.engine.get_method().to_string()
    }

    /// Process a key press and return the result
    /// Returns JSON: { "action": "commit"|"update"|"passthrough", "output": "...", "backspace": 0 }
    #[wasm_bindgen]
    pub fn process_key(&mut self, key: char, shift: bool) -> String {
        let result = self.engine.process_key(key, shift);
        serde_json::to_string(&result).unwrap_or_default()
    }

    /// Clear the input buffer (call on word boundary)
    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.engine.clear();
    }

    /// Get current buffer content
    #[wasm_bindgen]
    pub fn get_buffer(&self) -> String {
        self.engine.get_buffer()
    }

    /// Check if IME is enabled
    #[wasm_bindgen]
    pub fn is_enabled(&self) -> bool {
        self.engine.is_enabled()
    }

    /// Toggle IME on/off
    #[wasm_bindgen]
    pub fn toggle(&mut self) {
        self.engine.toggle();
    }
}

impl Default for VietFlux {
    fn default() -> Self {
        Self::new()
    }
}

// Initialize panic hook for better error messages in WASM
#[wasm_bindgen(start)]
pub fn init() {
    // Future: add console_error_panic_hook when needed
}
