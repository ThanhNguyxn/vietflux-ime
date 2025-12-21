//! Shortcut/Abbreviation Expansion System
//!
//! Allows user-defined text shortcuts like "hv" → "không"

use std::collections::HashMap;

/// A single shortcut definition
#[derive(Debug, Clone)]
pub struct Shortcut {
    /// The abbreviation to type
    pub trigger: String,
    /// The expanded text
    pub expansion: String,
}

impl Shortcut {
    pub fn new(trigger: impl Into<String>, expansion: impl Into<String>) -> Self {
        Self {
            trigger: trigger.into(),
            expansion: expansion.into(),
        }
    }
}

/// Shortcut table with priority-based matching
#[derive(Debug, Clone, Default)]
pub struct ShortcutTable {
    /// Shortcuts indexed by first character for fast lookup
    shortcuts: HashMap<char, Vec<Shortcut>>,
    /// All shortcuts for iteration
    all: Vec<Shortcut>,
}

impl ShortcutTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a shortcut
    pub fn add(&mut self, trigger: impl Into<String>, expansion: impl Into<String>) {
        let shortcut = Shortcut::new(trigger, expansion);
        if let Some(first_char) = shortcut.trigger.chars().next() {
            self.shortcuts
                .entry(first_char)
                .or_default()
                .push(shortcut.clone());
        }
        self.all.push(shortcut);
    }

    /// Add multiple shortcuts
    pub fn add_all(&mut self, shortcuts: impl IntoIterator<Item = (String, String)>) {
        for (trigger, expansion) in shortcuts {
            self.add(trigger, expansion);
        }
    }

    /// Find longest matching shortcut for the given buffer
    /// Uses longest-match-first strategy
    pub fn find_match(&self, buffer: &str) -> Option<&Shortcut> {
        let lower = buffer.to_lowercase();

        // Find longest match
        let mut best_match: Option<&Shortcut> = None;

        for shortcut in &self.all {
            if lower.ends_with(&shortcut.trigger.to_lowercase()) {
                match &best_match {
                    Some(current) if current.trigger.len() >= shortcut.trigger.len() => {}
                    _ => best_match = Some(shortcut),
                }
            }
        }

        best_match
    }

    /// Check if buffer could potentially match a shortcut
    pub fn has_potential_match(&self, buffer: &str) -> bool {
        let lower = buffer.to_lowercase();

        for shortcut in &self.all {
            let trigger = shortcut.trigger.to_lowercase();
            if trigger.starts_with(&lower) {
                return true;
            }
        }

        false
    }

    /// Clear all shortcuts
    pub fn clear(&mut self) {
        self.shortcuts.clear();
        self.all.clear();
    }

    /// Get number of shortcuts
    pub fn len(&self) -> usize {
        self.all.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.all.is_empty()
    }
}

/// Default Vietnamese shortcuts commonly used
pub fn default_shortcuts() -> ShortcutTable {
    let mut table = ShortcutTable::new();

    // Common abbreviations
    table.add("ko", "không");
    table.add("dc", "được");
    table.add("vs", "với");
    table.add("ng", "người");
    table.add("ntn", "như thế nào");
    table.add("bt", "bình thường");
    table.add("vd", "ví dụ");
    table.add("tg", "thời gian");
    table.add("nc", "nước");
    table.add("ck", "chồng");
    table.add("vk", "vợ");
    table.add("nyc", "người yêu cũ");
    table.add("ny", "người yêu");

    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcut_match() {
        let mut table = ShortcutTable::new();
        table.add("ko", "không");
        table.add("dc", "được");

        let result = table.find_match("ko");
        assert!(result.is_some());
        assert_eq!(result.unwrap().expansion, "không");
    }

    #[test]
    fn test_longest_match() {
        let mut table = ShortcutTable::new();
        table.add("n", "này");
        table.add("ng", "người");
        table.add("ngu", "ngủ");

        // Should match longest
        let result = table.find_match("xyzng");
        assert!(result.is_some());
        assert_eq!(result.unwrap().expansion, "người");
    }

    #[test]
    fn test_no_match() {
        let table = ShortcutTable::new();
        assert!(table.find_match("abc").is_none());
    }
}
