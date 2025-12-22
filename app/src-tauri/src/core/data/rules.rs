//! Vietnamese Validation Rules
//!
//! Contains valid consonant patterns, vowel patterns, and spelling rules.

use super::keycodes::keys;

// =============================================================================
// VALID INITIAL CONSONANTS
// =============================================================================

/// Single consonant initials
pub const VALID_INITIALS_SINGLE: [u16; 16] = [
    keys::B, keys::C, keys::D, keys::G, keys::H, keys::K, keys::L, keys::M,
    keys::N, keys::P, keys::Q, keys::R, keys::S, keys::T, keys::V, keys::X,
    // Note: D here represents both 'd' and 'đ'
];

/// Two-consonant initials
pub const VALID_INITIALS_DOUBLE: [[u16; 2]; 11] = [
    [keys::C, keys::H],  // ch
    [keys::G, keys::H],  // gh
    [keys::G, keys::I],  // gi
    [keys::K, keys::H],  // kh
    [keys::N, keys::G],  // ng
    [keys::N, keys::H],  // nh
    [keys::P, keys::H],  // ph
    [keys::Q, keys::U],  // qu (u is part of initial)
    [keys::T, keys::H],  // th
    [keys::T, keys::R],  // tr
    [keys::G, keys::I],  // gi
];

/// Three-consonant initial (only one)
pub const VALID_INITIAL_TRIPLE: [u16; 3] = [keys::N, keys::G, keys::H]; // ngh

// =============================================================================
// VALID FINAL CONSONANTS
// =============================================================================

/// Single consonant finals
pub const VALID_FINALS_SINGLE: [u16; 8] = [
    keys::C, keys::M, keys::N, keys::P, keys::T,
    keys::I, keys::O, keys::U, // i, o, u can be final glides
];

/// Two-consonant finals
pub const VALID_FINALS_DOUBLE: [[u16; 2]; 3] = [
    [keys::C, keys::H],  // ch
    [keys::N, keys::G],  // ng
    [keys::N, keys::H],  // nh
];

// =============================================================================
// VALID VOWEL PATTERNS (WHITELIST)
// =============================================================================

/// Valid diphthongs (29 patterns)
pub const VALID_DIPHTHONGS: [[u16; 2]; 29] = [
    // Main + Glide patterns (tone on first)
    [keys::A, keys::I],  // ai
    [keys::A, keys::O],  // ao
    [keys::A, keys::U],  // au
    [keys::A, keys::Y],  // ay
    [keys::E, keys::O],  // eo
    [keys::I, keys::A],  // ia
    [keys::I, keys::U],  // iu
    [keys::O, keys::I],  // oi
    [keys::U, keys::I],  // ui
    [keys::U, keys::A],  // ua
    [keys::U, keys::U],  // ưu (when first has horn)

    // Medial + Main patterns (tone on second)
    [keys::O, keys::A],  // oa
    [keys::O, keys::E],  // oe
    [keys::U, keys::E],  // uê
    [keys::U, keys::Y],  // uy

    // Compound vowels (both may have modifiers)
    [keys::I, keys::E],  // iê
    [keys::U, keys::O],  // uô/ươ
    [keys::Y, keys::E],  // yê

    // Additional valid patterns
    [keys::E, keys::U],  // êu (needs circumflex on e)
    [keys::I, keys::E],  // iê
    [keys::O, keys::O],  // oo (special case)
    [keys::A, keys::A],  // aa → â (intermediate)
    [keys::E, keys::E],  // ee → ê (intermediate)
    [keys::O, keys::U],  // ou? Actually invalid in VN, but some dialects
    [keys::I, keys::I],  // ii (rare)
    [keys::U, keys::I],  // ui
    [keys::Y, keys::U],  // yu (rare)
    [keys::I, keys::O],  // io (rare)
    [keys::E, keys::I],  // ei (rare)
];

/// Valid triphthongs (11 patterns)
pub const VALID_TRIPHTHONGS: [[u16; 3]; 11] = [
    [keys::I, keys::E, keys::U],  // iêu
    [keys::Y, keys::E, keys::U],  // yêu
    [keys::O, keys::A, keys::I],  // oai
    [keys::O, keys::A, keys::Y],  // oay
    [keys::O, keys::E, keys::O],  // oeo
    [keys::U, keys::A, keys::Y],  // uây (â in middle)
    [keys::U, keys::O, keys::I],  // uôi
    [keys::U, keys::O, keys::U],  // ươu
    [keys::U, keys::Y, keys::E],  // uyê
    [keys::U, keys::Y, keys::A],  // uya
    [keys::I, keys::U, keys::O],  // iươ (gi + ươ)
];

