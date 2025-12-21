//! Vietnamese Syllable Validation
//!
//! Validates if a string is a valid Vietnamese syllable.
//! Based on Vietnamese phonology rules with Foreign Word Detection.

use crate::chars::{self, VowelMod, REVERSE_MAP};

// ============================================================
// PHONOLOGY CONSTANTS
// ============================================================

/// Valid initial consonants (phụ âm đầu) - 16 single + 10 pairs + ngh
pub const VALID_INITIALS: &[&str] = &[
    // Single consonants
    "", "b", "c", "d", "đ", "g", "h", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "x",
    // Consonant pairs
    "ch", "gh", "gi", "kh", "ng", "nh", "ph", "th", "tr", "qu",
    // Special triple
    "ngh",
];

/// Valid final consonants (phụ âm cuối)  
pub const VALID_FINALS: &[&str] = &[
    "", "c", "ch", "m", "n", "ng", "nh", "p", "t",
];

/// Valid vowel nuclei (including diphthongs and triphthongs)
pub const VALID_VOWEL_PATTERNS: &[&str] = &[
    // Single vowels (base)
    "a", "ă", "â", "e", "ê", "i", "o", "ô", "ơ", "u", "ư", "y",
    // Diphthongs (nguyên âm đôi)
    "ai", "ao", "au", "ay", "âu", "ây", 
    "eo", "êu",
    "ia", "iê", "iu",
    "oa", "oă", "oe", "oi", "oo", "ôi", "ơi",
    "ua", "uâ", "uê", "ui", "uo", "uô", "uơ", "ươ",
    "ưa", "ưi", "ưu",
    "ya", "yê",
    // Triphthongs (nguyên âm ba)
    "iêu", "oai", "oay", "oeo", "uây", "uôi", "ươi", "ươu", "yêu",
    // With qu- prefix patterns
    "uya", "uyê", "uyu",
];

/// Foreign word consonant clusters (not valid in Vietnamese)
#[allow(dead_code)]
const FOREIGN_CLUSTERS: &[&str] = &[
    "tr", "pr", "cr", "br", "dr", "gr", "fr",  // After finals
    "st", "sp", "sc", "sk", "sm", "sn", "sl", "sw",
    "bl", "cl", "fl", "gl", "pl",
    "ck", "dg", "ght", "wh", "wr",
];

/// Invalid vowel patterns (impossible in Vietnamese)
#[allow(dead_code)]
const INVALID_VOWEL_PATTERNS: &[&str] = &[
    // These vowel combinations don't exist in Vietnamese
    "eư", "oư", "iư",  // ư cannot follow e, o, i directly
    "ưe", "ưo", "ưy",  // Invalid ư combinations
    "ou", "yo",        // English-like patterns
    "ea", "ie", "ei",  // English diphthongs
];

// ============================================================
// VALIDATION RESULT
// ============================================================

/// Validation result with detailed failure reason
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    InvalidInitial,
    InvalidFinal,
    InvalidSpelling,
    InvalidVowelPattern,
    NoVowel,
    ForeignWord,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }
}

// ============================================================
// VALIDATION FUNCTIONS
// ============================================================

/// Full validation with detailed result
pub fn validate(s: &str) -> ValidationResult {
    if s.is_empty() {
        return ValidationResult::NoVowel;
    }
    
    let lower = s.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();
    
    // Rule 1: Must have vowel
    if !has_vowel(&chars) {
        return ValidationResult::NoVowel;
    }
    
    // Rule 2: Check for foreign word patterns first
    if is_foreign_pattern(&lower) {
        return ValidationResult::ForeignWord;
    }
    
    // Try to parse syllable structure: Initial + Vowel + Final
    if let Some((initial, vowel, final_c)) = parse_syllable(&lower) {
        // Rule 3: Valid initial consonant
        if !VALID_INITIALS.contains(&initial) {
            return ValidationResult::InvalidInitial;
        }
        
        // Rule 4: Valid final consonant
        if !VALID_FINALS.contains(&final_c) {
            return ValidationResult::InvalidFinal;
        }
        
        // Rule 5: Spelling rules (c/k/g restrictions)
        if !check_spelling_rules(initial, vowel) {
            return ValidationResult::InvalidSpelling;
        }
        
        // Rule 6: Valid vowel pattern (diphthong/triphthong)
        let vowel_base = normalize_vowel(vowel);
        if !is_valid_vowel_pattern(&vowel_base) {
            return ValidationResult::InvalidVowelPattern;
        }
        
        ValidationResult::Valid
    } else {
        ValidationResult::InvalidSpelling
    }
}

/// Quick check if valid
pub fn is_valid_syllable(s: &str) -> bool {
    validate(s).is_valid()
}

