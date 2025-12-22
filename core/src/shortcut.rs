//! Shortcut System - Text Abbreviation Expansion
//!
//! Allows users to define shortcuts like "vn" → "Việt Nam"
//! Shortcuts can be enabled/disabled and added/removed at runtime.
//!
//! Based on GoNhanh's shortcut system, adapted for VietFlux.

use std::collections::HashMap;

/// Maximum replacement length in characters
pub const MAX_REPLACEMENT_LEN: usize = 63;

/// When to trigger the shortcut
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TriggerCondition {
    /// Trigger immediately when buffer matches
    Immediate,
    /// Trigger when word boundary (space, punctuation) is pressed
    #[default]
    OnWordBoundary,
}

/// A single shortcut entry
#[derive(Debug, Clone)]
pub struct Shortcut {
    /// Trigger string (case-sensitive)
    pub trigger: String,
    /// Replacement text
    pub replacement: String,
    /// When to trigger
    pub condition: TriggerCondition,
    /// Whether this shortcut is enabled
    pub enabled: bool,
}

impl Shortcut {
    /// Create a new shortcut (triggers on word boundary by default)
    pub fn new(trigger: &str, replacement: &str) -> Self {
        Self {
            trigger: trigger.to_string(),
            replacement: Self::validate_replacement(replacement),
            condition: TriggerCondition::OnWordBoundary,
            enabled: true,
        }
    }

    /// Create an immediate trigger shortcut
    pub fn immediate(trigger: &str, replacement: &str) -> Self {
        Self {
            trigger: trigger.to_string(),
            replacement: Self::validate_replacement(replacement),
            condition: TriggerCondition::Immediate,
            enabled: true,
        }
    }

    /// Validate and truncate replacement if too long
    fn validate_replacement(replacement: &str) -> String {
        let char_count = replacement.chars().count();
        if char_count <= MAX_REPLACEMENT_LEN {
            replacement.to_string()
        } else {
            replacement.chars().take(MAX_REPLACEMENT_LEN).collect()
        }
    }

    /// Enable this shortcut
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable this shortcut
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

/// Shortcut match result
#[derive(Debug)]
pub struct ShortcutMatch {
    /// Number of characters to backspace
    pub backspace_count: usize,
    /// Replacement text to output
    pub replacement: String,
    /// Whether to include the trigger key (space) in output
    pub include_trigger_key: bool,
}

/// Shortcut table manager with on/off functionality
#[derive(Debug, Default)]
pub struct ShortcutTable {
    /// Global enable/disable for entire shortcut system
    enabled: bool,
    /// Shortcuts indexed by trigger
    shortcuts: HashMap<String, Shortcut>,
    /// Sorted triggers by length (longest first) for matching
    sorted_triggers: Vec<String>,
}

impl ShortcutTable {
    /// Create a new empty shortcut table (disabled by default)
    pub fn new() -> Self {
        Self {
            enabled: false,
            shortcuts: HashMap::new(),
            sorted_triggers: vec![],
        }
    }

    /// Create with default Vietnamese shortcuts
    pub fn with_defaults() -> Self {
        let mut table = Self::new();
        table.enabled = true;

        // Common Vietnamese abbreviations (immediate trigger for compatibility)
        table.add(Shortcut::immediate("vn", "Việt Nam"));
        table.add(Shortcut::immediate("hcm", "Hồ Chí Minh"));
        table.add(Shortcut::immediate("hn", "Hà Nội"));
        table.add(Shortcut::immediate("dc", "được"));
        table.add(Shortcut::immediate("ko", "không"));

        table
    }

    /// Enable the shortcut system
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the shortcut system
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if shortcut system is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Add a shortcut
    pub fn add(&mut self, shortcut: Shortcut) {
        let trigger = shortcut.trigger.clone();
        self.shortcuts.insert(trigger, shortcut);
        self.rebuild_sorted_triggers();
    }

    /// Remove a shortcut by trigger
    pub fn remove(&mut self, trigger: &str) -> Option<Shortcut> {
        let result = self.shortcuts.remove(trigger);
        if result.is_some() {
            self.rebuild_sorted_triggers();
        }
        result
    }

    /// Enable a specific shortcut by trigger
    pub fn enable_shortcut(&mut self, trigger: &str) -> bool {
        if let Some(shortcut) = self.shortcuts.get_mut(trigger) {
            shortcut.enable();
            true
        } else {
            false
        }
    }

    /// Disable a specific shortcut by trigger
    pub fn disable_shortcut(&mut self, trigger: &str) -> bool {
        if let Some(shortcut) = self.shortcuts.get_mut(trigger) {
            shortcut.disable();
            true
        } else {
            false
        }
    }

    /// Get a shortcut by trigger
    pub fn get(&self, trigger: &str) -> Option<&Shortcut> {
        self.shortcuts.get(trigger)
    }

    /// Get mutable shortcut by trigger
    pub fn get_mut(&mut self, trigger: &str) -> Option<&mut Shortcut> {
        self.shortcuts.get_mut(trigger)
    }