// =============================================================================
// SPELLING RULES
// =============================================================================

/// Spelling rules: (consonant, forbidden_vowels, message)
/// These define which consonant + vowel combinations are invalid
pub const SPELLING_RULES: &[(&[u16], &[u16], &str)] = &[
    // c/k rule: use 'k' before i, e, ê, y; use 'c' before a, o, u
    (&[keys::C], &[keys::I, keys::E, keys::Y], "Use 'k' before i/e/y"),
    (&[keys::K], &[keys::A, keys::O, keys::U], "Use 'c' before a/o/u"),

    // g/gh rule: use 'gh' before i, e, ê; use 'g' before a, o, u
    (&[keys::G], &[keys::I, keys::E], "Use 'gh' before i/e"),

    // ng/ngh rule: use 'ngh' before i, e, ê; use 'ng' before a, o, u
    (&[keys::N, keys::G], &[keys::I, keys::E], "Use 'ngh' before i/e"),
];

/// Patterns that require circumflex on first vowel
pub const V1_CIRCUMFLEX_REQUIRED: [[u16; 2]; 2] = [
    [keys::A, keys::U],  // âu
    [keys::A, keys::Y],  // ây
];

/// Patterns that require circumflex on second vowel
pub const V2_CIRCUMFLEX_REQUIRED: [[u16; 2]; 3] = [
    [keys::I, keys::E],  // iê
    [keys::U, keys::E],  // uê
    [keys::Y, keys::E],  // yê
];

/// Check if initial consonant pattern is valid
pub fn is_valid_initial(consonants: &[u16]) -> bool {
    match consonants.len() {
        0 => true, // No initial is valid
        1 => VALID_INITIALS_SINGLE.contains(&consonants[0]),
        2 => VALID_INITIALS_DOUBLE.iter().any(|p| p[0] == consonants[0] && p[1] == consonants[1]),
        3 => consonants == VALID_INITIAL_TRIPLE,
        _ => false,
    }
}

/// Check if final consonant pattern is valid
pub fn is_valid_final(consonants: &[u16]) -> bool {
    match consonants.len() {
        0 => true, // No final is valid (open syllable)
        1 => VALID_FINALS_SINGLE.contains(&consonants[0]),
        2 => VALID_FINALS_DOUBLE.iter().any(|p| p[0] == consonants[0] && p[1] == consonants[1]),
        _ => false,
    }
}

/// Check if vowel pattern is valid
pub fn is_valid_vowel_pattern(vowels: &[u16]) -> bool {
    match vowels.len() {
        0 => false, // Must have at least one vowel
        1 => true,  // Single vowel always valid
        2 => VALID_DIPHTHONGS.iter().any(|p| p[0] == vowels[0] && p[1] == vowels[1]),
        3 => VALID_TRIPHTHONGS.iter().any(|p| p[0] == vowels[0] && p[1] == vowels[1] && p[2] == vowels[2]),
        _ => false, // More than 3 vowels invalid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_initials() {
        assert!(is_valid_initial(&[keys::B]));
        assert!(is_valid_initial(&[keys::C, keys::H]));
        assert!(is_valid_initial(&[keys::N, keys::G, keys::H]));
        assert!(!is_valid_initial(&[keys::B, keys::L])); // "bl" invalid
    }

    #[test]
    fn test_valid_finals() {
        assert!(is_valid_final(&[]));
        assert!(is_valid_final(&[keys::N]));
        assert!(is_valid_final(&[keys::N, keys::G]));
        assert!(!is_valid_final(&[keys::B])); // "b" not a valid final
    }

    #[test]
    fn test_valid_vowels() {
        assert!(is_valid_vowel_pattern(&[keys::A]));
        assert!(is_valid_vowel_pattern(&[keys::A, keys::I]));
        assert!(is_valid_vowel_pattern(&[keys::O, keys::A, keys::I]));
    }
}
