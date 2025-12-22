//! Vietnamese Phonology
//!
//! Implements Vietnamese vowel rules and tone mark placement.

use super::data::keycodes::keys;

/// Tone placement position
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TonePosition {
    First,
    Second,
    Last,
}

/// Diphthongs where tone goes on FIRST vowel
const TONE_FIRST_PATTERNS: [[u16; 2]; 11] = [
    [keys::A, keys::I],  // ai
    [keys::A, keys::O],  // ao
    [keys::A, keys::U],  // au
    [keys::A, keys::Y],  // ay
    [keys::E, keys::O],  // eo
    [keys::I, keys::A],  // ia
    [keys::I, keys::U],  // iu
    [keys::O, keys::I],  // oi
    [keys::U, keys::I],  // ui
    [keys::U, keys::A],  // ua (without qu)
    [keys::U, keys::U],  // ưu
];

/// Diphthongs where tone goes on SECOND vowel
const TONE_SECOND_PATTERNS: [[u16; 2]; 6] = [
    [keys::O, keys::A],  // oa
    [keys::O, keys::E],  // oe
    [keys::U, keys::E],  // uê
    [keys::U, keys::Y],  // uy
    [keys::I, keys::E],  // iê
    [keys::U, keys::O],  // uô/ươ
];

/// Vowel info for phonology analysis
#[derive(Clone, Copy, Debug)]
pub struct VowelInfo {
    pub key: u16,
    pub pos: usize,
    pub has_tone_mod: bool,
}

/// Find the position where tone mark should be placed
///
/// # Arguments
/// * `vowels` - List of vowel info (key, position, has_tone_mod)
/// * `has_final_consonant` - Whether syllable has final consonant
/// * `has_qu_initial` - Whether syllable starts with "qu"
/// * `has_gi_initial` - Whether syllable starts with "gi"
/// * `use_modern_style` - true for modern (hoà), false for traditional (hòa)
pub fn find_mark_position(
    vowels: &[VowelInfo],
    has_final_consonant: bool,
    has_qu_initial: bool,
    has_gi_initial: bool,
    use_modern_style: bool,
) -> usize {
    // Handle gi-initial: first 'i' is part of consonant
    if has_gi_initial && vowels.len() >= 2 && vowels[0].key == keys::I {
        let remaining = &vowels[1..];
        return find_mark_position_impl(remaining, has_final_consonant, false, false, use_modern_style);
    }

    // Handle qu-initial: first 'u' is part of consonant
    if has_qu_initial && vowels.len() >= 2 && vowels[0].key == keys::U {
        let remaining = &vowels[1..];
        return find_mark_position_impl(remaining, has_final_consonant, false, false, use_modern_style);
    }

    find_mark_position_impl(vowels, has_final_consonant, has_qu_initial, has_gi_initial, use_modern_style)
}

fn find_mark_position_impl(
    vowels: &[VowelInfo],
    has_final_consonant: bool,
    has_qu_initial: bool,
    has_gi_initial: bool,
    use_modern_style: bool,
) -> usize {
    match vowels.len() {
        0 => 0,
        1 => vowels[0].pos,
        2 => {
            let (v1, v2) = (&vowels[0], &vowels[1]);

            // Rule 1: With final consonant → second vowel
            if has_final_consonant {
                return v2.pos;
            }

            // Rule 2: If first has tone modifier and second doesn't → first
            if v1.has_tone_mod && !v2.has_tone_mod {
                return v1.pos;
            }
            // If second has tone modifier → second
            if v2.has_tone_mod {
                return v2.pos;
            }

            // Rule 3: Special patterns
            // ia: first unless gi-initial
            if v1.key == keys::I && v2.key == keys::A {
                return if has_gi_initial { v2.pos } else { v1.pos };
            }
            // ua: first unless qu-initial
            if v1.key == keys::U && v2.key == keys::A {
                return if has_qu_initial { v2.pos } else { v1.pos };
            }
            // uy with qu: always second
            if v1.key == keys::U && v2.key == keys::Y && has_qu_initial {
                return v2.pos;
            }

            // Rule 4: Pattern lookup
            let pair = [v1.key, v2.key];

            // Check second patterns (oa, oe, uy without qu, etc.)
            if TONE_SECOND_PATTERNS.iter().any(|p| p[0] == pair[0] && p[1] == pair[1]) {
                // Modern/traditional style affects oa, oe, uy
                let is_style_pattern = matches!(
                    (v1.key, v2.key),
                    (keys::O, keys::A) | (keys::O, keys::E) | (keys::U, keys::Y)
                );
                if is_style_pattern {
                    return if use_modern_style { v2.pos } else { v1.pos };
                }
                return v2.pos;
            }

            // Check first patterns
            if TONE_FIRST_PATTERNS.iter().any(|p| p[0] == pair[0] && p[1] == pair[1]) {
                return v1.pos;
            }

            // Default: second
            v2.pos
        }
        3 => {
            // Triphthongs: usually middle vowel gets the mark
            // Exception: uyê → last vowel
            let (v1, v2, v3) = (&vowels[0], &vowels[1], &vowels[2]);

            // uyê → last
            if v1.key == keys::U && v2.key == keys::Y && v3.key == keys::E {
                return v3.pos;
            }

            // Check for tone modifiers
            if v1.has_tone_mod { return v1.pos; }
            if v2.has_tone_mod { return v2.pos; }
            if v3.has_tone_mod { return v3.pos; }

            // Default: middle
            v2.pos
        }
        _ => {
            // 4+ vowels: use middle with tone modifier, else middle
            let mid = vowels.len() / 2;
            if vowels[mid].has_tone_mod {
                return vowels[mid].pos;
            }
            for v in vowels {
                if v.has_tone_mod {
                    return v.pos;
                }
            }
            vowels[mid].pos
        }
    }
}

