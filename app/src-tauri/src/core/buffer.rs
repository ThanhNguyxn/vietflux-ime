//! Composition Buffer
//!
//! Manages the input buffer for Vietnamese text composition.
//! Each character is represented as an InputUnit with modifiers.

use super::data::keycodes::{keys, is_vowel};
use super::data::unicode::{self, tone, mark};

/// Maximum buffer size (characters per word)
pub const MAX_BUFFER_SIZE: usize = 32;

/// Single input unit in the composition buffer
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputUnit {
    /// Base keycode (a, e, i, o, u, y, b, c, d, ...)
    pub key: u16,
    /// Uppercase flag
    pub is_uppercase: bool,
    /// Tone modifier: 0=none, 1=circumflex, 2=horn/breve
    pub tone_mod: u8,
    /// Tone mark: 0=none, 1=acute, 2=grave, 3=hook, 4=tilde, 5=dot
    pub mark_type: u8,
    /// Stroke flag (for đ)
    pub is_stroked: bool,
}

impl InputUnit {
    /// Create a new input unit from keycode
    pub fn new(key: u16, is_uppercase: bool) -> Self {
        Self {
            key,
            is_uppercase,
            tone_mod: tone::NONE,
            mark_type: mark::NONE,
            is_stroked: false,
        }
    }

    /// Check if this unit is a vowel
    pub fn is_vowel(&self) -> bool {
        is_vowel(self.key)
    }

    /// Check if this unit has any modifier (tone or mark)
    pub fn has_modifier(&self) -> bool {
        self.tone_mod != tone::NONE || self.mark_type != mark::NONE
    }

    /// Convert to Vietnamese character
    pub fn to_char(&self) -> Option<char> {
        if self.key == keys::D && self.is_stroked {
            return Some(unicode::get_stroke_d(self.is_uppercase));
        }
        unicode::compose_char(self.key, self.is_uppercase, self.tone_mod, self.mark_type)
    }
}

/// Composition buffer for Vietnamese input
#[derive(Clone, Debug)]
pub struct CompositionBuffer {
    /// Input units in the buffer
    units: Vec<InputUnit>,
    /// Raw input history (for ESC restore)
    raw_input: Vec<char>,
}

impl Default for CompositionBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl CompositionBuffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        Self {
            units: Vec::with_capacity(MAX_BUFFER_SIZE),
            raw_input: Vec::with_capacity(MAX_BUFFER_SIZE),
        }
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    /// Get buffer length
    pub fn len(&self) -> usize {
        self.units.len()
    }

    /// Push a new unit to buffer
    pub fn push(&mut self, unit: InputUnit, raw_char: char) {
        if self.units.len() < MAX_BUFFER_SIZE {
            self.units.push(unit);
            self.raw_input.push(raw_char);
        }
    }

    /// Pop the last unit from buffer
    pub fn pop(&mut self) -> Option<InputUnit> {
        self.raw_input.pop();
        self.units.pop()
    }

    /// Get unit at position
    pub fn get(&self, index: usize) -> Option<&InputUnit> {
        self.units.get(index)
    }

    /// Get mutable unit at position
    pub fn get_mut(&mut self, index: usize) -> Option<&mut InputUnit> {
        self.units.get_mut(index)
    }

    /// Get last unit
    pub fn last(&self) -> Option<&InputUnit> {
        self.units.last()
    }

    /// Get mutable last unit
    pub fn last_mut(&mut self) -> Option<&mut InputUnit> {
        self.units.last_mut()
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.units.clear();
        self.raw_input.clear();
    }

    /// Get all keycodes in buffer
    pub fn keys(&self) -> Vec<u16> {
        self.units.iter().map(|u| u.key).collect()
    }

    /// Get all tone modifiers in buffer
    pub fn tones(&self) -> Vec<u8> {
        self.units.iter().map(|u| u.tone_mod).collect()
    }

    /// Find all vowel positions
    pub fn find_vowels(&self) -> Vec<usize> {
        self.units
            .iter()
            .enumerate()
            .filter(|(_, u)| u.is_vowel())
            .map(|(i, _)| i)
            .collect()
    }

    /// Convert buffer to Vietnamese string
    pub fn to_string(&self) -> String {
        self.units
            .iter()
            .filter_map(|u| u.to_char())
            .collect()
    }

    /// Get raw input string (for ESC restore)
    pub fn raw_string(&self) -> String {
        self.raw_input.iter().collect()
    }

    /// Iterate over units
    pub fn iter(&self) -> impl Iterator<Item = &InputUnit> {
        self.units.iter()
    }

    /// Mutable iterate over units
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut InputUnit> {
        self.units.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_unit_creation() {
        let unit = InputUnit::new(keys::A, false);
        assert_eq!(unit.key, keys::A);
        assert!(!unit.is_uppercase);
        assert!(unit.is_vowel());
        assert!(!unit.has_modifier());
    }

    #[test]
    fn test_buffer_push_pop() {
        let mut buf = CompositionBuffer::new();
        assert!(buf.is_empty());

        buf.push(InputUnit::new(keys::A, false), 'a');
        assert_eq!(buf.len(), 1);

        let popped = buf.pop();
        assert!(popped.is_some());
        assert!(buf.is_empty());
    }

    #[test]
    fn test_buffer_to_string() {
        let mut buf = CompositionBuffer::new();
        buf.push(InputUnit::new(keys::V, false), 'v');
        buf.push(InputUnit::new(keys::I, false), 'i');
        buf.push(InputUnit::new(keys::E, false), 'e');
        buf.push(InputUnit::new(keys::T, false), 't');

        assert_eq!(buf.to_string(), "viet");
    }

    #[test]
    fn test_buffer_with_modifiers() {
        let mut buf = CompositionBuffer::new();
        
        let mut unit = InputUnit::new(keys::A, false);
        unit.mark_type = mark::ACUTE;
        buf.push(unit, 'a');

        assert_eq!(buf.to_string(), "á");
    }
}
