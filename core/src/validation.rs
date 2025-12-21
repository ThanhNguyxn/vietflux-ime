//! Vietnamese Syllable Validation
//!
//! Validates if a string is a valid Vietnamese syllable.
//! Based on Vietnamese phonology rules.

use crate::chars;

/// Valid initial consonants (phụ âm đầu)
const VALID_INITIALS: &[&str] = &[
    "", "b", "c", "ch", "d", "đ", "g", "gh", "gi", "h", "k", "kh",
    "l", "m", "n", "ng", "ngh", "nh", "p", "ph", "q", "r", "s", "t",
    "th", "tr", "v", "x",
];

/// Valid final consonants (phụ âm cuối)  
const VALID_FINALS: &[&str] = &[
    "", "c", "ch", "m", "n", "ng", "nh", "p", "t",
];

/// Valid vowel clusters (nguyên âm)
const VALID_VOWELS: &[&str] = &[
    // Single vowels
    "a", "ă", "â", "e", "ê", "i", "o", "ô", "ơ", "u", "ư", "y",
    // Diphthongs
    "ai", "ao", "au", "ay", "âu", "ây", "eo", "êu",
    "ia", "iê", "iu", "oa", "oă", "oe", "oi", "oo", "ôi",
    "ơi", "ua", "uâ", "uê", "ui", "uo", "uô", "uơ", "ưa", "ưi", "ưu",
    // Triphthongs
    "iêu", "oai", "oay", "oeo", "uây", "uôi", "ươi", "ươu", "yêu",
    // With u/o prefix (qu-, etc.)
    "uya", "uyê", "uyu",
];

/// Check if a string is a valid Vietnamese syllable
pub fn is_valid_syllable(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    
    let lower = s.to_lowercase();
    
    // Try to parse syllable structure: Initial + Vowel + Final
    for initial in VALID_INITIALS {
        if lower.starts_with(initial) {
            let rest = &lower[initial.len()..];
            
            for vowel in VALID_VOWELS {
                if rest.starts_with(vowel) || starts_with_toned(rest, vowel) {
                    let after_vowel = skip_vowel(rest, vowel);
                    
                    for final_c in VALID_FINALS {
                        if after_vowel == *final_c {
                            return true;
                        }
                    }
                }
            }
        }
    }
    
    false
}

/// Check if string starts with vowel (ignoring tone marks)
fn starts_with_toned(s: &str, vowel: &str) -> bool {
    let s_base: String = s.chars().map(chars::get_base).collect();
    let vowel_base: String = vowel.chars().map(chars::get_base).collect();
    s_base.starts_with(&vowel_base)
}

/// Skip vowel part and return remaining string
fn skip_vowel<'a>(s: &'a str, vowel: &str) -> &'a str {
    let mut chars_to_skip = vowel.chars().count();
    let mut char_indices = s.char_indices();
    
    while chars_to_skip > 0 {
        if char_indices.next().is_none() {
            return "";
        }
        chars_to_skip -= 1;
    }
    
    if let Some((idx, _)) = char_indices.next() {
        &s[idx..]
    } else {
        ""
    }
}

/// Check if character sequence could become valid Vietnamese
/// Used during typing to allow incomplete words
pub fn is_potentially_valid(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    
    let lower = s.to_lowercase();
    
    // Check if any valid syllable starts with this
    for initial in VALID_INITIALS {
        if initial.starts_with(&lower) || lower.starts_with(initial) {
            let rest = if lower.len() > initial.len() {
                &lower[initial.len()..]
            } else {
                return true; // Still typing initial consonant
            };
            
            for vowel in VALID_VOWELS {
                if vowel.starts_with(rest) || starts_with_toned(rest, vowel) {
                    return true;
                }
            }
        }
    }
    
    false
}

/// Check if word ends at a valid boundary (space, punctuation, etc.)
pub fn is_word_boundary(ch: char) -> bool {
    ch.is_whitespace() || matches!(ch, '.' | ',' | '!' | '?' | ':' | ';' | '"' | '\'' | '(' | ')' | '[' | ']')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_syllables() {
        assert!(is_valid_syllable("an"));
        assert!(is_valid_syllable("em"));
        assert!(is_valid_syllable("anh"));
        assert!(is_valid_syllable("viet"));
        assert!(is_valid_syllable("nam"));
        assert!(is_valid_syllable("nguyen"));
        assert!(is_valid_syllable("xin"));
        assert!(is_valid_syllable("chao"));
    }

    #[test]
    fn test_invalid_syllables() {
        assert!(!is_valid_syllable("xyz"));
        assert!(!is_valid_syllable("bcd"));
        assert!(!is_valid_syllable("mmm"));
    }

    #[test]
    fn test_potentially_valid() {
        assert!(is_potentially_valid("vi"));  // typing "viet"
        assert!(is_potentially_valid("ng"));  // typing "nguyen"
        assert!(is_potentially_valid(""));
    }

    #[test]
    fn test_word_boundary() {
        assert!(is_word_boundary(' '));
        assert!(is_word_boundary('.'));
        assert!(!is_word_boundary('a'));
    }
}
