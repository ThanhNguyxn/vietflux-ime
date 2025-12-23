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
                self.apply_tone(tone, key_to_process)
            }

            KeyAction::Modifier(modifier) => {
                if self.possible_foreign {
                    return self.handle_regular_char(key_to_process);
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
        // Remove the first consonant (which was doubled)
        if !self.buffer.is_empty() {
            self.buffer.pop();
        }

        // Add the replacement characters (preserving case of original)
        let was_upper = self
            .buffer
            .last()
            .map(|bc| bc.ch.is_uppercase())
            .unwrap_or(false);

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
        assert_eq!(result.output, "“");

        engine.process_key('a', false);

        // 2. Close quote
        let result = engine.process_key('"', false);
        assert!(result.output.ends_with("”"));
    }
}
