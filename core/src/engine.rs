//! Core IME Engine
//!
//! Main engine that coordinates input processing.

use serde::{Deserialize, Serialize};
use crate::buffer::Buffer;
use crate::chars::{self, ToneMark, VowelMod};
use crate::methods::{self, InputMethod, KeyAction};
use crate::transform;
use crate::validation;

/// Result action type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    /// Commit text and clear buffer
    Commit,
    /// Update buffer (backspace + new text)
    Update,
    /// Pass key through unchanged
    Passthrough,
}

/// Result of processing a key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessResult {
    /// Action to take
    pub action: Action,
    /// Output text to send
    pub output: String,
    /// Number of backspaces needed
    pub backspace: usize,
}

impl ProcessResult {
    pub fn passthrough() -> Self {
        Self {
            action: Action::Passthrough,
            output: String::new(),
            backspace: 0,
        }
    }

    pub fn commit(text: String) -> Self {
        Self {
            action: Action::Commit,
            output: text,
            backspace: 0,
        }
    }

    pub fn update(text: String, backspace: usize) -> Self {
        Self {
            action: Action::Update,
            output: text,
            backspace,
        }
    }
}

/// Main IME Engine
pub struct Engine {
    /// Input buffer
    buffer: Buffer,
    /// Current input method
    method: Box<dyn InputMethod>,
    /// Engine enabled state
    enabled: bool,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            method: methods::get_method("telex"),
            enabled: true,
        }
    }

    /// Set input method by name
    pub fn set_method(&mut self, name: &str) {
        self.method = methods::get_method(name);
        self.buffer.clear();
    }

    /// Get current method name
    pub fn get_method(&self) -> &str {
        self.method.name()
    }

    /// Process a key press
    pub fn process_key(&mut self, key: char, _shift: bool) -> ProcessResult {
        if !self.enabled {
            return ProcessResult::passthrough();
        }

        // Check for word boundary
        if validation::is_word_boundary(key) {
            let result = if self.buffer.is_empty() {
                ProcessResult::passthrough()
            } else {
                let text = format!("{}{}", self.buffer.get_text(), key);
                ProcessResult::commit(text)
            };
            self.buffer.clear();
            return result;
        }

        // Get previous character for context
        let prev_char = self.buffer.last().map(|bc| bc.ch);
        
        // Process through input method
        let action = self.method.process(key, prev_char);
        
        match action {
            KeyAction::None => {
                // Regular character - add to buffer
                self.buffer.push_simple(key);
                let text = self.buffer.get_text();
                ProcessResult::update(text, self.buffer.len() - 1)
            }
            
            KeyAction::Tone(tone) => {
                self.apply_tone(tone, key)
            }
            
            KeyAction::Modifier(modifier) => {
                self.apply_modifier(modifier, key)
            }
            
            KeyAction::Stroke => {
                self.apply_stroke()
            }
            
            KeyAction::RemoveDiacritics => {
                self.remove_diacritics()
            }
            
            KeyAction::Undo => {
                // TODO: Implement undo
                ProcessResult::passthrough()
            }
        }
    }

    /// Apply tone mark to appropriate vowel
    fn apply_tone(&mut self, tone: ToneMark, raw_key: char) -> ProcessResult {
        let chars: Vec<char> = self.buffer.iter().map(|bc| bc.ch).collect();
        
        if let Some(pos) = transform::find_tone_position(&chars) {
            let old_char = chars[pos];
            if let Some(new_char) = transform::apply_tone(old_char, tone) {
                self.buffer.replace(pos, new_char);
                let text = self.buffer.get_text();
                return ProcessResult::update(text, self.buffer.len());
            }
        }
        
        // No valid vowel found - treat as regular character
        self.buffer.push_simple(raw_key);
        let text = self.buffer.get_text();
        ProcessResult::update(text, self.buffer.len() - 1)
    }

    /// Apply vowel modifier (circumflex, horn, breve)
    fn apply_modifier(&mut self, modifier: VowelMod, raw_key: char) -> ProcessResult {
        let chars: Vec<char> = self.buffer.iter().map(|bc| bc.ch).collect();
        
        if let Some(pos) = transform::find_modifier_position(&chars, modifier) {
            let old_char = chars[pos];
            if let Some(new_char) = transform::apply_modifier(old_char, modifier) {
                self.buffer.replace(pos, new_char);
                // Pop the trigger key (a, e, o for circumflex, or w for horn/breve)
                if self.method.name() == "telex" {
                    // For Telex, the trigger key was already added, we need to remove it
                    // Actually, we process before adding, so just don't add
                }
                let text = self.buffer.get_text();
                return ProcessResult::update(text, self.buffer.len());
            }
        }
        
        // No valid vowel found - treat as regular character
        self.buffer.push_simple(raw_key);
        let text = self.buffer.get_text();
        ProcessResult::update(text, self.buffer.len() - 1)
    }

    /// Apply stroke (d -> đ)
    fn apply_stroke(&mut self) -> ProcessResult {
        // Find last 'd' and convert to 'đ'
        for i in (0..self.buffer.len()).rev() {
            if let Some(bc) = self.buffer.get(i) {
                if bc.ch.to_ascii_lowercase() == 'd' {
                    let new_char = transform::toggle_stroke(bc.ch);
                    self.buffer.replace(i, new_char);
                    let text = self.buffer.get_text();
                    return ProcessResult::update(text, self.buffer.len());
                }
            }
        }
        
        ProcessResult::passthrough()
    }

    /// Remove all diacritics
    fn remove_diacritics(&mut self) -> ProcessResult {
        let mut changed = false;
        
        for i in 0..self.buffer.len() {
            if let Some(bc) = self.buffer.get(i) {
                let old = bc.ch;
                let new = transform::remove_diacritics(old);
                if old != new {
                    self.buffer.replace(i, new);
                    changed = true;
                }
            }
        }
        
        if changed {
            let text = self.buffer.get_text();
            ProcessResult::update(text, self.buffer.len())
        } else {
            ProcessResult::passthrough()
        }
    }

    /// Clear buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Get current buffer text
    pub fn get_buffer(&self) -> String {
        self.buffer.get_text()
    }

    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Toggle enabled state
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
        if !self.enabled {
            self.buffer.clear();
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telex_basic() {
        let mut engine = Engine::new();
        engine.set_method("telex");
        
        // Type "viet"
        engine.process_key('v', false);
        engine.process_key('i', false);
        engine.process_key('e', false);
        engine.process_key('t', false);
        
        assert_eq!(engine.get_buffer(), "viet");
    }

    #[test]
    fn test_telex_tone() {
        let mut engine = Engine::new();
        engine.set_method("telex");
        
        // Type "as" -> "ás" (s = sắc)
        engine.process_key('a', false);
        engine.process_key('s', false);
        
        assert_eq!(engine.get_buffer(), "á");
    }

    #[test]
    fn test_telex_circumflex() {
        let mut engine = Engine::new();
        engine.set_method("telex");
        
        // Type "aa" -> "â"
        engine.process_key('a', false);
        engine.process_key('a', false);
        
        assert_eq!(engine.get_buffer(), "â");
    }

    #[test]
    fn test_vni_basic() {
        let mut engine = Engine::new();
        engine.set_method("vni");
        
        // Type "a1" -> "á"
        engine.process_key('a', false);
        engine.process_key('1', false);
        
        assert_eq!(engine.get_buffer(), "á");
    }
}
