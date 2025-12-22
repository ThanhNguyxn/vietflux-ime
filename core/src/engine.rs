//! Core IME Engine
//!
//! Main engine with advanced features:
//! - Validation-First approach
//! - Foreign Word Detection
//! - Auto-restore on word boundary
//! - UO Compound handling
//! - Tone repositioning
//! - Double mark undo
//! - Shortcut expansion

use crate::buffer::Buffer;
use crate::chars::{self, ToneMark, VowelMod};
use crate::methods::{self, InputMethod, KeyAction};
use crate::shortcut::ShortcutTable;
use crate::transform;
use crate::validation::{self, ValidationResult};
use serde::{Deserialize, Serialize};

/// Engine action result
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    /// Commit text and clear buffer
    Commit,
    /// Update buffer (backspace + new text)
    Update,
    /// Pass key through unchanged
    Passthrough,
    /// Restore ASCII (undo Vietnamese transform)
    Restore,
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
    /// Whether this was an auto-restore
    pub restored: bool,
}

impl ProcessResult {
    pub fn passthrough() -> Self {
        Self {
            action: Action::Passthrough,
            output: String::new(),
            backspace: 0,
            restored: false,
        }
    }

    pub fn commit(text: String) -> Self {
        Self {
            action: Action::Commit,
            output: text,
            backspace: 0,
            restored: false,
        }
    }

    pub fn update(text: String, backspace: usize) -> Self {
        Self {
            action: Action::Update,
            output: text,
            backspace,
            restored: false,
        }
    }

    pub fn restore(raw_text: String, backspace: usize) -> Self {
        Self {
            action: Action::Restore,
            output: raw_text,
            backspace,
            restored: true,
        }
    }
}

