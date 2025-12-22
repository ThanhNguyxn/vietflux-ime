//! Vietnamese Unicode Character System
//!
//! Provides bidirectional conversion between modifiers and Vietnamese Unicode.
//! Uses lookup tables for O(1) conversion.

use super::keycodes::keys;

/// Tone modifiers (dấu phụ) - changes base vowel form
pub mod tone {
    pub const NONE: u8 = 0;
    pub const CIRCUMFLEX: u8 = 1; // â, ê, ô
    pub const HORN: u8 = 2;       // ơ, ư, ă (breve for 'a')
}

/// Marks (dấu thanh) - Vietnamese tone marks
pub mod mark {
    pub const NONE: u8 = 0;
    pub const ACUTE: u8 = 1;  // sắc (á)
    pub const GRAVE: u8 = 2;  // huyền (à)
    pub const HOOK: u8 = 3;   // hỏi (ả)
    pub const TILDE: u8 = 4;  // ngã (ã)
    pub const DOT: u8 = 5;    // nặng (ạ)
}

/// Vietnamese vowel lookup table
/// Each entry: (base_char, [acute, grave, hook, tilde, dot])
const VOWEL_TABLE: [(char, [char; 5]); 12] = [
    ('a', ['á', 'à', 'ả', 'ã', 'ạ']),
    ('ă', ['ắ', 'ằ', 'ẳ', 'ẵ', 'ặ']),
    ('â', ['ấ', 'ầ', 'ẩ', 'ẫ', 'ậ']),
    ('e', ['é', 'è', 'ẻ', 'ẽ', 'ẹ']),
    ('ê', ['ế', 'ề', 'ể', 'ễ', 'ệ']),
    ('i', ['í', 'ì', 'ỉ', 'ĩ', 'ị']),
    ('o', ['ó', 'ò', 'ỏ', 'õ', 'ọ']),
    ('ô', ['ố', 'ồ', 'ổ', 'ỗ', 'ộ']),
    ('ơ', ['ớ', 'ờ', 'ở', 'ỡ', 'ợ']),
    ('u', ['ú', 'ù', 'ủ', 'ũ', 'ụ']),
    ('ư', ['ứ', 'ừ', 'ử', 'ữ', 'ự']),
    ('y', ['ý', 'ỳ', 'ỷ', 'ỹ', 'ỵ']),
];

/// Get base vowel character from key + tone modifier
fn get_base_vowel(key: u16, tone_mod: u8) -> Option<char> {
    match key {
        keys::A => Some(match tone_mod {
            tone::CIRCUMFLEX => 'â',
            tone::HORN => 'ă', // breve for 'a'
            _ => 'a',
        }),
        keys::E => Some(match tone_mod {
            tone::CIRCUMFLEX => 'ê',
            _ => 'e',
        }),
        keys::I => Some('i'),
        keys::O => Some(match tone_mod {
            tone::CIRCUMFLEX => 'ô',
            tone::HORN => 'ơ',
            _ => 'o',
        }),
        keys::U => Some(match tone_mod {
            tone::HORN => 'ư',
            _ => 'u',
        }),
        keys::Y => Some('y'),
        _ => None,
    }
}

/// Apply mark to base vowel character
fn apply_mark(base: char, mark_type: u8) -> char {
    if mark_type == mark::NONE || mark_type > mark::DOT {
        return base;
    }

    VOWEL_TABLE
        .iter()
        .find(|(b, _)| *b == base)
        .map(|(_, marks)| marks[(mark_type - 1) as usize])
        .unwrap_or(base)
}

/// Convert uppercase using Rust's built-in method
fn to_uppercase(ch: char) -> char {
    ch.to_uppercase().next().unwrap_or(ch)
}

/// Convert key + modifiers to Vietnamese character
///
/// # Arguments
/// * `key` - Virtual keycode
/// * `uppercase` - Uppercase flag
/// * `tone_mod` - Tone modifier: 0=none, 1=circumflex, 2=horn/breve
/// * `mark_type` - Mark: 0=none, 1=acute, 2=grave, 3=hook, 4=tilde, 5=dot
pub fn compose_char(key: u16, uppercase: bool, tone_mod: u8, mark_type: u8) -> Option<char> {
    // Handle D specially (not a vowel but needs conversion)
    if key == keys::D {
        return Some(if uppercase { 'D' } else { 'd' });
    }

    let base = get_base_vowel(key, tone_mod)?;
    let marked = apply_mark(base, mark_type);
    Some(if uppercase { to_uppercase(marked) } else { marked })
}

