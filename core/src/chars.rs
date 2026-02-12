//! Vietnamese Character Data
//!
//! Contains all Vietnamese character mappings, tones, and marks.
//! Based on Unicode Vietnamese block.

use std::collections::HashMap;
use std::sync::LazyLock;

/// Vietnamese vowels with their base forms
pub const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u', 'y', 'ă', 'â', 'ê', 'ô', 'ơ', 'ư'];

/// Vietnamese consonants
pub const CONSONANTS: &[char] = &[
    'b', 'c', 'd', 'đ', 'g', 'h', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'x',
];

/// Tone marks (dấu)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToneMark {
    None,  // không dấu
    Acute, // sắc (á)
    Grave, // huyền (à)
    Hook,  // hỏi (ả)
    Tilde, // ngã (ã)
    Dot,   // nặng (ạ)
}

/// Vowel modifiers (mũ/móc)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VowelMod {
    None,       // a, e, o, u
    Circumflex, // â, ê, ô (mũ)
    Horn,       // ơ, ư (móc)
    Breve,      // ă (trăng)
}

/// Complete Vietnamese character with all components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VietChar {
    pub base: char, // Base vowel (a, e, i, o, u, y)
    pub modifier: VowelMod,
    pub tone: ToneMark,
}

/// Mapping from base vowel + modifier + tone to final character
pub static CHAR_MAP: LazyLock<HashMap<(char, VowelMod, ToneMark), char>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // A variants
    map.insert(('a', VowelMod::None, ToneMark::None), 'a');
    map.insert(('a', VowelMod::None, ToneMark::Acute), 'á');
    map.insert(('a', VowelMod::None, ToneMark::Grave), 'à');
    map.insert(('a', VowelMod::None, ToneMark::Hook), 'ả');
    map.insert(('a', VowelMod::None, ToneMark::Tilde), 'ã');
    map.insert(('a', VowelMod::None, ToneMark::Dot), 'ạ');

    map.insert(('a', VowelMod::Circumflex, ToneMark::None), 'â');
    map.insert(('a', VowelMod::Circumflex, ToneMark::Acute), 'ấ');
    map.insert(('a', VowelMod::Circumflex, ToneMark::Grave), 'ầ');
    map.insert(('a', VowelMod::Circumflex, ToneMark::Hook), 'ẩ');
    map.insert(('a', VowelMod::Circumflex, ToneMark::Tilde), 'ẫ');
    map.insert(('a', VowelMod::Circumflex, ToneMark::Dot), 'ậ');

    map.insert(('a', VowelMod::Breve, ToneMark::None), 'ă');
    map.insert(('a', VowelMod::Breve, ToneMark::Acute), 'ắ');
    map.insert(('a', VowelMod::Breve, ToneMark::Grave), 'ằ');
    map.insert(('a', VowelMod::Breve, ToneMark::Hook), 'ẳ');
    map.insert(('a', VowelMod::Breve, ToneMark::Tilde), 'ẵ');
    map.insert(('a', VowelMod::Breve, ToneMark::Dot), 'ặ');

    // E variants
    map.insert(('e', VowelMod::None, ToneMark::None), 'e');
    map.insert(('e', VowelMod::None, ToneMark::Acute), 'é');
    map.insert(('e', VowelMod::None, ToneMark::Grave), 'è');
    map.insert(('e', VowelMod::None, ToneMark::Hook), 'ẻ');
    map.insert(('e', VowelMod::None, ToneMark::Tilde), 'ẽ');
    map.insert(('e', VowelMod::None, ToneMark::Dot), 'ẹ');

    map.insert(('e', VowelMod::Circumflex, ToneMark::None), 'ê');
    map.insert(('e', VowelMod::Circumflex, ToneMark::Acute), 'ế');
    map.insert(('e', VowelMod::Circumflex, ToneMark::Grave), 'ề');
    map.insert(('e', VowelMod::Circumflex, ToneMark::Hook), 'ể');
    map.insert(('e', VowelMod::Circumflex, ToneMark::Tilde), 'ễ');
    map.insert(('e', VowelMod::Circumflex, ToneMark::Dot), 'ệ');

    // I variants
    map.insert(('i', VowelMod::None, ToneMark::None), 'i');
    map.insert(('i', VowelMod::None, ToneMark::Acute), 'í');
    map.insert(('i', VowelMod::None, ToneMark::Grave), 'ì');
    map.insert(('i', VowelMod::None, ToneMark::Hook), 'ỉ');
    map.insert(('i', VowelMod::None, ToneMark::Tilde), 'ĩ');
    map.insert(('i', VowelMod::None, ToneMark::Dot), 'ị');

    // O variants
    map.insert(('o', VowelMod::None, ToneMark::None), 'o');
    map.insert(('o', VowelMod::None, ToneMark::Acute), 'ó');
    map.insert(('o', VowelMod::None, ToneMark::Grave), 'ò');
    map.insert(('o', VowelMod::None, ToneMark::Hook), 'ỏ');
    map.insert(('o', VowelMod::None, ToneMark::Tilde), 'õ');
    map.insert(('o', VowelMod::None, ToneMark::Dot), 'ọ');

    map.insert(('o', VowelMod::Circumflex, ToneMark::None), 'ô');
    map.insert(('o', VowelMod::Circumflex, ToneMark::Acute), 'ố');
    map.insert(('o', VowelMod::Circumflex, ToneMark::Grave), 'ồ');
    map.insert(('o', VowelMod::Circumflex, ToneMark::Hook), 'ổ');
    map.insert(('o', VowelMod::Circumflex, ToneMark::Tilde), 'ỗ');
    map.insert(('o', VowelMod::Circumflex, ToneMark::Dot), 'ộ');

    map.insert(('o', VowelMod::Horn, ToneMark::None), 'ơ');
    map.insert(('o', VowelMod::Horn, ToneMark::Acute), 'ớ');
    map.insert(('o', VowelMod::Horn, ToneMark::Grave), 'ờ');
    map.insert(('o', VowelMod::Horn, ToneMark::Hook), 'ở');
    map.insert(('o', VowelMod::Horn, ToneMark::Tilde), 'ỡ');
    map.insert(('o', VowelMod::Horn, ToneMark::Dot), 'ợ');

    // U variants
    map.insert(('u', VowelMod::None, ToneMark::None), 'u');
    map.insert(('u', VowelMod::None, ToneMark::Acute), 'ú');
    map.insert(('u', VowelMod::None, ToneMark::Grave), 'ù');
    map.insert(('u', VowelMod::None, ToneMark::Hook), 'ủ');
    map.insert(('u', VowelMod::None, ToneMark::Tilde), 'ũ');
    map.insert(('u', VowelMod::None, ToneMark::Dot), 'ụ');

    map.insert(('u', VowelMod::Horn, ToneMark::None), 'ư');
    map.insert(('u', VowelMod::Horn, ToneMark::Acute), 'ứ');
    map.insert(('u', VowelMod::Horn, ToneMark::Grave), 'ừ');
    map.insert(('u', VowelMod::Horn, ToneMark::Hook), 'ử');
    map.insert(('u', VowelMod::Horn, ToneMark::Tilde), 'ữ');
    map.insert(('u', VowelMod::Horn, ToneMark::Dot), 'ự');

    // Y variants
    map.insert(('y', VowelMod::None, ToneMark::None), 'y');
    map.insert(('y', VowelMod::None, ToneMark::Acute), 'ý');
    map.insert(('y', VowelMod::None, ToneMark::Grave), 'ỳ');
    map.insert(('y', VowelMod::None, ToneMark::Hook), 'ỷ');
    map.insert(('y', VowelMod::None, ToneMark::Tilde), 'ỹ');
    map.insert(('y', VowelMod::None, ToneMark::Dot), 'ỵ');

    map
});