/// Last transform info for undo
#[derive(Debug, Clone, Default)]
struct LastTransform {
    /// Position of last transform
    position: Option<usize>,
    /// Type of transform
    #[allow(dead_code)]
    transform_type: TransformType,
    /// Original character before transform
    original: Option<char>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum TransformType {
    #[default]
    None,
    Tone(ToneMark),
    Modifier(VowelMod),
    Stroke,
}

/// Main IME Engine with advanced features
pub struct Engine {
    /// Input buffer
    buffer: Buffer,
    /// Current input method
    method: Box<dyn InputMethod>,
    /// Method name
    method_name: String,
    /// Engine enabled state
    enabled: bool,
    /// Shortcut table
    shortcuts: ShortcutTable,
    /// Last transform for undo
    last_transform: LastTransform,
    /// Track if current word might be foreign
    possible_foreign: bool,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            method: methods::get_method("telex"),
            method_name: "telex".to_string(),
            enabled: true,
            shortcuts: ShortcutTable::with_defaults(),
            last_transform: LastTransform::default(),
            possible_foreign: false,
        }
    }

    /// Set input method by name
    pub fn set_method(&mut self, name: &str) {
        self.method = methods::get_method(name);
        self.method_name = name.to_lowercase();
        self.buffer.clear();
        self.reset_state();
    }

    /// Get current method name
    pub fn get_method(&self) -> &str {
        &self.method_name
    }

    /// Set custom shortcuts
    pub fn set_shortcuts(&mut self, shortcuts: ShortcutTable) {
        self.shortcuts = shortcuts;
    }

    /// Add a shortcut
    pub fn add_shortcut(&mut self, trigger: &str, replacement: &str) {
        use crate::shortcut::Shortcut;
        self.shortcuts.add(Shortcut::new(trigger, replacement));
    }

    /// Remove a shortcut
    pub fn remove_shortcut(&mut self, trigger: &str) {
        self.shortcuts.remove(trigger);
    }

    /// Toggle a shortcut
    pub fn toggle_shortcut(&mut self, trigger: &str) {
        self.shortcuts.toggle(trigger);
    }

    /// Get all shortcuts
    pub fn get_shortcuts(&self) -> Vec<crate::shortcut::Shortcut> {
        self.shortcuts.get_all()
    }

    /// Process a key press
    pub fn process_key(&mut self, key: char, _shift: bool) -> ProcessResult {
        if !self.enabled {
            return ProcessResult::passthrough();
        }

        // Check for word boundary - triggers auto-restore check
        if validation::is_word_boundary(key) {
            return self.handle_word_boundary(key);
        }

        // Get previous character for context
        let prev_char = self.buffer.last().map(|bc| bc.ch);

        // Check for foreign word pattern BEFORE processing
        let current_text = self.buffer.get_text();
        if validation::is_foreign_word_pattern(&current_text, Some(key)) {
            self.possible_foreign = true;
        }

        // Process through input method
        let action = self.method.process(key, prev_char);

        match action {
            KeyAction::None => self.handle_regular_char(key),

            KeyAction::Tone(tone) => {
                // Skip if foreign word
                if self.possible_foreign {
                    return self.handle_regular_char(key);
                }
                self.apply_tone(tone, key)
            }

            KeyAction::Modifier(modifier) => {
                if self.possible_foreign {
                    return self.handle_regular_char(key);
                }
                self.apply_modifier(modifier, key)
            }

            KeyAction::Stroke => self.apply_stroke(),

            KeyAction::RemoveDiacritics => self.remove_all_diacritics(),

            KeyAction::Undo => self.undo_last_transform(),
        }
    }

    /// Handle regular character input
    fn handle_regular_char(&mut self, key: char) -> ProcessResult {
        self.buffer.push_simple(key);
        self.last_transform = LastTransform::default();

        // Check for shortcut match
        if let Some(m) = self.shortcuts.try_match(&self.buffer.get_text(), false) {
            let replacement = m.replacement.clone();
            let backspace = m.backspace_count;

            // Remove trigger chars
            for _ in 0..backspace {
                self.buffer.pop();
            }

            // Add replacement chars
            for ch in replacement.chars() {
                self.buffer.push_simple(ch);
            }

            let text = self.buffer.get_text();
            return ProcessResult::update(
                text,
                self.buffer.len() + backspace - replacement.chars().count(),
            );
        }

        let text = self.buffer.get_text();
        ProcessResult::update(text, self.buffer.len() - 1)
    }

    /// Handle word boundary - check for auto-restore
    fn handle_word_boundary(&mut self, boundary_char: char) -> ProcessResult {
        if self.buffer.is_empty() {
            return ProcessResult::passthrough();
        }

        let transformed = self.buffer.get_text();
        let raw = self.buffer.get_raw();

        // Validate the transformed text
        let validation = validation::validate(&transformed);

        // If invalid Vietnamese AND looks foreign, restore to raw ASCII
        if !validation.is_valid() {
            let should_restore = matches!(
                validation,
                ValidationResult::ForeignWord
                    | ValidationResult::InvalidVowelPattern
                    | ValidationResult::InvalidSpelling
            );

            if should_restore && transformed != raw {
                // Restore to raw ASCII
                let backspace_count = transformed.chars().count();
                let output = format!("{}{}", raw, boundary_char);

                self.buffer.clear();
                self.reset_state();

                return ProcessResult::restore(output, backspace_count);
            }
        }

        // Valid or acceptable - commit as-is
        let text = format!("{}{}", transformed, boundary_char);
        self.buffer.clear();
        self.reset_state();

        ProcessResult::commit(text)
    }

    /// Apply tone mark with smart positioning
    fn apply_tone(&mut self, tone: ToneMark, raw_key: char) -> ProcessResult {
        let chars: Vec<char> = self.buffer.iter().map(|bc| bc.ch).collect();
        let vowel_indices = transform::find_vowel_indices(&chars);

        // Check for double-mark undo
        if let Some(&last_idx) = vowel_indices.last() {
            if transform::should_undo_tone(chars[last_idx], tone) {
                // Undo: remove tone and output raw key
                let (without_tone, _) = transform::remove_tone(chars[last_idx]);
                self.buffer.replace(last_idx, without_tone);
                self.buffer.push_simple(raw_key);

                self.last_transform = LastTransform {
                    position: Some(last_idx),
                    transform_type: TransformType::None,
                    original: None,
                };

                let text = self.buffer.get_text();
                return ProcessResult::update(text, self.buffer.len());
            }
        }

        // Find best position for tone
        if let Some(pos) = transform::find_tone_position(&chars, &vowel_indices) {
            let old_char = chars[pos];
            if let Some(new_char) = transform::apply_tone(old_char, tone) {
                self.buffer.replace(pos, new_char);

                self.last_transform = LastTransform {
                    position: Some(pos),
                    transform_type: TransformType::Tone(tone),
                    original: Some(old_char),
                };

                let text = self.buffer.get_text();
                return ProcessResult::update(text, self.buffer.len());
            }
        }

        // No valid vowel found - treat as regular character
        self.handle_regular_char(raw_key)
    }

    /// Apply vowel modifier with UO compound handling
    fn apply_modifier(&mut self, modifier: VowelMod, raw_key: char) -> ProcessResult {
        let mut chars: Vec<char> = self.buffer.iter().map(|bc| bc.ch).collect();

        // Check for UO compound first (uo → ươ)
        if modifier == VowelMod::Horn {
            if let Some(pos) = self.find_uo_compound(&chars) {
                let result = transform::apply_uo_compound(&mut chars, pos);
                if result.success {
                    // Update buffer with transformed chars
                    self.buffer.replace(pos, chars[pos]);
                    self.buffer.replace(pos + 1, chars[pos + 1]);

                    self.last_transform = LastTransform {
                        position: Some(pos),
                        transform_type: TransformType::Modifier(modifier),
                        original: None,
                    };

                    let text = self.buffer.get_text();
                    return ProcessResult::update(text, self.buffer.len());
                }
            }
        }

        // Find position for modifier
        if let Some(pos) = transform::find_modifier_position(&chars, modifier) {
            let old_char = chars[pos];

            // Check for double-mark undo
            if transform::should_undo_modifier(old_char, modifier) {
                let (without_mod, _) = transform::remove_modifier(old_char);
                self.buffer.replace(pos, without_mod);
                self.buffer.push_simple(raw_key);

                let text = self.buffer.get_text();
                return ProcessResult::update(text, self.buffer.len());
            }

            if let Some(new_char) = transform::apply_modifier(old_char, modifier) {
                self.buffer.replace(pos, new_char);

                self.last_transform = LastTransform {
                    position: Some(pos),
                    transform_type: TransformType::Modifier(modifier),
                    original: Some(old_char),
                };

                let text = self.buffer.get_text();
                return ProcessResult::update(text, self.buffer.len());
            }
        }

        // No valid vowel found - treat as regular character
        self.handle_regular_char(raw_key)
    }

    /// Find UO compound position
    fn find_uo_compound(&self, chars: &[char]) -> Option<usize> {
        for i in 0..chars.len().saturating_sub(1) {
            let first = chars::get_base(chars[i].to_ascii_lowercase());
            let second = chars::get_base(chars[i + 1].to_ascii_lowercase());

            if first == 'u' && second == 'o' {
                return Some(i);
            }
        }
        None
    }

    /// Apply stroke (d → đ)
    fn apply_stroke(&mut self) -> ProcessResult {
        // Find last 'd' and convert to 'đ'
        for i in (0..self.buffer.len()).rev() {
            if let Some(bc) = self.buffer.get(i) {
                let ch = bc.ch;
                if ch.eq_ignore_ascii_case(&'d') || ch.eq_ignore_ascii_case(&'đ') {
                    let new_char = transform::toggle_stroke(ch);
                    self.buffer.replace(i, new_char);

                    self.last_transform = LastTransform {
                        position: Some(i),
                        transform_type: TransformType::Stroke,
                        original: Some(ch),
                    };

                    let text = self.buffer.get_text();
                    return ProcessResult::update(text, self.buffer.len());
                }
            }
        }

        ProcessResult::passthrough()
    }

    /// Remove all diacritics
    fn remove_all_diacritics(&mut self) -> ProcessResult {
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
            self.last_transform = LastTransform::default();
            let text = self.buffer.get_text();
            ProcessResult::update(text, self.buffer.len())
        } else {
            ProcessResult::passthrough()
        }
    }

    /// Undo last transform
    fn undo_last_transform(&mut self) -> ProcessResult {
        if let (Some(pos), Some(original)) =
            (self.last_transform.position, self.last_transform.original)
        {
            if pos < self.buffer.len() {
                self.buffer.replace(pos, original);
                self.last_transform = LastTransform::default();

                let text = self.buffer.get_text();
                return ProcessResult::update(text, self.buffer.len());
            }
        }

        ProcessResult::passthrough()
    }

    /// Reset internal state
    fn reset_state(&mut self) {
        self.last_transform = LastTransform::default();
        self.possible_foreign = false;
    }

    /// Clear buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.reset_state();
    }

    /// Get current buffer text
    pub fn get_buffer(&self) -> String {
        self.buffer.get_text()
    }

    /// Get raw buffer (without transforms)
    pub fn get_raw_buffer(&self) -> String {
        self.buffer.get_raw()
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
            self.reset_state();
        }
    }

    /// Set enabled state
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.buffer.clear();
            self.reset_state();
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telex_basic() {
        let mut engine = Engine::new();
        engine.set_method("telex");

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

        engine.process_key('a', false);
        engine.process_key('s', false);

        assert_eq!(engine.get_buffer(), "á");
    }

    #[test]
    fn test_telex_circumflex() {
        let mut engine = Engine::new();
        engine.set_method("telex");

        engine.process_key('a', false);
        engine.process_key('a', false);

        assert_eq!(engine.get_buffer(), "â");
    }

    #[test]
    fn test_double_mark_undo() {
        let mut engine = Engine::new();
        engine.set_method("telex");

        engine.process_key('a', false);
        engine.process_key('s', false); // á
        engine.process_key('s', false); // should become "as"

        assert_eq!(engine.get_buffer(), "as");
    }

    #[test]
    fn test_vni_basic() {
        let mut engine = Engine::new();
        engine.set_method("vni");

        engine.process_key('a', false);
        engine.process_key('1', false);

        assert_eq!(engine.get_buffer(), "á");
    }

    #[test]
    fn test_shortcut_expansion() {
        let mut engine = Engine::new();

        engine.process_key('k', false);
        engine.process_key('o', false);

        // Should expand "ko" to "không"
        assert_eq!(engine.get_buffer(), "không");
    }
}