/// Horn placement result
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HornPlacement {
    /// Apply horn to both vowels (ươ compound)
    Both,
    /// Apply horn to first vowel only
    First,
    /// Apply horn/breve to second vowel only
    Second,
}

/// Find which vowel(s) should receive horn modifier
///
/// Returns list of positions that should get horn
pub fn find_horn_positions(vowel_keys: &[u16], vowel_positions: &[usize]) -> Vec<usize> {
    if vowel_positions.is_empty() {
        return vec![];
    }

    // Check adjacent pairs
    for i in 0..vowel_positions.len().saturating_sub(1) {
        let pos1 = vowel_positions[i];
        let pos2 = vowel_positions[i + 1];

        // Must be adjacent
        if pos2 != pos1 + 1 {
            continue;
        }

        let k1 = vowel_keys.get(i).copied().unwrap_or(0);
        let k2 = vowel_keys.get(i + 1).copied().unwrap_or(0);

        // uo or ou → both get horn (ươ compound)
        if (k1 == keys::U && k2 == keys::O) || (k1 == keys::O && k2 == keys::U) {
            return vec![pos1, pos2];
        }

        // uu → first gets horn (ưu)
        if k1 == keys::U && k2 == keys::U {
            return vec![pos1];
        }

        // oa → second gets breve (oă)
        if k1 == keys::O && k2 == keys::A {
            return vec![pos2];
        }
    }

    // Default: find last u or o
    for &pos in vowel_positions.iter().rev() {
        let idx = vowel_positions.iter().position(|&p| p == pos).unwrap_or(0);
        let k = vowel_keys.get(idx).copied().unwrap_or(0);
        if k == keys::U || k == keys::O {
            return vec![pos];
        }
    }

    // Fall back to last 'a' for breve
    if let Some(&pos) = vowel_positions.last() {
        let idx = vowel_positions.iter().position(|&p| p == pos).unwrap_or(0);
        let k = vowel_keys.get(idx).copied().unwrap_or(0);
        if k == keys::A {
            return vec![pos];
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vowel(key: u16, pos: usize, has_tone: bool) -> VowelInfo {
        VowelInfo { key, pos, has_tone_mod: has_tone }
    }

    #[test]
    fn test_single_vowel() {
        let vowels = vec![vowel(keys::A, 0, false)];
        assert_eq!(find_mark_position(&vowels, false, false, false, true), 0);
    }

    #[test]
    fn test_ai_pattern() {
        let vowels = vec![vowel(keys::A, 0, false), vowel(keys::I, 1, false)];
        assert_eq!(find_mark_position(&vowels, false, false, false, true), 0); // ai → first
    }

    #[test]
    fn test_oa_modern() {
        let vowels = vec![vowel(keys::O, 0, false), vowel(keys::A, 1, false)];
        assert_eq!(find_mark_position(&vowels, false, false, false, true), 1); // oa modern → second
    }

    #[test]
    fn test_oa_traditional() {
        let vowels = vec![vowel(keys::O, 0, false), vowel(keys::A, 1, false)];
        assert_eq!(find_mark_position(&vowels, false, false, false, false), 0); // oa traditional → first
    }

    #[test]
    fn test_with_final_consonant() {
        let vowels = vec![vowel(keys::O, 0, false), vowel(keys::A, 1, false)];
        assert_eq!(find_mark_position(&vowels, true, false, false, true), 1); // always second with final
    }
}
