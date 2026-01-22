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
    /// Special character prefix for shortcuts (e.g., #vn)
    shortcut_prefix: Option<char>,
    /// Auto-capitalize first letter of sentences
    auto_capitalize: bool,
    /// Smart quotes (replace ' and " with curly variants)
    smart_quotes: bool,
    /// Spell check enabled
    spell_check: bool,
    /// Modern tone positioning style (hoà vs hòa)
    /// true = modern (hoà, khoẻ, thuỷ), false = traditional (hòa, khỏe, thủy)
    modern_style: bool,
    /// Flag to capitalize next character
    next_char_upper: bool,
    /// Last committed character (for context)
    last_committed_char: Option<char>,
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
            shortcut_prefix: None,
            auto_capitalize: true,
            smart_quotes: false,
            spell_check: true,
            modern_style: true, // Default to modern style
            next_char_upper: true, // Start with capital
            last_committed_char: None,
        }
    }

    /// Set input method by name
    pub fn set_method(&mut self, name: &str) {
        self.method = methods::get_method(name);
        self.method_name = name.to_lowercase();
        self.buffer.clear();
        self.reset_state();
    }

    /// Set engine options
    pub fn set_options(&mut self, auto_capitalize: bool, smart_quotes: bool, spell_check: bool) {
        self.auto_capitalize = auto_capitalize;
        self.smart_quotes = smart_quotes;
        self.spell_check = spell_check;
    }

    /// Set tone positioning style
    /// true = modern (hoà, khoẻ, thuỷ), false = traditional (hòa, khỏe, thủy)
    pub fn set_modern_style(&mut self, modern: bool) {
        self.modern_style = modern;
    }

    /// Get current style
    pub fn is_modern_style(&self) -> bool {
        self.modern_style
    }

    /// Get engine options
    pub fn get_options(&self) -> (bool, bool, bool) {
        (self.auto_capitalize, self.smart_quotes, self.spell_check)
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

        // Smart Quotes
        if self.smart_quotes && (key == '"' || key == '\'') {
            let is_open = self.buffer.is_empty()
                && (self.last_committed_char.is_none()
                    || self.last_committed_char.unwrap().is_whitespace());

            let quote = if key == '"' {
                if is_open {
                    '“'
                } else {
                    '”'
                }
            } else if is_open {
                '‘'
            } else {
                '’'
            };

            return self.handle_regular_char(quote);
        }

        // Auto-capitalize
        let mut key_to_process = key;
        if self.auto_capitalize && self.next_char_upper && key.is_alphabetic() {
            key_to_process = key.to_uppercase().next().unwrap();
            self.next_char_upper = false;
        }

        // Check for word boundary - triggers auto-restore check
        if validation::is_word_boundary(key_to_process) {
            // Special case: Allow specific symbols as shortcut prefix if buffer is empty
            if self.buffer.is_empty() && self.is_valid_prefix(key_to_process) {
                self.shortcut_prefix = Some(key_to_process);
                return ProcessResult::passthrough();
            }
            return self.handle_word_boundary(key_to_process);
        }

        // Get previous character for context
        let prev_char = self.buffer.last().map(|bc| bc.ch);

        // Check for foreign word pattern BEFORE processing
        let current_text = self.buffer.get_text();
        if validation::is_foreign_word_pattern(&current_text, Some(key_to_process)) {
            self.possible_foreign = true;
        }

        // Process through input method
        let action = self.method.process(key_to_process, prev_char);

        match action {
            KeyAction::None => self.handle_regular_char(key_to_process),

            KeyAction::Tone(tone) => {
                // Skip if foreign word
                if self.possible_foreign {
                    return self.handle_regular_char(key_to_process);
                }

                // VNI fix: If number comes after consonant without vowel, treat as regular char
                // e.g., "var1" should be "var1" not "vár"
                if key_to_process.is_ascii_digit() {
                    let has_vowel = self.buffer.iter().any(|bc| chars::is_vowel(bc.ch));
                    if !has_vowel {
                        return self.handle_regular_char(key_to_process);
                    }
                    // Also check if last char is consonant (like "int1" -> should stay "int1")
                    if let Some(last) = self.buffer.last() {
                        if chars::is_consonant(last.ch) {
                            return self.handle_regular_char(key_to_process);
                        }
                    }
                }

                self.apply_tone(tone, key_to_process)
            }

            KeyAction::Modifier(modifier) => {
                if self.possible_foreign {
                    return self.handle_regular_char(key_to_process);
                }

                // VNI fix: Same logic for modifiers (6, 7, 8)
                if key_to_process.is_ascii_digit() {
                    let has_vowel = self.buffer.iter().any(|bc| chars::is_vowel(bc.ch));
                    if !has_vowel {
                        return self.handle_regular_char(key_to_process);
                    }
                }

                self.apply_modifier(modifier, key_to_process)
            }

            KeyAction::Stroke => self.apply_stroke(),

            KeyAction::RemoveDiacritics => self.remove_all_diacritics(),

            KeyAction::Undo => self.undo_last_transform(),

            KeyAction::QuickTelex(replacement) => {
                self.apply_quick_telex(replacement, key_to_process)
            }

            KeyAction::InsertChar(ch) => self.insert_char_directly(ch),
        }
    }

    /// Insert a character directly into the buffer (for quick shortcuts like [ → ư)
    fn insert_char_directly(&mut self, ch: char) -> ProcessResult {
        self.buffer.push_simple(ch);
        self.last_transform = LastTransform::default();
        let text = self.buffer.get_text();
        ProcessResult::update(text, self.buffer.len())
    }

    /// Handle regular character input
    fn handle_regular_char(&mut self, key: char) -> ProcessResult {
        self.buffer.push_simple(key);
        self.last_transform = LastTransform::default();

        // Check for shortcut match
        let current_text = self.buffer.get_text();
        let full_text = match self.shortcut_prefix {
            Some(prefix) => format!("{}{}", prefix, current_text),
            None => current_text.clone(),
        };

        if let Some(m) = self.shortcuts.try_match(&full_text, false) {
            let replacement = m.replacement.clone();
            // Backspace count includes buffer length + prefix (1) if present
            // Note: m.backspace_count from try_match is the length of the trigger
            // which matches full_text length.
            // But we only need to backspace what's in the buffer + the prefix char that was passed through
            let backspace = if self.shortcut_prefix.is_some() {
                self.buffer.len() + 1
            } else {
                self.buffer.len()
            };

            // Remove trigger chars from buffer
            self.buffer.clear();
            self.shortcut_prefix = None; // Reset prefix

            // Add replacement chars
            for ch in replacement.chars() {
                self.buffer.push_simple(ch);
            }

            let text = self.buffer.get_text();
            return ProcessResult::update(text, backspace);
        }

        let text = self.buffer.get_text();
        ProcessResult::update(text, self.buffer.len() - 1)
    }

    /// Check if char is a valid shortcut prefix
    fn is_valid_prefix(&self, key: char) -> bool {
        matches!(
            key,
            '#' | '@' | '!' | '$' | '%' | '^' | '&' | '*' | '/' | ':'
        )
    }

    /// Handle word boundary - check for auto-restore
    fn handle_word_boundary(&mut self, boundary_char: char) -> ProcessResult {
        if self.buffer.is_empty() {
            // If we have a prefix but no buffer, just clear the prefix
            if self.shortcut_prefix.is_some() {
                self.shortcut_prefix = None;
            }
            return ProcessResult::passthrough();
        }

        // Check for word boundary shortcut
        let current_text = self.buffer.get_text();
        let full_text = match self.shortcut_prefix {
            Some(prefix) => format!("{}{}", prefix, current_text),
            None => current_text.clone(),
        };

        if let Some(m) = self.shortcuts.try_match(&full_text, true) {
            let replacement = m.replacement.clone();
            let backspace = if self.shortcut_prefix.is_some() {
                self.buffer.len() + 1
            } else {
                self.buffer.len()
            };

            self.buffer.clear();
            self.shortcut_prefix = None;
            self.reset_state();

            // Append boundary char to replacement
            let output = format!("{}{}", replacement, boundary_char);

            // Update state
            self.last_committed_char = Some(boundary_char);
            if matches!(boundary_char, '.' | '!' | '?') {
                self.next_char_upper = true;
            }

            return ProcessResult::update(output, backspace);
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

                // Update state
                self.last_committed_char = Some(boundary_char);
                if matches!(boundary_char, '.' | '!' | '?') {
                    self.next_char_upper = true;
                }

                return ProcessResult::restore(output, backspace_count);
            }
        }

        // Valid or acceptable - commit as-is
        let text = format!("{}{}", transformed, boundary_char);
        self.buffer.clear();
        self.reset_state();

        // Update state
        self.last_committed_char = Some(boundary_char);
        if matches!(boundary_char, '.' | '!' | '?') {
            self.next_char_upper = true;
        }

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

        // Find best position for tone (using style setting)
        if let Some(pos) =
            transform::find_tone_position_styled(&chars, &vowel_indices, self.modern_style)
        {
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

    /// Apply stroke (d → đ) with delayed stroke logic
    /// Based on smart stroke handling:
    /// - Allow immediate stroke for short patterns (dd → đ, did → đi)
    /// - Validate syllable structure before applying to prevent invalid transforms
    fn apply_stroke(&mut self) -> ProcessResult {
        let chars: Vec<char> = self.buffer.iter().map(|bc| bc.ch).collect();

        // Check if buffer has any vowels
        let has_vowel = chars.iter().any(|&c| chars::is_vowel(c));

        // Check if any character has a tone mark applied (confirms Vietnamese intent)
        let has_mark = self.buffer.iter().any(|bc| {
            let base = chars::get_base(bc.ch);
            base != bc.ch // If base differs, there's a diacritic
        });

        // Find last 'd' position
        let d_pos = (0..self.buffer.len()).rev().find(|&i| {
            if let Some(bc) = self.buffer.get(i) {
                bc.ch.eq_ignore_ascii_case(&'d') || bc.ch.eq_ignore_ascii_case(&'đ')
            } else {
                false
            }
        });

        if let Some(i) = d_pos {
            if let Some(bc) = self.buffer.get(i) {
                let ch = bc.ch;

                // Delayed stroke logic:
                // If we have vowels but no mark, and this might be English (open syllable),
                // be more cautious. But allow short patterns like "did" → "đi"
                if has_vowel && !has_mark {
                    // Check if this is a short d+vowel+d pattern (2-3 chars)
                    // These are common Vietnamese: did→đi, dod→đo, dud→đu
                    let is_short_pattern = self.buffer.len() <= 3;

                    // If not a short pattern, validate the syllable first
                    if !is_short_pattern {
                        let text = self.buffer.get_text();
                        if !validation::is_valid_syllable(&text) {
                            // Invalid syllable - don't apply stroke
                            return ProcessResult::passthrough();
                        }
                    }
                }

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

    /// Apply Quick Telex: expand double consonant to consonant pair
    /// e.g., "cc" → "ch", "gg" → "gh", "nn" → "nh", etc.
    fn apply_quick_telex(&mut self, replacement: &str, _raw_key: char) -> ProcessResult {
        // Check if buffer has at least one character to pop
        if self.buffer.is_empty() {
            return ProcessResult::passthrough();
        }

        // Remember case of the character we're about to pop
        let was_upper = self
            .buffer
            .last()
            .map(|bc| bc.ch.is_uppercase())
            .unwrap_or(false);

        // Remove the first consonant (which was doubled)
        self.buffer.pop();

        // Add the replacement characters (preserving case of original)
        for (i, ch) in replacement.chars().enumerate() {
            let ch_to_push = if i == 0 && was_upper {
                ch.to_uppercase().next().unwrap_or(ch)
            } else {
                ch
            };
            self.buffer.push_simple(ch_to_push);
        }

        self.last_transform = LastTransform::default();
        let text = self.buffer.get_text();
        ProcessResult::update(text, self.buffer.len())
    }

    /// Reset internal state
    fn reset_state(&mut self) {
        self.last_transform = LastTransform::default();
        self.possible_foreign = false;
    }

    /// Clear buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.shortcut_prefix = None;
        self.reset_state();
    }

    /// Handle backspace - remove last character from buffer
    /// Returns the number of characters that were in the buffer before backspace
    pub fn backspace(&mut self) -> usize {
        let len = self.buffer.len();
        if len > 0 {
            self.buffer.pop();
            self.last_transform = LastTransform::default();
        }
        len
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
        engine.set_options(false, false, false);
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
        engine.set_options(false, false, false);
        engine.set_method("telex");

        engine.process_key('a', false);
        engine.process_key('s', false);

        assert_eq!(engine.get_buffer(), "á");
    }

    #[test]
    fn test_telex_circumflex() {
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        engine.process_key('a', false);
        engine.process_key('a', false);

        assert_eq!(engine.get_buffer(), "â");
    }

    #[test]
    fn test_double_mark_undo() {
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        engine.process_key('a', false);
        engine.process_key('s', false); // á
        engine.process_key('s', false); // should become "as"

        assert_eq!(engine.get_buffer(), "as");
    }

    #[test]
    fn test_vni_basic() {
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("vni");

        engine.process_key('a', false);
        engine.process_key('1', false);

        assert_eq!(engine.get_buffer(), "á");
    }

    #[test]
    fn test_shortcut_expansion() {
        let mut engine = Engine::new();
        engine.set_options(false, false, false); // Disable auto-cap

        engine.process_key('k', false);
        engine.process_key('o', false);

        // Should NOT expand yet (OnWordBoundary default)
        assert_eq!(engine.get_buffer(), "ko");

        // Type space to trigger
        let result = engine.process_key(' ', false);

        // Should expand "ko" to "không" + space
        // Buffer is cleared on boundary commit/update, so check result output
        assert_eq!(result.output, "không ");
    }

    #[test]
    fn test_shortcut_with_prefix() {
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.add_shortcut("#vn", "Việt Nam");

        // 1. Type prefix '#'
        let result = engine.process_key('#', false);
        assert_eq!(result.action, Action::Passthrough);
        assert!(engine.shortcut_prefix.is_some());

        // 2. Type 'v'
        engine.process_key('v', false);
        assert_eq!(engine.get_buffer(), "v");

        // 3. Type 'n'
        engine.process_key('n', false);
        assert_eq!(engine.get_buffer(), "vn");

        // 4. Type SPACE to trigger
        let result = engine.process_key(' ', false);

        // Should be Update action
        assert_eq!(result.action, Action::Update);
        // Output should be replacement + space
        assert_eq!(result.output, "Việt Nam ");
        // Backspace should be 3 (#vn)
        assert_eq!(result.backspace, 3);
    }

    #[test]
    fn test_auto_capitalize() {
        let mut engine = Engine::new();
        engine.set_options(true, false, true);

        // 1. Start of text -> Capitalize
        let result = engine.process_key('h', false);
        assert_eq!(result.output, "H");

        engine.process_key('i', false);

        // 2. Sentence end
        let _ = engine.process_key('.', false); // "Hi."
        assert!(engine.next_char_upper);

        let _ = engine.process_key(' ', false); // "Hi. "
        assert!(engine.next_char_upper);

        let result = engine.process_key('t', false); // "Hi. T"
        assert_eq!(result.output, "T");
        assert!(!engine.next_char_upper);
    }

    #[test]
    fn test_smart_quotes() {
        let mut engine = Engine::new();
        engine.set_options(false, true, true);

        // 1. Open quote
        let result = engine.process_key('"', false);
        assert_eq!(result.output, "\u{201C}"); // Left double quote

        engine.process_key('a', false);

        // 2. Close quote
        let result = engine.process_key('"', false);
        assert!(result.output.ends_with('\u{201D}')); // Right double quote
    }

    #[test]
    fn test_vni_number_after_consonant() {
        // VNI numbers should not apply tone to consonant-only buffers
        // "var1" should stay "var1" not become "vár"
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("vni");

        // Type "var1"
        engine.process_key('v', false);
        engine.process_key('a', false);
        engine.process_key('r', false);
        engine.process_key('1', false);

        // Should be "vár" because there's a vowel 'a' and last char is consonant
        // Actually the rule says: if last char is consonant after vowel, don't apply tone
        // Let me check: buffer = "var", last = 'r' which is consonant
        // So it should be "var1" not "vár"
        assert_eq!(engine.get_buffer(), "var1");
    }

    #[test]
    fn test_vni_number_no_vowel() {
        // VNI: "str1" should stay "str1" (no vowel to apply tone to)
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("vni");

        engine.process_key('s', false);
        engine.process_key('t', false);
        engine.process_key('r', false);
        engine.process_key('1', false);

        assert_eq!(engine.get_buffer(), "str1");
    }

    #[test]
    fn test_vni_number_after_vowel() {
        // VNI: "ba1" should become "bá" (vowel 'a' present, last char is vowel)
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("vni");

        engine.process_key('b', false);
        engine.process_key('a', false);
        engine.process_key('1', false);

        assert_eq!(engine.get_buffer(), "bá");
    }

    #[test]
    fn test_uo_compound() {
        // Test UO compound: "duoc" + horn → "dươc"
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        engine.process_key('d', false);
        engine.process_key('u', false);
        engine.process_key('o', false);
        engine.process_key('c', false);

        // Now apply horn with 'w'
        // Actually we need to go back - the 'w' should apply to u or o
        // Let me create a simpler test
        engine.clear();

        engine.process_key('d', false);
        engine.process_key('u', false);
        engine.process_key('o', false);
        engine.process_key('w', false); // Apply horn to "uo" → "ươ"

        assert_eq!(engine.get_buffer(), "dươ");
    }

    #[test]
    fn test_tone_position_hoa() {
        // Test tone position for "hoa" (modern: hoà, traditional: hòa)
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        // Modern style (default)
        engine.set_modern_style(true);
        engine.process_key('h', false);
        engine.process_key('o', false);
        engine.process_key('a', false);
        engine.process_key('f', false); // grave tone

        assert_eq!(engine.get_buffer(), "hoà"); // Modern: tone on 'a'

        // Traditional style
        engine.clear();
        engine.set_modern_style(false);
        engine.process_key('h', false);
        engine.process_key('o', false);
        engine.process_key('a', false);
        engine.process_key('f', false); // grave tone

        assert_eq!(engine.get_buffer(), "hòa"); // Traditional: tone on 'o'
    }

    #[test]
    fn test_backspace() {
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        engine.process_key('v', false);
        engine.process_key('i', false);
        engine.process_key('e', false);

        assert_eq!(engine.get_buffer(), "vie");
        assert_eq!(engine.backspace(), 3); // Returns previous length
        assert_eq!(engine.get_buffer(), "vi");
        assert_eq!(engine.backspace(), 2);
        assert_eq!(engine.get_buffer(), "v");
    }

    // NOTE: The bracket shortcuts ([ → ư, ] → ơ) are defined in telex.rs
    // but they don't work in engine because brackets are treated as word
    // boundaries before reaching the input method. This is a known limitation.
    // To use ư/ơ quickly, users should use uw/ow instead.

    #[test]
    fn test_uppercase_handling() {
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        // Uppercase with shift
        engine.process_key('V', false);
        engine.process_key('I', false);
        engine.process_key('E', false);
        engine.process_key('T', false);

        assert_eq!(engine.get_buffer(), "VIET");

        // Add tone to uppercase
        engine.clear();
        engine.process_key('A', false);
        engine.process_key('s', false); // tone modifier

        assert_eq!(engine.get_buffer(), "Á");
    }

    #[test]
    fn test_stroke_dd() {
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        engine.process_key('d', false);
        engine.process_key('d', false);

        assert_eq!(engine.get_buffer(), "đ");
    }

    #[test]
    fn test_stroke_uppercase() {
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        engine.process_key('D', false);
        engine.process_key('D', false);

        assert_eq!(engine.get_buffer(), "Đ");
    }

    #[test]
    fn test_quick_telex_nn() {
        // nn → nh
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        engine.process_key('n', false);
        engine.process_key('n', false);

        assert_eq!(engine.get_buffer(), "nh");
    }

    #[test]
    fn test_remove_diacritics_z() {
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        // Type "á" then press 'z' to remove tone
        engine.process_key('a', false);
        engine.process_key('s', false); // á
        assert_eq!(engine.get_buffer(), "á");

        engine.process_key('z', false); // remove diacritics
        assert_eq!(engine.get_buffer(), "a");
    }

    #[test]
    fn test_horn_uw_ow() {
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        // uw → ư
        engine.process_key('u', false);
        engine.process_key('w', false);
        assert_eq!(engine.get_buffer(), "ư");

        // ow → ơ
        engine.clear();
        engine.process_key('o', false);
        engine.process_key('w', false);
        assert_eq!(engine.get_buffer(), "ơ");
    }

    #[test]
    fn test_breve_aw() {
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        // aw → ă
        engine.process_key('a', false);
        engine.process_key('w', false);
        assert_eq!(engine.get_buffer(), "ă");
    }

    #[test]
    fn test_complex_vietnamese_word() {
        // Test "việt" - a complex word with modifier + tone
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        engine.process_key('v', false);
        engine.process_key('i', false);
        engine.process_key('e', false);
        engine.process_key('e', false); // ê
        engine.process_key('j', false); // ệ (dot tone)
        engine.process_key('t', false);

        assert_eq!(engine.get_buffer(), "việt");
    }

    #[test]
    fn test_multiple_vowels_in_word() {
        // Test "người" - uơi vowel cluster
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        engine.process_key('n', false);
        engine.process_key('g', false);
        engine.process_key('u', false);
        engine.process_key('o', false);
        engine.process_key('w', false); // ươ
        engine.process_key('i', false);

        assert_eq!(engine.get_buffer(), "ngươi");

        // Add grave tone
        engine.process_key('f', false);
        assert_eq!(engine.get_buffer(), "người");
    }

    #[test]
    fn test_foreign_word_passthrough() {
        // "hello" should be detected as foreign and not transformed
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        engine.process_key('h', false);
        engine.process_key('e', false);
        engine.process_key('l', false);
        engine.process_key('l', false); // "ll" is foreign
        engine.process_key('o', false);

        // Should remain as "hello" - the 'll' makes it foreign
        assert_eq!(engine.get_buffer(), "hello");
    }

    #[test]
    fn test_vni_circumflex() {
        // VNI: a6 → â, e6 → ê, o6 → ô
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("vni");

        engine.process_key('a', false);
        engine.process_key('6', false);
        assert_eq!(engine.get_buffer(), "â");

        engine.clear();
        engine.process_key('e', false);
        engine.process_key('6', false);
        assert_eq!(engine.get_buffer(), "ê");

        engine.clear();
        engine.process_key('o', false);
        engine.process_key('6', false);
        assert_eq!(engine.get_buffer(), "ô");
    }

    #[test]
    fn test_vni_horn() {
        // VNI: o7 → ơ, u7 → ư
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("vni");

        engine.process_key('o', false);
        engine.process_key('7', false);
        assert_eq!(engine.get_buffer(), "ơ");

        engine.clear();
        engine.process_key('u', false);
        engine.process_key('7', false);
        assert_eq!(engine.get_buffer(), "ư");
    }

    #[test]
    fn test_vni_breve() {
        // VNI: a8 → ă
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("vni");

        engine.process_key('a', false);
        engine.process_key('8', false);
        assert_eq!(engine.get_buffer(), "ă");
    }

    #[test]
    fn test_vni_stroke() {
        // VNI: d9 → đ
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("vni");

        engine.process_key('d', false);
        engine.process_key('9', false);
        assert_eq!(engine.get_buffer(), "đ");
    }

    #[test]
    fn test_vni_all_tones() {
        // VNI: 1=sắc, 2=huyền, 3=hỏi, 4=ngã, 5=nặng
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("vni");

        // a1 → á
        engine.process_key('a', false);
        engine.process_key('1', false);
        assert_eq!(engine.get_buffer(), "á");

        // a2 → à
        engine.clear();
        engine.process_key('a', false);
        engine.process_key('2', false);
        assert_eq!(engine.get_buffer(), "à");

        // a3 → ả
        engine.clear();
        engine.process_key('a', false);
        engine.process_key('3', false);
        assert_eq!(engine.get_buffer(), "ả");

        // a4 → ã
        engine.clear();
        engine.process_key('a', false);
        engine.process_key('4', false);
        assert_eq!(engine.get_buffer(), "ã");

        // a5 → ạ
        engine.clear();
        engine.process_key('a', false);
        engine.process_key('5', false);
        assert_eq!(engine.get_buffer(), "ạ");
    }

    #[test]
    fn test_combined_modifier_and_tone() {
        // Test combining circumflex + tone: â + sắc = ấ
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        engine.process_key('a', false);
        engine.process_key('a', false); // â
        engine.process_key('s', false); // ấ
        assert_eq!(engine.get_buffer(), "ấ");

        // horn + tone: ơ + huyền = ờ
        engine.clear();
        engine.process_key('o', false);
        engine.process_key('w', false); // ơ
        engine.process_key('f', false); // ờ
        assert_eq!(engine.get_buffer(), "ờ");

        // breve + tone: ă + ngã = ẵ
        engine.clear();
        engine.process_key('a', false);
        engine.process_key('w', false); // ă
        engine.process_key('x', false); // ẵ
        assert_eq!(engine.get_buffer(), "ẵ");
    }

    #[test]
    fn test_tone_before_modifier() {
        // Test applying tone before modifier
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        // Type "a" + tone 's' → "á", then 'a' for circumflex → "ấ"
        engine.process_key('a', false);
        engine.process_key('s', false); // á
        engine.process_key('a', false); // ấ
        assert_eq!(engine.get_buffer(), "ấ");
    }

    #[test]
    fn test_horn_after_tone() {
        // Test applying horn after tone: ó + w → ớ
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        engine.process_key('o', false);
        engine.process_key('s', false); // ó
        engine.process_key('w', false); // ớ
        assert_eq!(engine.get_buffer(), "ớ");
    }

    #[test]
    fn test_breve_after_tone() {
        // Test applying breve after tone: á + w → ắ
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        engine.process_key('a', false);
        engine.process_key('s', false); // á
        engine.process_key('w', false); // ắ
        assert_eq!(engine.get_buffer(), "ắ");
    }

    #[test]
    fn test_programming_keyword_passthrough() {
        // "null" should be detected as programming keyword and not transformed
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        engine.process_key('n', false);
        engine.process_key('u', false);
        engine.process_key('l', false);
        engine.process_key('l', false); // "ll" is foreign

        // Should remain as "null"
        assert_eq!(engine.get_buffer(), "null");
    }

    #[test]
    fn test_quick_telex_all() {
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        // cc → ch
        engine.process_key('c', false);
        engine.process_key('c', false);
        assert_eq!(engine.get_buffer(), "ch");

        // gg → gh
        engine.clear();
        engine.process_key('g', false);
        engine.process_key('g', false);
        assert_eq!(engine.get_buffer(), "gh");

        // kk → kh
        engine.clear();
        engine.process_key('k', false);
        engine.process_key('k', false);
        assert_eq!(engine.get_buffer(), "kh");

        // pp → ph
        engine.clear();
        engine.process_key('p', false);
        engine.process_key('p', false);
        assert_eq!(engine.get_buffer(), "ph");

        // tt → th
        engine.clear();
        engine.process_key('t', false);
        engine.process_key('t', false);
        assert_eq!(engine.get_buffer(), "th");

        // qq → qu
        engine.clear();
        engine.process_key('q', false);
        engine.process_key('q', false);
        assert_eq!(engine.get_buffer(), "qu");
    }

    #[test]
    fn test_stroke_toggle() {
        // Test đ + d = toggle back to 'd'
        let mut engine = Engine::new();
        engine.set_options(false, false, false);
        engine.set_method("telex");

        // dd → đ
        engine.process_key('d', false);
        engine.process_key('d', false);
        assert_eq!(engine.get_buffer(), "đ");

        // đd → d (toggle stroke off, just 'd' remains)
        // Note: This is consistent with how tones work (ás → as)
        // The second 'd' toggles the stroke off
        engine.process_key('d', false);
        assert_eq!(engine.get_buffer(), "d");
    }
}