/// Get đ/Đ character
pub fn get_stroke_d(uppercase: bool) -> char {
    if uppercase { 'Đ' } else { 'đ' }
}

/// Parsed character components (for reverse conversion)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CharComponents {
    pub key: u16,
    pub uppercase: bool,
    pub tone_mod: u8,
    pub mark_type: u8,
    pub is_stroked: bool,
}

impl CharComponents {
    const fn new(key: u16, uppercase: bool, tone_mod: u8, mark_type: u8) -> Self {
        Self {
            key,
            uppercase,
            tone_mod,
            mark_type,
            is_stroked: false,
        }
    }

    const fn stroked(key: u16, uppercase: bool) -> Self {
        Self {
            key,
            uppercase,
            tone_mod: 0,
            mark_type: 0,
            is_stroked: true,
        }
    }
}

/// Parse Vietnamese character to components
pub fn decompose_char(c: char) -> Option<CharComponents> {
    use keys::*;
    use tone::{CIRCUMFLEX, HORN};
    use mark::{ACUTE, GRAVE, HOOK, TILDE, DOT};

    macro_rules! vowel {
        ($key:expr, $up:expr, $tone:expr, $mark:expr) => {
            Some(CharComponents::new($key, $up, $tone, $mark))
        };
    }

    match c {
        // A variants
        'a' => vowel!(A, false, 0, 0),
        'A' => vowel!(A, true, 0, 0),
        'á' => vowel!(A, false, 0, ACUTE),
        'Á' => vowel!(A, true, 0, ACUTE),
        'à' => vowel!(A, false, 0, GRAVE),
        'À' => vowel!(A, true, 0, GRAVE),
        'ả' => vowel!(A, false, 0, HOOK),
        'Ả' => vowel!(A, true, 0, HOOK),
        'ã' => vowel!(A, false, 0, TILDE),
        'Ã' => vowel!(A, true, 0, TILDE),
        'ạ' => vowel!(A, false, 0, DOT),
        'Ạ' => vowel!(A, true, 0, DOT),
        // ă (breve)
        'ă' => vowel!(A, false, HORN, 0),
        'Ă' => vowel!(A, true, HORN, 0),
        'ắ' => vowel!(A, false, HORN, ACUTE),
        'Ắ' => vowel!(A, true, HORN, ACUTE),
        'ằ' => vowel!(A, false, HORN, GRAVE),
        'Ằ' => vowel!(A, true, HORN, GRAVE),
        'ẳ' => vowel!(A, false, HORN, HOOK),
        'Ẳ' => vowel!(A, true, HORN, HOOK),
        'ẵ' => vowel!(A, false, HORN, TILDE),
        'Ẵ' => vowel!(A, true, HORN, TILDE),
        'ặ' => vowel!(A, false, HORN, DOT),
        'Ặ' => vowel!(A, true, HORN, DOT),
        // â (circumflex)
        'â' => vowel!(A, false, CIRCUMFLEX, 0),
        'Â' => vowel!(A, true, CIRCUMFLEX, 0),
        'ấ' => vowel!(A, false, CIRCUMFLEX, ACUTE),
        'Ấ' => vowel!(A, true, CIRCUMFLEX, ACUTE),
        'ầ' => vowel!(A, false, CIRCUMFLEX, GRAVE),
        'Ầ' => vowel!(A, true, CIRCUMFLEX, GRAVE),
        'ẩ' => vowel!(A, false, CIRCUMFLEX, HOOK),
        'Ẩ' => vowel!(A, true, CIRCUMFLEX, HOOK),
        'ẫ' => vowel!(A, false, CIRCUMFLEX, TILDE),
        'Ẫ' => vowel!(A, true, CIRCUMFLEX, TILDE),
        'ậ' => vowel!(A, false, CIRCUMFLEX, DOT),
        'Ậ' => vowel!(A, true, CIRCUMFLEX, DOT),

        // E variants
        'e' => vowel!(E, false, 0, 0),
        'E' => vowel!(E, true, 0, 0),
        'é' => vowel!(E, false, 0, ACUTE),
        'É' => vowel!(E, true, 0, ACUTE),
        'è' => vowel!(E, false, 0, GRAVE),
        'È' => vowel!(E, true, 0, GRAVE),
        'ẻ' => vowel!(E, false, 0, HOOK),
        'Ẻ' => vowel!(E, true, 0, HOOK),
        'ẽ' => vowel!(E, false, 0, TILDE),
        'Ẽ' => vowel!(E, true, 0, TILDE),
        'ẹ' => vowel!(E, false, 0, DOT),
        'Ẹ' => vowel!(E, true, 0, DOT),
        // ê (circumflex)
        'ê' => vowel!(E, false, CIRCUMFLEX, 0),
        'Ê' => vowel!(E, true, CIRCUMFLEX, 0),
        'ế' => vowel!(E, false, CIRCUMFLEX, ACUTE),
        'Ế' => vowel!(E, true, CIRCUMFLEX, ACUTE),
        'ề' => vowel!(E, false, CIRCUMFLEX, GRAVE),
        'Ề' => vowel!(E, true, CIRCUMFLEX, GRAVE),
        'ể' => vowel!(E, false, CIRCUMFLEX, HOOK),
        'Ể' => vowel!(E, true, CIRCUMFLEX, HOOK),
        'ễ' => vowel!(E, false, CIRCUMFLEX, TILDE),
        'Ễ' => vowel!(E, true, CIRCUMFLEX, TILDE),
        'ệ' => vowel!(E, false, CIRCUMFLEX, DOT),
        'Ệ' => vowel!(E, true, CIRCUMFLEX, DOT),

        // I variants
        'i' => vowel!(I, false, 0, 0),
        'I' => vowel!(I, true, 0, 0),
        'í' => vowel!(I, false, 0, ACUTE),
        'Í' => vowel!(I, true, 0, ACUTE),
        'ì' => vowel!(I, false, 0, GRAVE),
        'Ì' => vowel!(I, true, 0, GRAVE),
        'ỉ' => vowel!(I, false, 0, HOOK),
        'Ỉ' => vowel!(I, true, 0, HOOK),
        'ĩ' => vowel!(I, false, 0, TILDE),
        'Ĩ' => vowel!(I, true, 0, TILDE),
        'ị' => vowel!(I, false, 0, DOT),
        'Ị' => vowel!(I, true, 0, DOT),

        // O variants
        'o' => vowel!(O, false, 0, 0),
        'O' => vowel!(O, true, 0, 0),
        'ó' => vowel!(O, false, 0, ACUTE),
        'Ó' => vowel!(O, true, 0, ACUTE),
        'ò' => vowel!(O, false, 0, GRAVE),
        'Ò' => vowel!(O, true, 0, GRAVE),
        'ỏ' => vowel!(O, false, 0, HOOK),
        'Ỏ' => vowel!(O, true, 0, HOOK),
        'õ' => vowel!(O, false, 0, TILDE),
        'Õ' => vowel!(O, true, 0, TILDE),
        'ọ' => vowel!(O, false, 0, DOT),
        'Ọ' => vowel!(O, true, 0, DOT),
        // ô (circumflex)
        'ô' => vowel!(O, false, CIRCUMFLEX, 0),
        'Ô' => vowel!(O, true, CIRCUMFLEX, 0),
        'ố' => vowel!(O, false, CIRCUMFLEX, ACUTE),
        'Ố' => vowel!(O, true, CIRCUMFLEX, ACUTE),
        'ồ' => vowel!(O, false, CIRCUMFLEX, GRAVE),
        'Ồ' => vowel!(O, true, CIRCUMFLEX, GRAVE),
        'ổ' => vowel!(O, false, CIRCUMFLEX, HOOK),
        'Ổ' => vowel!(O, true, CIRCUMFLEX, HOOK),
        'ỗ' => vowel!(O, false, CIRCUMFLEX, TILDE),
        'Ỗ' => vowel!(O, true, CIRCUMFLEX, TILDE),
        'ộ' => vowel!(O, false, CIRCUMFLEX, DOT),
        'Ộ' => vowel!(O, true, CIRCUMFLEX, DOT),
        // ơ (horn)
        'ơ' => vowel!(O, false, HORN, 0),
        'Ơ' => vowel!(O, true, HORN, 0),
        'ớ' => vowel!(O, false, HORN, ACUTE),
        'Ớ' => vowel!(O, true, HORN, ACUTE),
        'ờ' => vowel!(O, false, HORN, GRAVE),
        'Ờ' => vowel!(O, true, HORN, GRAVE),
        'ở' => vowel!(O, false, HORN, HOOK),
        'Ở' => vowel!(O, true, HORN, HOOK),
        'ỡ' => vowel!(O, false, HORN, TILDE),
        'Ỡ' => vowel!(O, true, HORN, TILDE),
        'ợ' => vowel!(O, false, HORN, DOT),
        'Ợ' => vowel!(O, true, HORN, DOT),

        // U variants
        'u' => vowel!(U, false, 0, 0),
        'U' => vowel!(U, true, 0, 0),
        'ú' => vowel!(U, false, 0, ACUTE),
        'Ú' => vowel!(U, true, 0, ACUTE),
        'ù' => vowel!(U, false, 0, GRAVE),
        'Ù' => vowel!(U, true, 0, GRAVE),
        'ủ' => vowel!(U, false, 0, HOOK),
        'Ủ' => vowel!(U, true, 0, HOOK),
        'ũ' => vowel!(U, false, 0, TILDE),
        'Ũ' => vowel!(U, true, 0, TILDE),
        'ụ' => vowel!(U, false, 0, DOT),
        'Ụ' => vowel!(U, true, 0, DOT),
        // ư (horn)
        'ư' => vowel!(U, false, HORN, 0),
        'Ư' => vowel!(U, true, HORN, 0),
        'ứ' => vowel!(U, false, HORN, ACUTE),
        'Ứ' => vowel!(U, true, HORN, ACUTE),
        'ừ' => vowel!(U, false, HORN, GRAVE),
        'Ừ' => vowel!(U, true, HORN, GRAVE),
        'ử' => vowel!(U, false, HORN, HOOK),
        'Ử' => vowel!(U, true, HORN, HOOK),
        'ữ' => vowel!(U, false, HORN, TILDE),
        'Ữ' => vowel!(U, true, HORN, TILDE),
        'ự' => vowel!(U, false, HORN, DOT),
        'Ự' => vowel!(U, true, HORN, DOT),

        // Y variants
        'y' => vowel!(Y, false, 0, 0),
        'Y' => vowel!(Y, true, 0, 0),
        'ý' => vowel!(Y, false, 0, ACUTE),
        'Ý' => vowel!(Y, true, 0, ACUTE),
        'ỳ' => vowel!(Y, false, 0, GRAVE),
        'Ỳ' => vowel!(Y, true, 0, GRAVE),
        'ỷ' => vowel!(Y, false, 0, HOOK),
        'Ỷ' => vowel!(Y, true, 0, HOOK),
        'ỹ' => vowel!(Y, false, 0, TILDE),
        'Ỹ' => vowel!(Y, true, 0, TILDE),
        'ỵ' => vowel!(Y, false, 0, DOT),
        'Ỵ' => vowel!(Y, true, 0, DOT),

        // D variants
        'đ' => Some(CharComponents::stroked(D, false)),
        'Đ' => Some(CharComponents::stroked(D, true)),
        'd' => vowel!(D, false, 0, 0),
        'D' => vowel!(D, true, 0, 0),

        // Other consonants (simplified for now)
        c if c.is_ascii_alphabetic() => {
            let key = super::keycodes::from_char(c)?;
            Some(CharComponents::new(key, c.is_uppercase(), 0, 0))
        }

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_basic() {
        assert_eq!(compose_char(keys::A, false, 0, 0), Some('a'));
        assert_eq!(compose_char(keys::A, true, 0, 0), Some('A'));
    }

    #[test]
    fn test_compose_with_tone() {
        assert_eq!(compose_char(keys::A, false, tone::CIRCUMFLEX, 0), Some('â'));
        assert_eq!(compose_char(keys::O, false, tone::HORN, 0), Some('ơ'));
        assert_eq!(compose_char(keys::U, false, tone::HORN, 0), Some('ư'));
    }

    #[test]
    fn test_compose_with_mark() {
        assert_eq!(compose_char(keys::A, false, 0, mark::ACUTE), Some('á'));
        assert_eq!(compose_char(keys::A, false, 0, mark::GRAVE), Some('à'));
        assert_eq!(compose_char(keys::A, false, tone::CIRCUMFLEX, mark::ACUTE), Some('ấ'));
    }

    #[test]
    fn test_decompose() {
        let c = decompose_char('á').unwrap();
        assert_eq!(c.key, keys::A);
        assert_eq!(c.mark_type, mark::ACUTE);

        let c = decompose_char('ấ').unwrap();
        assert_eq!(c.key, keys::A);
        assert_eq!(c.tone_mod, tone::CIRCUMFLEX);
        assert_eq!(c.mark_type, mark::ACUTE);
    }
}
