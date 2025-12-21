//! Character Transformation
//!
//! Handles Vietnamese character transformations:
//! - Adding/removing tone marks
//! - Adding/removing vowel modifiers
//! - Converting d to đ

use crate::chars::{self, ToneMark, VowelMod, CHAR_MAP, REVERSE_MAP};

/// Transform result
#[derive(Debug, Clone, PartialEq)]
pub enum TransformResult {
    /// Character was transformed
    Success { 
        old_char: char, 
        new_char: char,
        position: usize,
    },
    /// No transformation needed (passthrough)
    Passthrough,
    /// Cannot transform (invalid)
    Invalid,
}

/// Apply tone mark to a vowel
pub fn apply_tone(ch: char, tone: ToneMark) -> Option<char> {
    chars::with_tone(ch, tone)
}

/// Apply vowel modifier (circumflex, horn, breve)
pub fn apply_modifier(ch: char, modifier: VowelMod) -> Option<char> {
    chars::with_modifier(ch, modifier)
}

/// Remove all diacritics from a character
pub fn remove_diacritics(ch: char) -> char {
    let lower = ch.to_ascii_lowercase();
    
    if let Some(&(base, _, _)) = REVERSE_MAP.get(&lower) {
        if ch.is_uppercase() {
            base.to_ascii_uppercase()
        } else {
            base
        }
    } else {
        // Special case: đ -> d
        match ch {
            'đ' => 'd',
            'Đ' => 'D',
            _ => ch,
        }
    }
}

/// Convert d to đ or đ to d (toggle)
pub fn toggle_stroke(ch: char) -> char {
    match ch {
        'd' => 'đ',
        'D' => 'Đ',
        'đ' => 'd',
        'Đ' => 'D',
        _ => ch,
    }
}

/// Find the best vowel position for tone placement
/// Based on Vietnamese rules:
/// 1. If there's only one vowel, put tone on it
/// 2. If there are two vowels:
///    - If first vowel has modifier (ư, ơ, â, ê, ô, ă), put tone on it
///    - Otherwise put on second vowel
/// 3. If there are three vowels, put tone on the middle one
pub fn find_tone_position(chars: &[char]) -> Option<usize> {
    let vowel_positions: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter(|(_, &c)| chars::is_vowel(c))
        .map(|(i, _)| i)
        .collect();
    
    match vowel_positions.len() {
        0 => None,
        1 => Some(vowel_positions[0]),
        2 => {
            let first = chars[vowel_positions[0]];
            // Check if first vowel has modifier
            if let Some(&(_, modifier, _)) = REVERSE_MAP.get(&first.to_ascii_lowercase()) {
                if modifier != VowelMod::None {
                    return Some(vowel_positions[0]);
                }
            }
            // Default to second vowel
            Some(vowel_positions[1])
        }
        _ => {
            // Three or more vowels: middle one
            Some(vowel_positions[vowel_positions.len() / 2])
        }
    }
}

/// Find vowel to apply modifier to
/// For circumflex: a, e, o
/// For horn: o, u
/// For breve: a
pub fn find_modifier_position(chars: &[char], modifier: VowelMod) -> Option<usize> {
    let valid_bases: &[char] = match modifier {
        VowelMod::Circumflex => &['a', 'e', 'o'],
        VowelMod::Horn => &['o', 'u'],
        VowelMod::Breve => &['a'],
        VowelMod::None => return None,
    };
    
    // Find last matching vowel
    for i in (0..chars.len()).rev() {
        let c = chars[i].to_ascii_lowercase();
        let base = chars::get_base(c);
        if valid_bases.contains(&base) {
            return Some(i);
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_tone() {
        assert_eq!(apply_tone('a', ToneMark::Acute), Some('á'));
        assert_eq!(apply_tone('â', ToneMark::Grave), Some('ầ'));
        assert_eq!(apply_tone('A', ToneMark::Tilde), Some('Ã'));
    }

    #[test]
    fn test_apply_modifier() {
        assert_eq!(apply_modifier('a', VowelMod::Circumflex), Some('â'));
        assert_eq!(apply_modifier('o', VowelMod::Horn), Some('ơ'));
        assert_eq!(apply_modifier('a', VowelMod::Breve), Some('ă'));
    }

    #[test]
    fn test_remove_diacritics() {
        assert_eq!(remove_diacritics('á'), 'a');
        assert_eq!(remove_diacritics('ầ'), 'a');
        assert_eq!(remove_diacritics('đ'), 'd');
        assert_eq!(remove_diacritics('Đ'), 'D');
    }

    #[test]
    fn test_toggle_stroke() {
        assert_eq!(toggle_stroke('d'), 'đ');
        assert_eq!(toggle_stroke('đ'), 'd');
        assert_eq!(toggle_stroke('D'), 'Đ');
    }

    #[test]
    fn test_find_tone_position() {
        // Single vowel
        assert_eq!(find_tone_position(&['v', 'a', 'n']), Some(1));
        
        // Two vowels
        assert_eq!(find_tone_position(&['t', 'i', 'e', 'n']), Some(2));
        
        // Three vowels
        assert_eq!(find_tone_position(&['u', 'y', 'e', 'n']), Some(2));
    }
}
