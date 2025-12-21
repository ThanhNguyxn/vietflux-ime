//! Character Transformation
//!
//! Handles Vietnamese character transformations with smart features:
//! - Adding/removing tone marks
//! - Adding/removing vowel modifiers
//! - UO Compound handling (dươc)
//! - Tone repositioning (hoaf → hoà)
//! - Double mark undo (ass → as)

use crate::chars::{self, ToneMark, VowelMod, REVERSE_MAP};

/// Transform result with details
#[derive(Debug, Clone, PartialEq)]
pub struct TransformResult {
    /// Whether transformation occurred
    pub success: bool,
    /// Position of transformed character
    pub position: Option<usize>,
    /// Number of characters affected
    pub chars_affected: usize,
    /// Whether this was an undo operation
    pub was_undo: bool,
}

impl TransformResult {
    pub fn success(position: usize, chars_affected: usize) -> Self {
        Self {
            success: true,
            position: Some(position),
            chars_affected,
            was_undo: false,
        }
    }
    
    pub fn undo(position: usize) -> Self {
        Self {
            success: true,
            position: Some(position),
            chars_affected: 1,
            was_undo: true,
        }
    }
    
    pub fn none() -> Self {
        Self {
            success: false,
            position: None,
            chars_affected: 0,
            was_undo: false,
        }
    }
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

/// Get the current tone of a character
pub fn get_tone(ch: char) -> ToneMark {
    let lower = ch.to_ascii_lowercase();
    if let Some(&(_, _, tone)) = REVERSE_MAP.get(&lower) {
        tone
    } else {
        ToneMark::None
    }
}

/// Get the current modifier of a character
pub fn get_modifier(ch: char) -> VowelMod {
    let lower = ch.to_ascii_lowercase();
    if let Some(&(_, modifier, _)) = REVERSE_MAP.get(&lower) {
        modifier
    } else {
        VowelMod::None
    }
}

/// Check if character has any tone mark
pub fn has_tone(ch: char) -> bool {
    get_tone(ch) != ToneMark::None
}

/// Check if character has any modifier
pub fn has_modifier(ch: char) -> bool {
    get_modifier(ch) != VowelMod::None
}

// ============================================================
// SMART TONE POSITIONING
// ============================================================

/// Find the best vowel position for tone placement
/// Based on Vietnamese linguistic rules:
/// 
/// Rules (in order of priority):
/// 1. If word contains ơ, ư, ă, â, ê, ô → put tone there
/// 2. For ươ compound: put tone on ơ (second vowel)
/// 3. If there's only one vowel → put tone on it
/// 4. If there are two vowels:
///    - If vowel cluster is: ia, ua, ưa, oa, oe, ue, uy → put on first
///    - Otherwise → put on second (last)
/// 5. If there are three vowels → put tone on the middle one
pub fn find_tone_position(chars: &[char], vowel_indices: &[usize]) -> Option<usize> {
    if vowel_indices.is_empty() {
        return None;
    }
    
    // Special case: ươ compound - tone goes on ơ
    if vowel_indices.len() >= 2 {
        for i in 0..vowel_indices.len() - 1 {
            let first_idx = vowel_indices[i];
            let second_idx = vowel_indices[i + 1];
            
            let first = chars[first_idx].to_ascii_lowercase();
            let second = chars[second_idx].to_ascii_lowercase();
            
            let first_base = chars::get_base(first);
            let second_base = chars::get_base(second);
            let first_mod = get_modifier(chars[first_idx]);
            let second_mod = get_modifier(chars[second_idx]);
            
            // ươ compound: tone goes on ơ
            if (first_base == 'u' && first_mod == VowelMod::Horn) ||
               (second_base == 'o' && second_mod == VowelMod::Horn) {
                // If we have ư + ơ, tone on ơ
                if first_base == 'u' && second_base == 'o' {
                    return Some(second_idx);
                }
            }
        }
    }
    
    // Rule 1: Check for modified vowels first (ơ, ư, ă, â, ê, ô)
    for &idx in vowel_indices {
        if get_modifier(chars[idx]) != VowelMod::None {
            return Some(idx);
        }
    }
    
    match vowel_indices.len() {
        // Rule 3: Single vowel
        1 => Some(vowel_indices[0]),
        
        // Rule 4: Two vowels
        2 => {
            let first = chars[vowel_indices[0]].to_ascii_lowercase();
            let second = chars[vowel_indices[1]].to_ascii_lowercase();
            
            let first_base = chars::get_base(first);
            let second_base = chars::get_base(second);
            
            // Special pairs where tone goes on first vowel
            let first_tone_pairs = [
                ('i', 'a'), ('i', 'ê'), // ia, iê
                ('u', 'a'), ('u', 'ô'), ('u', 'ơ'), // ua, uô, uơ
                ('ư', 'a'), ('ư', 'ơ'), // ưa, ươ  
                ('o', 'a'), ('o', 'e'), // oa, oe
            ];
            
            // Check if this is a "first vowel" pair
            for (f, s) in first_tone_pairs {
                if first_base == f && second_base == s {
                    return Some(vowel_indices[0]);
                }
            }
            
            // Default: tone on second (last) vowel
            Some(vowel_indices[1])
        }
        
        // Rule 5: Three or more vowels - middle one
        _ => Some(vowel_indices[vowel_indices.len() / 2]),
    }
}

/// Find all vowel indices in a char slice
pub fn find_vowel_indices(chars: &[char]) -> Vec<usize> {
    chars
        .iter()
        .enumerate()
        .filter(|(_, &c)| chars::is_vowel(c))
        .map(|(i, _)| i)
        .collect()
}

// ============================================================
// UO COMPOUND HANDLING
// ============================================================

/// Apply UO compound transformation
/// When applying horn to "uo", both u and o should become ư and ơ
/// e.g., "duoc" + horn → "dươc"
pub fn apply_uo_compound(chars: &mut [char], start_idx: usize) -> TransformResult {
    if start_idx + 1 >= chars.len() {
        return TransformResult::none();
    }
    
    let first = chars[start_idx].to_ascii_lowercase();
    let second = chars[start_idx + 1].to_ascii_lowercase();
    
    let first_base = chars::get_base(first);
    let second_base = chars::get_base(second);
    
    // Check for "uo" pattern
    if first_base == 'u' && second_base == 'o' {
        // Get current tones
        let first_tone = get_tone(chars[start_idx]);
        let second_tone = get_tone(chars[start_idx + 1]);
        
        // Apply horn to both
        if let Some(new_first) = apply_modifier_with_tone('u', VowelMod::Horn, first_tone) {
            if let Some(new_second) = apply_modifier_with_tone('o', VowelMod::Horn, second_tone) {
                // Preserve case
                chars[start_idx] = if chars[start_idx].is_uppercase() {
                    new_first.to_ascii_uppercase()
                } else {
                    new_first
                };
                chars[start_idx + 1] = if chars[start_idx + 1].is_uppercase() {
                    new_second.to_ascii_uppercase()
                } else {
                    new_second
                };
                
                return TransformResult::success(start_idx, 2);
            }
        }
    }
    
    TransformResult::none()
}

/// Apply modifier while preserving existing tone
fn apply_modifier_with_tone(base: char, modifier: VowelMod, tone: ToneMark) -> Option<char> {
    CHAR_MAP.get(&(base, modifier, tone)).copied()
}

// ============================================================
// DOUBLE MARK UNDO
// ============================================================

/// Check if applying the same tone again should undo it
/// e.g., "ás" → typing 's' again → "as" (remove tone, keep raw 's')
pub fn should_undo_tone(current_char: char, pressing_tone: ToneMark) -> bool {
    get_tone(current_char) == pressing_tone && pressing_tone != ToneMark::None
}

/// Check if applying the same modifier again should undo it
/// e.g., "â" → typing 'a' again → "aa" (circumflex removed, get raw 'a')
pub fn should_undo_modifier(current_char: char, pressing_modifier: VowelMod) -> bool {
    get_modifier(current_char) == pressing_modifier && pressing_modifier != VowelMod::None
}

/// Remove tone from character, return (base_with_modifier, was_modified)
pub fn remove_tone(ch: char) -> (char, bool) {
    let current_tone = get_tone(ch);
    if current_tone == ToneMark::None {
        return (ch, false);
    }
    
    if let Some(without_tone) = apply_tone(chars::get_base(ch), ToneMark::None) {
        // Preserve modifier if any
        let modifier = get_modifier(ch);
        if modifier != VowelMod::None {
            if let Some(with_mod) = apply_modifier(without_tone, modifier) {
                let result = if ch.is_uppercase() {
                    with_mod.to_ascii_uppercase()
                } else {
                    with_mod
                };
                return (result, true);
            }
        }
        
        let result = if ch.is_uppercase() {
            without_tone.to_ascii_uppercase()
        } else {
            without_tone
        };
        return (result, true);
    }
    
    (ch, false)
}

/// Remove modifier from character, return (base_with_tone, was_modified)
pub fn remove_modifier(ch: char) -> (char, bool) {
    let current_mod = get_modifier(ch);
    if current_mod == VowelMod::None {
        return (ch, false);
    }
    
    let base = chars::get_base(ch);
    let tone = get_tone(ch);
    
    if let Some(without_mod) = CHAR_MAP.get(&(base, VowelMod::None, tone)).copied() {
        let result = if ch.is_uppercase() {
            without_mod.to_ascii_uppercase()
        } else {
            without_mod
        };
        return (result, true);
    }
    
    (ch, false)
}

// ============================================================
// FIND MODIFIER POSITION
// ============================================================

/// Find vowel to apply modifier to
/// For circumflex: a, e, o
/// For horn: o, u (searches backwards for last matching vowel)
/// For breve: a only
pub fn find_modifier_position(chars: &[char], modifier: VowelMod) -> Option<usize> {
    let valid_bases: &[char] = match modifier {
        VowelMod::Circumflex => &['a', 'e', 'o'],
        VowelMod::Horn => &['o', 'u'],
        VowelMod::Breve => &['a'],
        VowelMod::None => return None,
    };
    
    // Find last matching vowel (right to left)
    for i in (0..chars.len()).rev() {
        let c = chars[i].to_ascii_lowercase();
        let base = chars::get_base(c);
        if valid_bases.contains(&base) {
            return Some(i);
        }
    }
    
    None
}

// ============================================================
// TESTS
// ============================================================

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
    fn test_tone_position_single() {
        // Single vowel: "van" - tone on 'a'
        let chars: Vec<char> = "van".chars().collect();
        let vowels = find_vowel_indices(&chars);
        assert_eq!(find_tone_position(&chars, &vowels), Some(1));
    }
    
    #[test]
    fn test_tone_position_two() {
        // "tien" - tone on 'e' (second vowel)
        let chars: Vec<char> = "tien".chars().collect();
        let vowels = find_vowel_indices(&chars);
        assert_eq!(find_tone_position(&chars, &vowels), Some(2));
    }
    
    #[test]
    fn test_tone_position_three() {
        // "uyen" - tone on 'y' (middle)
        let chars: Vec<char> = "uyen".chars().collect();
        let vowels = find_vowel_indices(&chars);
        assert_eq!(find_tone_position(&chars, &vowels), Some(1));
    }

    #[test]
    fn test_should_undo() {
        assert!(should_undo_tone('á', ToneMark::Acute));
        assert!(!should_undo_tone('á', ToneMark::Grave));
        assert!(!should_undo_tone('a', ToneMark::Acute));
    }
    
    #[test]
    fn test_uo_compound() {
        let mut chars: Vec<char> = "duoc".chars().collect();
        let result = apply_uo_compound(&mut chars, 1);
        assert!(result.success);
        assert_eq!(chars[1], 'ư');
        assert_eq!(chars[2], 'ơ');
    }
}
