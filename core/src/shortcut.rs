//! Shortcut System - Text Abbreviation Expansion
//!
//! Allows users to define shortcuts like "vn" → "Việt Nam"
//! Shortcuts can be enabled/disabled and added/removed at runtime.

use std::collections::HashMap;

/// Maximum replacement length
pub const MAX_REPLACEMENT_LEN: usize = 63;

/// When to trigger
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TriggerCondition {
    /// Trigger immediately when buffer matches
    #[default]
    Immediate,
    /// Trigger on word boundary (space, punctuation)
    OnWordBoundary,
}

/// A single shortcut entry
#[derive(Debug, Clone)]
pub struct Shortcut {
    pub trigger: String,
    pub replacement: String,
    pub condition: TriggerCondition,
    pub enabled: bool,
}

impl Shortcut {
    /// Create immediate trigger shortcut
    pub fn new(trigger: &str, replacement: &str) -> Self {
        Self {
            trigger: trigger.to_string(),
            replacement: replacement.chars().take(MAX_REPLACEMENT_LEN).collect(),
            condition: TriggerCondition::Immediate,
            enabled: true,
        }
    }

    /// Create word boundary trigger shortcut
    pub fn on_boundary(trigger: &str, replacement: &str) -> Self {
        Self {
            trigger: trigger.to_string(),
            replacement: replacement.chars().take(MAX_REPLACEMENT_LEN).collect(),
            condition: TriggerCondition::OnWordBoundary,
            enabled: true,
        }
    }
}

/// Shortcut match result
#[derive(Debug)]
pub struct ShortcutMatch {
    pub backspace_count: usize,
    pub replacement: String,
}

/// Shortcut table with on/off functionality
#[derive(Debug, Default)]
pub struct ShortcutTable {
    enabled: bool,
    shortcuts: HashMap<String, Shortcut>,
    sorted_triggers: Vec<String>,
}

impl ShortcutTable {
    /// Create empty table (disabled by default)
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with default Vietnamese shortcuts
    pub fn with_defaults() -> Self {
        let mut table = Self::new();
        table.enabled = true;
        table.add(Shortcut::new("vn", "Việt Nam"));
        table.add(Shortcut::new("hcm", "Hồ Chí Minh"));
        table.add(Shortcut::new("hn", "Hà Nội"));
        table.add(Shortcut::new("dc", "được"));
        table.add(Shortcut::new("ko", "không"));
        table
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Add a shortcut
    pub fn add(&mut self, shortcut: Shortcut) {
        let trigger = shortcut.trigger.clone();
        self.shortcuts.insert(trigger, shortcut);
        self.rebuild_sorted();
    }

    /// Remove a shortcut
    pub fn remove(&mut self, trigger: &str) -> Option<Shortcut> {
        let result = self.shortcuts.remove(trigger);
        if result.is_some() {
            self.rebuild_sorted();
        }
        result
    }

    /// Try to match buffer
    pub fn try_match(&self, buffer: &str, is_word_boundary: bool) -> Option<ShortcutMatch> {
        if !self.enabled {
            return None;
        }

        for trigger in &self.sorted_triggers {
            if buffer == trigger {
                if let Some(s) = self.shortcuts.get(trigger) {
                    if !s.enabled {
                        continue;
                    }
                    match s.condition {
                        TriggerCondition::Immediate => {
                            return Some(ShortcutMatch {
                                backspace_count: trigger.len(),
                                replacement: s.replacement.clone(),
                            });
                        }
                        TriggerCondition::OnWordBoundary if is_word_boundary => {
                            return Some(ShortcutMatch {
                                backspace_count: trigger.len(),
                                replacement: s.replacement.clone(),
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
        None
    }

    fn rebuild_sorted(&mut self) {
        self.sorted_triggers = self.shortcuts.keys().cloned().collect();
        self.sorted_triggers
            .sort_by_key(|s| std::cmp::Reverse(s.len()));
    }

    pub fn len(&self) -> usize {
        self.shortcuts.len()
    }
    pub fn is_empty(&self) -> bool {
        self.shortcuts.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcut_table() {
        let table = ShortcutTable::with_defaults();
        assert!(table.is_enabled());
        assert!(table.len() >= 5);

        let m = table.try_match("ko", false);
        assert!(m.is_some());
        assert_eq!(m.unwrap().replacement, "không");
    }
}