/// Check if pattern looks like a foreign/English word
pub fn is_foreign_word_pattern(buffer: &str, modifier_key: Option<char>) -> bool {
    let lower = buffer.to_lowercase();
    
    // Skip check if buffer has Vietnamese diacritics (horn, circumflex, breve)
    // User is typing Vietnamese intentionally
    if has_vietnamese_diacritics(&lower) {
        return false;
    }
    
    // Skip check for valid typing sequences like "dodo" (typing đô)
    if is_valid_typing_sequence(&lower) {
        return false;
    }
    
    // Check for invalid vowel patterns (eư, oư, etc)
    for pattern in INVALID_VOWEL_PATTERNS {
        if lower.contains(pattern) {
            return true;
        }
    }
    
    // Check for foreign consonant clusters (not at start)
    for cluster in FOREIGN_CLUSTERS {
        if lower.contains(cluster) {
            // Allow "tr" at the start (Vietnamese "tr")
            if *cluster == "tr" && lower.starts_with("tr") {
                continue;
            }
            // Check if cluster is after a vowel (foreign pattern)
            if let Some(pos) = lower.find(cluster) {
                if pos > 0 {
                    let prev_char = lower.chars().nth(pos - 1);
                    if prev_char.map(|c| crate::chars::is_vowel(c)).unwrap_or(false) {
                        return true;
                    }
                }
            }
        }
    }
    
    // English prefix patterns: de+s (describe, design)
    if lower.starts_with("de") && modifier_key == Some('s') {
        // But not "des" alone which could be Vietnamese
        if lower.len() > 3 {
            return true;
        }
    }
    
    // pr- prefix not at start of Vietnamese word
    if lower.starts_with("pr") && lower.len() > 3 {
        return true;
    }
    
    // -tion, -sion endings (clearly English)
    if lower.ends_with("tion") || lower.ends_with("sion") {
        return true;
    }
    
    false
}

// ============================================================
// HELPER FUNCTIONS
// ============================================================

/// Check if string has any vowel
fn has_vowel(chars: &[char]) -> bool {
    chars.iter().any(|&c| crate::chars::is_vowel(c))
}

/// Check if string has Vietnamese diacritics (horn, circumflex, breve)
fn has_vietnamese_diacritics(s: &str) -> bool {
    for c in s.chars() {
        if let Some(&(_, modifier, tone)) = REVERSE_MAP.get(&c) {
            if modifier != VowelMod::None || tone != crate::chars::ToneMark::None {
                return true;
            }
        }
        // Direct check for Vietnamese special chars
        if matches!(c, 'ư' | 'ơ' | 'â' | 'ê' | 'ô' | 'ă' | 'đ') {
            return true;
        }
    }
    false
}

/// Check if this is a valid Vietnamese typing sequence
/// e.g., "dodo" could be typing "đô" (d→o→d→o where second d triggers stroke)
fn is_valid_typing_sequence(s: &str) -> bool {
    let lower = s.to_lowercase();
    
    // Patterns that look foreign but are valid Vietnamese typing sequences:
    
    // "dodo", "dodi" - typing đô, đồ, etc. with dd for đ
    if lower.starts_with("do") && lower.chars().nth(2) == Some('d') {
        return true;
    }
    
    // "soso", "lolo" - typing có repeated consonants for emphasis
    // But these could also be valid Vietnamese with tones added later
    let chars: Vec<char> = lower.chars().collect();
    if chars.len() >= 4 {
        // Check if it's a simple repetition pattern like "xoxo"
        if chars[0] == chars[2] && chars[1] == chars[3] {
            // If first char is consonant and second is vowel, could be valid
            if !crate::chars::is_vowel(chars[0]) && crate::chars::is_vowel(chars[1]) {
                return true;
            }
        }
    }
    
    false
}

/// Check for foreign word patterns
fn is_foreign_pattern(s: &str) -> bool {
    // Consonant clusters that don't exist in Vietnamese
    for cluster in FOREIGN_CLUSTERS {
        if s.contains(cluster) {
            // But allow "tr" at the start (Vietnamese "tr")
            if *cluster == "tr" && s.starts_with("tr") {
                continue;
            }
            return true;
        }
    }
    false
}

/// Parse syllable into (initial, vowel, final) components
fn parse_syllable(s: &str) -> Option<(&str, &str, &str)> {
    // Try longest initial first
    let mut initials: Vec<&&str> = VALID_INITIALS.iter().collect();
    initials.sort_by(|a, b| b.len().cmp(&a.len()));
    
    for initial in initials {
        if s.starts_with(initial) {
            let rest = &s[initial.len()..];
            
            // Try to find vowel + final
            if let Some((vowel, final_c)) = parse_vowel_final(rest) {
                return Some((initial, vowel, final_c));
            }
        }
    }
    
    None
}