    /// Try to match buffer with shortcut
    ///
    /// # Arguments
    /// * `buffer` - Current buffer content
    /// * `trigger_char` - The character that triggered this check (e.g., space)
    /// * `is_word_boundary` - Whether trigger_char is a word boundary
    ///
    /// # Returns
    /// ShortcutMatch if a shortcut should be triggered
    pub fn try_match(
        &self,
        buffer: &str,
        trigger_char: Option<char>,
        is_word_boundary: bool,
    ) -> Option<ShortcutMatch> {
        // System disabled → no match
        if !self.enabled {
            return None;
        }

        // Longest-match-first
        for trigger in &self.sorted_triggers {
            if buffer == trigger {
                if let Some(shortcut) = self.shortcuts.get(trigger) {
                    if !shortcut.enabled {
                        continue;
                    }

                    match shortcut.condition {
                        TriggerCondition::Immediate => {
                            return Some(ShortcutMatch {
                                backspace_count: trigger.len(),
                                replacement: shortcut.replacement.clone(),
                                include_trigger_key: false,
                            });
                        }
                        TriggerCondition::OnWordBoundary => {
                            if is_word_boundary {
                                let mut replacement = shortcut.replacement.clone();
                                if let Some(ch) = trigger_char {
                                    replacement.push(ch);
                                }
                                return Some(ShortcutMatch {
                                    backspace_count: trigger.len(),
                                    replacement,
                                    include_trigger_key: true,
                                });
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Rebuild sorted triggers list (longest first)
    fn rebuild_sorted_triggers(&mut self) {
        self.sorted_triggers = self.shortcuts.keys().cloned().collect();
        self.sorted_triggers
            .sort_by_key(|s| std::cmp::Reverse(s.len()));
    }

    /// Check if table is empty
    pub fn is_empty(&self) -> bool {
        self.shortcuts.is_empty()
    }

    /// Get number of shortcuts
    pub fn len(&self) -> usize {
        self.shortcuts.len()
    }

    /// Clear all shortcuts
    pub fn clear(&mut self) {
        self.shortcuts.clear();
        self.sorted_triggers.clear();
    }

    /// List all triggers
    pub fn list_triggers(&self) -> Vec<&str> {
        self.shortcuts.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcut_creation() {
        let s = Shortcut::new("vn", "Việt Nam");
        assert_eq!(s.trigger, "vn");
        assert_eq!(s.replacement, "Việt Nam");
        assert!(s.enabled);
    }

    #[test]
    fn test_shortcut_table_add_remove() {
        let mut table = ShortcutTable::new();
        table.enable();

        table.add(Shortcut::new("vn", "Việt Nam"));
        assert_eq!(table.len(), 1);

        let removed = table.remove("vn");
        assert!(removed.is_some());
        assert_eq!(table.len(), 0);
    }

    #[test]
    fn test_shortcut_table_disabled() {
        let mut table = ShortcutTable::new();
        table.add(Shortcut::new("vn", "Việt Nam"));
        // Table disabled by default
        assert!(!table.is_enabled());

        let result = table.try_match("vn", Some(' '), true);
        assert!(result.is_none());
    }

    #[test]
    fn test_shortcut_table_enabled() {
        let mut table = ShortcutTable::new();
        table.enable();
        table.add(Shortcut::new("vn", "Việt Nam"));

        let result = table.try_match("vn", Some(' '), true);
        assert!(result.is_some());
        let m = result.unwrap();
        assert_eq!(m.replacement, "Việt Nam ");
        assert_eq!(m.backspace_count, 2);
    }

    #[test]
    fn test_shortcut_individual_disable() {
        let mut table = ShortcutTable::new();
        table.enable();
        table.add(Shortcut::new("vn", "Việt Nam"));

        // Disable specific shortcut
        table.disable_shortcut("vn");

        let result = table.try_match("vn", Some(' '), true);
        assert!(result.is_none());

        // Re-enable
        table.enable_shortcut("vn");
        let result = table.try_match("vn", Some(' '), true);
        assert!(result.is_some());
    }

    #[test]
    fn test_shortcut_with_defaults() {
        let table = ShortcutTable::with_defaults();
        assert!(table.is_enabled());
        assert!(table.len() >= 5);

        // Immediate trigger - doesn't need word boundary
        let result = table.try_match("dc", None, false);
        assert!(result.is_some());
        let m = result.unwrap();
        assert_eq!(m.replacement, "được");
    }

    #[test]
    fn test_longest_match_first() {
        let mut table = ShortcutTable::new();
        table.enable();
        table.add(Shortcut::new("h", "họ"));
        table.add(Shortcut::new("hcm", "Hồ Chí Minh"));

        // "hcm" should match longer trigger
        let result = table.try_match("hcm", Some(' '), true);
        assert!(result.is_some());
        let m = result.unwrap();
        assert!(m.replacement.contains("Hồ Chí Minh"));
    }

    #[test]
    fn test_immediate_trigger() {
        let mut table = ShortcutTable::new();
        table.enable();
        table.add(Shortcut::immediate(";;", "→"));

        // Immediate doesn't need word boundary
        let result = table.try_match(";;", None, false);
        assert!(result.is_some());
        let m = result.unwrap();
        assert_eq!(m.replacement, "→");
        assert!(!m.include_trigger_key);
    }

    #[test]
    fn test_replacement_validation() {
        // Very long replacement should be truncated
        let long_text = "a".repeat(100);
        let s = Shortcut::new("test", &long_text);
        assert!(s.replacement.len() <= MAX_REPLACEMENT_LEN);
    }
}
