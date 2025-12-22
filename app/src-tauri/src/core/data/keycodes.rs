//! Virtual Keycode Definitions
//!
//! Provides keycode constants and classification helpers for VietFlux.

/// Virtual keycodes (using simple sequential numbering)
pub mod keys {
    pub const A: u16 = 0;
    pub const B: u16 = 1;
    pub const C: u16 = 2;
    pub const D: u16 = 3;
    pub const E: u16 = 4;
    pub const F: u16 = 5;
    pub const G: u16 = 6;
    pub const H: u16 = 7;
    pub const I: u16 = 8;
    pub const J: u16 = 9;
    pub const K: u16 = 10;
    pub const L: u16 = 11;
    pub const M: u16 = 12;
    pub const N: u16 = 13;
    pub const O: u16 = 14;
    pub const P: u16 = 15;
    pub const Q: u16 = 16;
    pub const R: u16 = 17;
    pub const S: u16 = 18;
    pub const T: u16 = 19;
    pub const U: u16 = 20;
    pub const V: u16 = 21;
    pub const W: u16 = 22;
    pub const X: u16 = 23;
    pub const Y: u16 = 24;
    pub const Z: u16 = 25;

    // Number keys (for VNI mode)
    pub const N0: u16 = 48;
    pub const N1: u16 = 49;
    pub const N2: u16 = 50;
    pub const N3: u16 = 51;
    pub const N4: u16 = 52;
    pub const N5: u16 = 53;
    pub const N6: u16 = 54;
    pub const N7: u16 = 55;
    pub const N8: u16 = 56;
    pub const N9: u16 = 57;

    // Special keys
    pub const SPACE: u16 = 100;
    pub const BACKSPACE: u16 = 101;
    pub const ESC: u16 = 102;
}

/// Vietnamese vowels
const VOWELS: [u16; 6] = [keys::A, keys::E, keys::I, keys::O, keys::U, keys::Y];

/// Check if keycode is a vowel
pub fn is_vowel(key: u16) -> bool {
    VOWELS.contains(&key)
}

/// Check if keycode is a consonant (letter but not vowel)
pub fn is_consonant(key: u16) -> bool {
    key <= keys::Z && !is_vowel(key)
}

/// Check if keycode is a letter (A-Z)
pub fn is_letter(key: u16) -> bool {
    key <= keys::Z
}

/// Check if keycode is a number (0-9)
pub fn is_number(key: u16) -> bool {
    (keys::N0..=keys::N9).contains(&key)
}

/// Convert char to keycode
pub fn from_char(c: char) -> Option<u16> {
    match c.to_ascii_lowercase() {
        'a' => Some(keys::A),
        'b' => Some(keys::B),
        'c' => Some(keys::C),
        'd' => Some(keys::D),
        'e' => Some(keys::E),
        'f' => Some(keys::F),
        'g' => Some(keys::G),
        'h' => Some(keys::H),
        'i' => Some(keys::I),
        'j' => Some(keys::J),
        'k' => Some(keys::K),
        'l' => Some(keys::L),
        'm' => Some(keys::M),
        'n' => Some(keys::N),
        'o' => Some(keys::O),
        'p' => Some(keys::P),
        'q' => Some(keys::Q),
        'r' => Some(keys::R),
        's' => Some(keys::S),
        't' => Some(keys::T),
        'u' => Some(keys::U),
        'v' => Some(keys::V),
        'w' => Some(keys::W),
        'x' => Some(keys::X),
        'y' => Some(keys::Y),
        'z' => Some(keys::Z),
        '0' => Some(keys::N0),
        '1' => Some(keys::N1),
        '2' => Some(keys::N2),
        '3' => Some(keys::N3),
        '4' => Some(keys::N4),
        '5' => Some(keys::N5),
        '6' => Some(keys::N6),
        '7' => Some(keys::N7),
        '8' => Some(keys::N8),
        '9' => Some(keys::N9),
        ' ' => Some(keys::SPACE),
        _ => None,
    }
}

/// Convert keycode to char
pub fn to_char(key: u16, uppercase: bool) -> Option<char> {
    let c = match key {
        keys::A => 'a',
        keys::B => 'b',
        keys::C => 'c',
        keys::D => 'd',
        keys::E => 'e',
        keys::F => 'f',
        keys::G => 'g',
        keys::H => 'h',
        keys::I => 'i',
        keys::J => 'j',
        keys::K => 'k',
        keys::L => 'l',
        keys::M => 'm',
        keys::N => 'n',
        keys::O => 'o',
        keys::P => 'p',
        keys::Q => 'q',
        keys::R => 'r',
        keys::S => 's',
        keys::T => 't',
        keys::U => 'u',
        keys::V => 'v',
        keys::W => 'w',
        keys::X => 'x',
        keys::Y => 'y',
        keys::Z => 'z',
        keys::N0 => '0',
        keys::N1 => '1',
        keys::N2 => '2',
        keys::N3 => '3',
        keys::N4 => '4',
        keys::N5 => '5',
        keys::N6 => '6',
        keys::N7 => '7',
        keys::N8 => '8',
        keys::N9 => '9',
        keys::SPACE => ' ',
        _ => return None,
    };
    Some(if uppercase { c.to_ascii_uppercase() } else { c })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vowel_detection() {
        assert!(is_vowel(keys::A));
        assert!(is_vowel(keys::E));
        assert!(is_vowel(keys::I));
        assert!(is_vowel(keys::O));
        assert!(is_vowel(keys::U));
        assert!(is_vowel(keys::Y));
        assert!(!is_vowel(keys::B));
        assert!(!is_vowel(keys::D));
    }

    #[test]
    fn test_consonant_detection() {
        assert!(is_consonant(keys::B));
        assert!(is_consonant(keys::C));
        assert!(is_consonant(keys::D));
        assert!(!is_consonant(keys::A));
        assert!(!is_consonant(keys::E));
    }

    #[test]
    fn test_char_conversion() {
        assert_eq!(from_char('a'), Some(keys::A));
        assert_eq!(from_char('A'), Some(keys::A));
        assert_eq!(to_char(keys::A, false), Some('a'));
        assert_eq!(to_char(keys::A, true), Some('A'));
    }
}
