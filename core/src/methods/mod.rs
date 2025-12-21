//! Input Methods Module
//!
//! Defines Telex and VNI input method implementations.

pub mod telex;
pub mod vni;

pub use telex::Telex;
pub use vni::Vni;

use crate::chars::{ToneMark, VowelMod};

/// Key action result from input method
#[derive(Debug, Clone, PartialEq)]
pub enum KeyAction {
    /// No action - passthrough
    None,
    /// Add tone mark (sắc, huyền, hỏi, ngã, nặng)
    Tone(ToneMark),
    /// Add vowel modifier (circumflex, horn, breve)
    Modifier(VowelMod),
    /// Toggle đ/d
    Stroke,
    /// Remove all diacritics
    RemoveDiacritics,
    /// Undo last transformation
    Undo,
}

/// Input method trait
pub trait InputMethod: Send + Sync {
    /// Get method name
    fn name(&self) -> &'static str;

    /// Process a key and return the action
    fn process(&self, key: char, prev_char: Option<char>) -> KeyAction;

    /// Check if key is a modifier key (not a regular character)
    fn is_modifier_key(&self, key: char) -> bool;
}

/// Get input method by name
pub fn get_method(name: &str) -> Box<dyn InputMethod> {
    match name.to_lowercase().as_str() {
        "vni" => Box::new(Vni),
        _ => Box::new(Telex),
    }
}