/// Reverse mapping from Vietnamese char to components
pub static REVERSE_MAP: LazyLock<HashMap<char, (char, VowelMod, ToneMark)>> =
    LazyLock::new(|| CHAR_MAP.iter().map(|(&k, &v)| (v, k)).collect());

/// Check if a character is a vowel (including modified forms)
pub fn is_vowel(c: char) -> bool {
    let lower = c.to_ascii_lowercase();
    VOWELS.contains(&lower) || REVERSE_MAP.contains_key(&c)
}

/// Check if a character is a consonant
pub fn is_consonant(c: char) -> bool {
    CONSONANTS.contains(&c.to_ascii_lowercase())
}

/// Get the base form of a Vietnamese character
pub fn get_base(c: char) -> char {
    REVERSE_MAP
        .get(&c.to_ascii_lowercase())
        .map_or(c, |&(base, _, _)| {
            if c.is_uppercase() {
                base.to_ascii_uppercase()
            } else {
                base
            }
        })
}

/// Get character with new tone mark
pub fn with_tone(c: char, tone: ToneMark) -> Option<char> {
    let lower = c.to_ascii_lowercase();
    let (base, modifier, _) =
        REVERSE_MAP
            .get(&lower)
            .copied()
            .unwrap_or((lower, VowelMod::None, ToneMark::None));

    CHAR_MAP.get(&(base, modifier, tone)).map(|&result| {
        if c.is_uppercase() {
            result.to_uppercase().next().unwrap_or(result)
        } else {
            result
        }
    })
}

/// Get character with new vowel modifier
pub fn with_modifier(c: char, modifier: VowelMod) -> Option<char> {
    let lower = c.to_ascii_lowercase();
    let (base, _, tone) =
        REVERSE_MAP
            .get(&lower)
            .copied()
            .unwrap_or((lower, VowelMod::None, ToneMark::None));

    CHAR_MAP.get(&(base, modifier, tone)).map(|&result| {
        if c.is_uppercase() {
            result.to_uppercase().next().unwrap_or(result)
        } else {
            result
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_vowel() {
        assert!(is_vowel('a'));
        assert!(is_vowel('ă'));
        assert!(is_vowel('ấ'));
        assert!(!is_vowel('b'));
    }

    #[test]
    fn test_with_tone() {
        assert_eq!(with_tone('a', ToneMark::Acute), Some('á'));
        assert_eq!(with_tone('â', ToneMark::Grave), Some('ầ'));
        assert_eq!(with_tone('A', ToneMark::Acute), Some('Á'));
    }

    #[test]
    fn test_with_modifier() {
        assert_eq!(with_modifier('a', VowelMod::Circumflex), Some('â'));
        assert_eq!(with_modifier('á', VowelMod::Circumflex), Some('ấ'));
        assert_eq!(with_modifier('o', VowelMod::Horn), Some('ơ'));
    }
}