/// Parse vowel and final consonant from string
fn parse_vowel_final(s: &str) -> Option<(&str, &str)> {
    if s.is_empty() {
        return None;
    }
    
    // Try longest final first
    let mut finals: Vec<&&str> = VALID_FINALS.iter().collect();
    finals.sort_by(|a, b| b.len().cmp(&a.len()));
    
    for final_c in finals {
        if s.ends_with(final_c) {
            let vowel_len = s.len() - final_c.len();
            if vowel_len > 0 {
                let vowel = &s[..vowel_len];
                // Check if all chars in vowel part are vowels (ignoring tones)
                if vowel.chars().all(|c| crate::chars::is_vowel(c) || is_glide(c)) {
                    return Some((vowel, final_c));
                }
            } else if final_c.is_empty() {
                // No final, entire string is vowel
                if s.chars().all(|c| crate::chars::is_vowel(c) || is_glide(c)) {
                    return Some((s, ""));
                }
            }
        }
    }
    
    None
}

/// Check if character is a glide (y/w used as consonant)
fn is_glide(c: char) -> bool {
    matches!(c, 'y' | 'w')
}

/// Normalize vowel pattern (remove tones for pattern matching)
fn normalize_vowel(vowel: &str) -> String {
    vowel.chars().map(chars::get_base).collect()
}

/// Check if vowel pattern is valid
fn is_valid_vowel_pattern(vowel_base: &str) -> bool {
    // Single vowels are always valid
    if vowel_base.len() == 1 {
        return true;
    }
    
    // Check against known patterns
    VALID_VOWEL_PATTERNS.contains(&vowel_base)
}

/// Check Vietnamese spelling rules
fn check_spelling_rules(initial: &str, vowel: &str) -> bool {
    let vowel_first = vowel.chars().next().unwrap_or(' ');
    let vowel_base = chars::get_base(vowel_first);
    
    match initial {
        // "c" can only precede non e/ê/i/y vowels (use "k" for those)
        "c" => !matches!(vowel_base, 'e' | 'ê' | 'i' | 'y'),
        
        // "k" can only precede e/ê/i/y vowels
        "k" => matches!(vowel_base, 'e' | 'ê' | 'i' | 'y'),
        
        // "g" cannot precede e/ê/i (use "gh" for those)
        "g" => !matches!(vowel_base, 'e' | 'ê' | 'i'),
        
        // "gh" can only precede e/ê/i
        "gh" => matches!(vowel_base, 'e' | 'ê' | 'i'),
        
        // "ng" cannot precede e/ê/i (use "ngh" for those)
        "ng" => !matches!(vowel_base, 'e' | 'ê' | 'i'),
        
        // "ngh" can only precede e/ê/i
        "ngh" => matches!(vowel_base, 'e' | 'ê' | 'i'),
        
        _ => true,
    }
}

/// Check if character sequence could become valid Vietnamese
/// Used during typing to allow incomplete words
pub fn is_potentially_valid(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    
    let lower = s.to_lowercase();
    
    // Check if it's clearly a foreign word
    if is_foreign_word_pattern(&lower, None) {
        return false;
    }
    
    // Check if any valid syllable could start with this
    for initial in VALID_INITIALS {
        if initial.starts_with(&lower) || lower.starts_with(initial) {
            return true;
        }
    }
    
    // Could be mid-vowel typing
    true
}

/// Check if word ends at a valid boundary
pub fn is_word_boundary(ch: char) -> bool {
    ch.is_whitespace() 
        || matches!(ch, '.' | ',' | '!' | '?' | ':' | ';' | '"' | '\'' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '/' | '\\' | '=' | '+' | '-' | '*' | '@' | '#' | '$' | '%' | '^' | '&' | '|' | '~' | '`')
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_syllables() {
        assert!(is_valid_syllable("an"));
        assert!(is_valid_syllable("em"));
        assert!(is_valid_syllable("anh"));
        assert!(is_valid_syllable("nam"));
        assert!(is_valid_syllable("xin"));
    }

    #[test]
    fn test_invalid_syllables() {
        assert!(!is_valid_syllable("xyz"));
        assert!(!is_valid_syllable("bcd"));
        assert!(!is_valid_syllable("mmm"));
    }

    #[test]
    fn test_foreign_word_detection() {
        assert!(is_foreign_word_pattern("programming", None));
        assert!(is_foreign_word_pattern("describe", Some('s')));
        assert!(is_foreign_word_pattern("spectrum", None));
        assert!(!is_foreign_word_pattern("việt", None)); // Has horn
        assert!(!is_foreign_word_pattern("xin", None)); // Valid Vietnamese
    }

    #[test]
    fn test_spelling_rules() {
        // c vs k
        assert!(check_spelling_rules("c", "a"));     // ca ✓
        assert!(!check_spelling_rules("c", "e"));    // ce ✗ (should be ke)
        assert!(check_spelling_rules("k", "e"));     // ke ✓
        
        // g vs gh
        assert!(check_spelling_rules("g", "a"));     // ga ✓
        assert!(!check_spelling_rules("g", "e"));    // ge ✗ (should be ghe)
        assert!(check_spelling_rules("gh", "e"));    // ghe ✓
    }

    #[test]
    fn test_word_boundary() {
        assert!(is_word_boundary(' '));
        assert!(is_word_boundary('.'));
        assert!(is_word_boundary('='));  // Programming
        assert!(is_word_boundary('{'));  // Code
        assert!(!is_word_boundary('a'));
    }
}
