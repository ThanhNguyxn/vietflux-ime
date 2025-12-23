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
            's' => KeyAction::Tone(ToneMark::Acute), // sắc
            'f' => KeyAction::Tone(ToneMark::Grave), // huyền
            'r' => KeyAction::Tone(ToneMark::Hook),  // hỏi
            'x' => KeyAction::Tone(ToneMark::Tilde), // ngã
            'j' => KeyAction::Tone(ToneMark::Dot),   // nặng

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
            'w' => prev_char.map_or(KeyAction::None, |prev| match prev.to_ascii_lowercase() {
                'a' => KeyAction::Modifier(VowelMod::Breve), // aw = ă
                'o' | 'u' => KeyAction::Modifier(VowelMod::Horn), // ow=ơ, uw=ư
                _ => KeyAction::None,
            }),

            // dd = đ
            'd' => {
                if let Some(prev) = prev_char {
                    if prev.eq_ignore_ascii_case(&'d') {
                        return KeyAction::Stroke;
                    }
                }
                KeyAction::None
            }

            // z = remove diacritics
            'z' => KeyAction::RemoveDiacritics,

            // Quick Telex: double consonant shortcuts
            // cc→ch, gg→gh, kk→kh, nn→nh, pp→ph, tt→th, qq→qu
            'c' | 'g' | 'k' | 'n' | 'p' | 't' | 'q' => {
                if let Some(prev) = prev_char {
                    if prev.to_ascii_lowercase() == key_lower {
                        let replacement = match key_lower {
                            'c' => "ch",
                            'g' => "gh",
                            'k' => "kh",
                            'n' => "nh",
                            'p' => "ph",
                            't' => "th",
                            'q' => "qu",
                            _ => unreachable!(),
                        };
                        return KeyAction::QuickTelex(replacement);
                    }
                }
                KeyAction::None
            }

            // Quick key shortcuts (OpenKey style)
            // [ → ư, ] → ơ for faster typing without needing uw/ow
            '[' => KeyAction::InsertChar('ư'),
            ']' => KeyAction::InsertChar('ơ'),

            _ => KeyAction::None,
        }
    }

    fn is_modifier_key(&self, key: char) -> bool {
        matches!(key.to_ascii_lowercase(), 's' | 'f' | 'r' | 'x' | 'j' | 'z')
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
        assert_eq!(
            telex.process('a', Some('a')),
            KeyAction::Modifier(VowelMod::Circumflex)
        );
        assert_eq!(
            telex.process('e', Some('e')),
            KeyAction::Modifier(VowelMod::Circumflex)
        );
        assert_eq!(
            telex.process('o', Some('o')),
            KeyAction::Modifier(VowelMod::Circumflex)
        );
    }

    #[test]
    fn test_telex_horn_breve() {
        let telex = Telex;
        assert_eq!(
            telex.process('w', Some('a')),
            KeyAction::Modifier(VowelMod::Breve)
        );
        assert_eq!(
            telex.process('w', Some('o')),
            KeyAction::Modifier(VowelMod::Horn)
        );
        assert_eq!(
            telex.process('w', Some('u')),
            KeyAction::Modifier(VowelMod::Horn)
        );
    }

    #[test]
    fn test_telex_stroke() {
        let telex = Telex;
        assert_eq!(telex.process('d', Some('d')), KeyAction::Stroke);
    }

    #[test]
    fn test_telex_quick_telex() {
        let telex = Telex;
        // cc → ch
        assert_eq!(telex.process('c', Some('c')), KeyAction::QuickTelex("ch"));
        // gg → gh
        assert_eq!(telex.process('g', Some('g')), KeyAction::QuickTelex("gh"));
        // nn → nh
        assert_eq!(telex.process('n', Some('n')), KeyAction::QuickTelex("nh"));
        // pp → ph
        assert_eq!(telex.process('p', Some('p')), KeyAction::QuickTelex("ph"));
        // tt → th
        assert_eq!(telex.process('t', Some('t')), KeyAction::QuickTelex("th"));
        // qq → qu
        assert_eq!(telex.process('q', Some('q')), KeyAction::QuickTelex("qu"));
        // kk → kh
        assert_eq!(telex.process('k', Some('k')), KeyAction::QuickTelex("kh"));
    }
}
