//! Vietnamese Syllable Validator
//!
//! Validates Vietnamese syllable structure using whitelist-based patterns.

use super::data::keycodes::{is_vowel, is_consonant};
use super::data::rules::{
    is_valid_initial, is_valid_final, is_valid_vowel_pattern,
    SPELLING_RULES,
};
use super::buffer::CompositionBuffer;

/// Validation result
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    NoVowel,
    InvalidInitial,
    InvalidFinal,
    InvalidSpelling,
    InvalidVowelPattern,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }
}

/// Parsed syllable structure
#[derive(Debug, Default)]
pub struct SyllableStructure {
    pub initial: Vec<usize>,       // Initial consonant positions
    pub vowels: Vec<usize>,        // Vowel positions
    pub final_consonant: Vec<usize>, // Final consonant positions
}

/// Parse buffer into syllable structure
pub fn parse_syllable(buffer_keys: &[u16]) -> SyllableStructure {
    let mut structure = SyllableStructure::default();
    let mut state = ParseState::Initial;

    for (i, &key) in buffer_keys.iter().enumerate() {
        let is_v = is_vowel(key);
        let is_c = is_consonant(key);

        match state {
            ParseState::Initial => {
                if is_v {
                    structure.vowels.push(i);
                    state = ParseState::Vowel;
                } else if is_c {
                    structure.initial.push(i);
                }
            }
            ParseState::Vowel => {
                if is_v {
                    structure.vowels.push(i);
                } else if is_c {
                    structure.final_consonant.push(i);
                    state = ParseState::Final;
                }
            }
            ParseState::Final => {
                if is_c {
                    structure.final_consonant.push(i);
                } else if is_v {
                    // Vowel after final consonant - unusual but handle it
                    structure.vowels.push(i);
                    state = ParseState::Vowel;
                }
            }
        }
    }

    structure
}

enum ParseState {
    Initial,
    Vowel,
    Final,
}

/// Validate buffer as Vietnamese syllable
pub fn validate(buffer: &CompositionBuffer) -> ValidationResult {
    let keys = buffer.keys();
    validate_keys(&keys)
}

/// Validate keycodes as Vietnamese syllable
pub fn validate_keys(buffer_keys: &[u16]) -> ValidationResult {
    if buffer_keys.is_empty() {
        return ValidationResult::NoVowel;
    }

    let syllable = parse_syllable(buffer_keys);

    // Rule 1: Must have at least one vowel
    if syllable.vowels.is_empty() {
        return ValidationResult::NoVowel;
    }

    // Rule 2: Initial consonant must be valid
    let initial_keys: Vec<u16> = syllable.initial.iter().map(|&i| buffer_keys[i]).collect();
    if !is_valid_initial(&initial_keys) {
        return ValidationResult::InvalidInitial;
    }

    // Rule 3: Spelling rules (c/k, g/gh, ng/ngh)
    if !syllable.initial.is_empty() && !syllable.vowels.is_empty() {
        let first_vowel = buffer_keys[syllable.vowels[0]];
        for &(consonant, forbidden_vowels, _msg) in SPELLING_RULES {
            if initial_keys == consonant && forbidden_vowels.contains(&first_vowel) {
                return ValidationResult::InvalidSpelling;
            }
        }
    }

    // Rule 4: Final consonant must be valid
    let final_keys: Vec<u16> = syllable.final_consonant.iter().map(|&i| buffer_keys[i]).collect();
    if !is_valid_final(&final_keys) {
        return ValidationResult::InvalidFinal;
    }

    // Rule 5: Vowel pattern must be valid
    let vowel_keys: Vec<u16> = syllable.vowels.iter().map(|&i| buffer_keys[i]).collect();
    if vowel_keys.len() >= 2 && !is_valid_vowel_pattern(&vowel_keys) {
        return ValidationResult::InvalidVowelPattern;
    }

    ValidationResult::Valid
}

/// Quick check if buffer is valid Vietnamese
pub fn is_valid(buffer_keys: &[u16]) -> bool {
    validate_keys(buffer_keys).is_valid()
}

/// Check if buffer could be valid after transformation
/// (More lenient - allows intermediate states like "aa")
pub fn is_valid_for_transform(buffer_keys: &[u16]) -> bool {
    if buffer_keys.is_empty() {
        return false;
    }

    let syllable = parse_syllable(buffer_keys);

    // Must have vowel
    if syllable.vowels.is_empty() {
        return false;
    }

    // Initial must be valid
    let initial_keys: Vec<u16> = syllable.initial.iter().map(|&i| buffer_keys[i]).collect();
    if !is_valid_initial(&initial_keys) {
        return false;
    }

    // Spelling must be valid
    if !syllable.initial.is_empty() && !syllable.vowels.is_empty() {
        let first_vowel = buffer_keys[syllable.vowels[0]];
        for &(consonant, forbidden_vowels, _msg) in SPELLING_RULES {
            if initial_keys == consonant && forbidden_vowels.contains(&first_vowel) {
                return false;
            }
        }
    }

    // Final must be valid
    let final_keys: Vec<u16> = syllable.final_consonant.iter().map(|&i| buffer_keys[i]).collect();
    if !is_valid_final(&final_keys) {
        return false;
    }

    // Skip vowel pattern check (allow intermediate states)
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::data::keycodes::from_char;

    fn keys_from_str(s: &str) -> Vec<u16> {
        s.chars().filter_map(from_char).collect()
    }

    #[test]
    fn test_valid_syllables() {
        assert!(is_valid(&keys_from_str("ba")));
        assert!(is_valid(&keys_from_str("an")));
        assert!(is_valid(&keys_from_str("em")));
        assert!(is_valid(&keys_from_str("viet")));
        assert!(is_valid(&keys_from_str("nam")));
    }

    #[test]
    fn test_invalid_no_vowel() {
        assert!(!is_valid(&keys_from_str("bcd")));
        assert!(!is_valid(&keys_from_str("xyz")));
    }

    #[test]
    fn test_invalid_initial() {
        assert!(!is_valid(&keys_from_str("bla"))); // bl is invalid
        assert!(!is_valid(&keys_from_str("clau"))); // cl is invalid
    }

    #[test]
    fn test_spelling_rules() {
        assert!(!is_valid(&keys_from_str("ci"))); // Use 'k' before i
        assert!(!is_valid(&keys_from_str("ce"))); // Use 'k' before e
        assert!(is_valid(&keys_from_str("ki")));
        assert!(is_valid(&keys_from_str("ke")));
    }
}
