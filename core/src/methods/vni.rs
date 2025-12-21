//! VNI Input Method
//!
//! Key mappings based on UniKey VNI:
//! - Tone marks: 1=sắc, 2=huyền, 3=hỏi, 4=ngã, 5=nặng
//! - Vowel modifiers: 6=circumflex, 7=horn, 8=breve
//! - Consonant: 9=đ (after d)
//! - Remove: 0

use super::{InputMethod, KeyAction};
use crate::chars::{ToneMark, VowelMod};

/// VNI input method
pub struct Vni;

impl InputMethod for Vni {
    fn name(&self) -> &'static str {
        "vni"
    }
    
    fn process(&self, key: char, prev_char: Option<char>) -> KeyAction {
        match key {
            // Tone marks
            '1' => KeyAction::Tone(ToneMark::Acute),   // sắc
            '2' => KeyAction::Tone(ToneMark::Grave),   // huyền
            '3' => KeyAction::Tone(ToneMark::Hook),    // hỏi
            '4' => KeyAction::Tone(ToneMark::Tilde),   // ngã
            '5' => KeyAction::Tone(ToneMark::Dot),     // nặng
            
            // Vowel modifiers
            '6' => KeyAction::Modifier(VowelMod::Circumflex), // â, ê, ô
            '7' => KeyAction::Modifier(VowelMod::Horn),       // ơ, ư
            '8' => KeyAction::Modifier(VowelMod::Breve),      // ă
            
            // Stroke (đ)
            '9' => {
                if let Some(prev) = prev_char {
                    if prev.to_ascii_lowercase() == 'd' {
                        return KeyAction::Stroke;
                    }
                }
                KeyAction::None
            }
            
            // Remove diacritics
            '0' => KeyAction::RemoveDiacritics,
            
            _ => KeyAction::None,
        }
    }
    
    fn is_modifier_key(&self, key: char) -> bool {
        matches!(key, '1'..='9' | '0')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vni_tones() {
        let vni = Vni;
        assert_eq!(vni.process('1', None), KeyAction::Tone(ToneMark::Acute));
        assert_eq!(vni.process('2', None), KeyAction::Tone(ToneMark::Grave));
        assert_eq!(vni.process('3', None), KeyAction::Tone(ToneMark::Hook));
        assert_eq!(vni.process('4', None), KeyAction::Tone(ToneMark::Tilde));
        assert_eq!(vni.process('5', None), KeyAction::Tone(ToneMark::Dot));
    }

    #[test]
    fn test_vni_modifiers() {
        let vni = Vni;
        assert_eq!(vni.process('6', None), KeyAction::Modifier(VowelMod::Circumflex));
        assert_eq!(vni.process('7', None), KeyAction::Modifier(VowelMod::Horn));
        assert_eq!(vni.process('8', None), KeyAction::Modifier(VowelMod::Breve));
    }

    #[test]
    fn test_vni_stroke() {
        let vni = Vni;
        assert_eq!(vni.process('9', Some('d')), KeyAction::Stroke);
        assert_eq!(vni.process('9', Some('a')), KeyAction::None);
    }
}
