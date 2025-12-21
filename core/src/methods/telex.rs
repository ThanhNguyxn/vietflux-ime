//! Telex Input Method
//!
//! Key mappings based on UniKey Telex:
//! - Tone marks: s=sắc, f=huyền, r=hỏi, x=ngã, j=nặng
//! - Vowel modifiers: aa=â, ee=ê, oo=ô, aw=ă, ow=ơ, uw=ư
//! - Consonant: dd=đ
//! - Remove: z

use super::{InputMethod, KeyAction};
use crate::chars::{ToneMark, VowelMod};

/// Telex input method
pub struct Telex;

impl InputMethod for Telex {
    fn name(&self) -> &'static str {
        "telex"
    }
    
    fn process(&self, key: char, prev_char: Option<char>) -> KeyAction {
        let key_lower = key.to_ascii_lowercase();
        
        match key_lower {
            // Tone marks
            's' => KeyAction::Tone(ToneMark::Acute),   // sắc
            'f' => KeyAction::Tone(ToneMark::Grave),   // huyền
            'r' => KeyAction::Tone(ToneMark::Hook),    // hỏi
            'x' => KeyAction::Tone(ToneMark::Tilde),   // ngã
            'j' => KeyAction::Tone(ToneMark::Dot),     // nặng
            
            // Vowel modifiers (double key = circumflex)
            'a' | 'e' | 'o' => {
                if let Some(prev) = prev_char {
                    if prev.to_ascii_lowercase() == key_lower {
                        return KeyAction::Modifier(VowelMod::Circumflex);
                    }
                }
                KeyAction::None
            }
            
            // w = horn (ơ, ư) or breve (ă)
            'w' => {
                if let Some(prev) = prev_char {
                    match prev.to_ascii_lowercase() {
                        'a' => KeyAction::Modifier(VowelMod::Breve),  // aw = ă
                        'o' | 'u' => KeyAction::Modifier(VowelMod::Horn), // ow=ơ, uw=ư
                        _ => KeyAction::None,
                    }
                } else {
                    KeyAction::None
                }
            }
            
            // dd = đ
            'd' => {
                if let Some(prev) = prev_char {
                    if prev.to_ascii_lowercase() == 'd' {
                        return KeyAction::Stroke;
                    }
                }
                KeyAction::None
            }
            
            // z = remove diacritics
            'z' => KeyAction::RemoveDiacritics,
            
            _ => KeyAction::None,
        }
    }
    
    fn is_modifier_key(&self, key: char) -> bool {
        matches!(
            key.to_ascii_lowercase(),
            's' | 'f' | 'r' | 'x' | 'j' | 'z'
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telex_tones() {
        let telex = Telex;
        assert_eq!(telex.process('s', None), KeyAction::Tone(ToneMark::Acute));
        assert_eq!(telex.process('f', None), KeyAction::Tone(ToneMark::Grave));
        assert_eq!(telex.process('r', None), KeyAction::Tone(ToneMark::Hook));
        assert_eq!(telex.process('x', None), KeyAction::Tone(ToneMark::Tilde));
        assert_eq!(telex.process('j', None), KeyAction::Tone(ToneMark::Dot));
    }

    #[test]
    fn test_telex_circumflex() {
        let telex = Telex;
        assert_eq!(telex.process('a', Some('a')), KeyAction::Modifier(VowelMod::Circumflex));
        assert_eq!(telex.process('e', Some('e')), KeyAction::Modifier(VowelMod::Circumflex));
        assert_eq!(telex.process('o', Some('o')), KeyAction::Modifier(VowelMod::Circumflex));
    }

    #[test]
    fn test_telex_horn_breve() {
        let telex = Telex;
        assert_eq!(telex.process('w', Some('a')), KeyAction::Modifier(VowelMod::Breve));
        assert_eq!(telex.process('w', Some('o')), KeyAction::Modifier(VowelMod::Horn));
        assert_eq!(telex.process('w', Some('u')), KeyAction::Modifier(VowelMod::Horn));
    }

    #[test]
    fn test_telex_stroke() {
        let telex = Telex;
        assert_eq!(telex.process('d', Some('d')), KeyAction::Stroke);
    }
}
