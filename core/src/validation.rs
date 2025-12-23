//! Vietnamese Syllable Validation
//!
//! Validates if a string is a valid Vietnamese syllable.
//! Based on Vietnamese phonology rules with Foreign Word Detection.

use crate::chars::{self, VowelMod, REVERSE_MAP};

// ============================================================
// PHONOLOGY CONSTANTS
// ============================================================

/// Valid initial consonants (phụ âm đầu) - 16 single + 10 pairs + ngh
pub const VALID_INITIALS: &[&str] = &[
    // Single consonants
    "", "b", "c", "d", "đ", "g", "h", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "x",
    // Consonant pairs
    "ch", "gh", "gi", "kh", "ng", "nh", "ph", "th", "tr", "qu", // Special triple
    "ngh",
];

/// Valid final consonants (phụ âm cuối)  
/// Note: 'k' is allowed for ethnic minority words (Đắk, Lắk, Búk)
pub const VALID_FINALS: &[&str] = &["", "c", "ch", "m", "n", "ng", "nh", "p", "t", "k"];

/// Valid vowel nuclei (including diphthongs and triphthongs)
pub const VALID_VOWEL_PATTERNS: &[&str] = &[
    // Single vowels (base)
    "a", "ă", "â", "e", "ê", "i", "o", "ô", "ơ", "u", "ư", "y",
    // Diphthongs (nguyên âm đôi)
    "ai", "ao", "au", "ay", "âu", "ây", "eo", "êu", "ia", "iê", "iu", "oa", "oă", "oe", "oi", "oo",
    "ôi", "ơi", "ua", "uâ", "uê", "ui", "uo", "uô", "uơ", "ươ", "ưa", "ưi", "ưu", "ya", "yê", "uy",
    // Triphthongs (nguyên âm ba)
    "iêu", "oai", "oay", "oeo", "uây", "uôi", "ươi", "ươu", "yêu",
    // With qu- prefix patterns
    "uya", "uyê", "uyu",
    // Special triphthongs (from GoNhanh analysis)
    "uêu", // nguều ngoào
    "oao", // ngoào
];

/// Foreign word consonant clusters (not valid in Vietnamese)
#[allow(dead_code)]
const FOREIGN_CLUSTERS: &[&str] = &[
    "tr", "pr", "cr", "br", "dr", "gr", "fr", // After finals
    "st", "sp", "sc", "sk", "sm", "sn", "sl", "sw", "bl", "cl", "fl", "gl", "pl", "ck", "dg",
    "ght", "wh", "wr",
];

/// Invalid vowel patterns (impossible in Vietnamese)
#[allow(dead_code)]
const INVALID_VOWEL_PATTERNS: &[&str] = &[
    // These vowel combinations don't exist in Vietnamese
    "eư", "oư", "iư", // ư cannot follow e, o, i directly
    "ưe", "ưo", "ưy", // Invalid ư combinations
    "ou", "yo", // English-like patterns
    "ea", "ei", // English diphthongs (NOT "ie" - that's valid: tiếp, tiếng)
];

// ============================================================
// VALIDATION RESULT
// ============================================================

/// Validation result with detailed failure reason
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    InvalidInitial,
    InvalidFinal,
    InvalidSpelling,
    InvalidVowelPattern,
    NoVowel,
    ForeignWord,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }
}

// ============================================================
// VALIDATION FUNCTIONS
// ============================================================

/// Full validation with detailed result
#[allow(clippy::option_if_let_else)]
pub fn validate(s: &str) -> ValidationResult {
    if s.is_empty() {
        return ValidationResult::NoVowel;
    }

    let lower = s.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();

    // Rule 1: Must have vowel
    if !has_vowel(&chars) {
        return ValidationResult::NoVowel;
    }

    // Rule 2: Check for foreign word patterns first
    if is_foreign_pattern(&lower) {
        return ValidationResult::ForeignWord;
    }

    // Try to parse syllable structure: Initial + Vowel + Final
    if let Some((initial, vowel, final_c)) = parse_syllable(&lower) {
        // Rule 3: Valid initial consonant
        if !VALID_INITIALS.contains(&initial) {
            return ValidationResult::InvalidInitial;
        }

        // Rule 4: Valid final consonant
        if !VALID_FINALS.contains(&final_c) {
            return ValidationResult::InvalidFinal;
        }

        // Rule 5: Spelling rules (c/k/g restrictions)
        if !check_spelling_rules(initial, vowel) {
            return ValidationResult::InvalidSpelling;
        }

        // Rule 6: Valid vowel pattern (diphthong/triphthong)
        let vowel_base = normalize_vowel(vowel);
        if !is_valid_vowel_pattern(&vowel_base) {
            return ValidationResult::InvalidVowelPattern;
        }

        // Rule 7: Breve restriction - ă cannot be followed by another vowel
        // Valid: ăm, ăn, ăng (consonant endings), oă (xoăn)
        // Invalid: ăi, ăo, ău, ăy
        if is_breve_followed_by_vowel(vowel) {
            return ValidationResult::InvalidVowelPattern;
        }

        ValidationResult::Valid
    } else {
        ValidationResult::InvalidSpelling
    }
}

/// Quick check if valid
pub fn is_valid_syllable(s: &str) -> bool {
    validate(s).is_valid()
}

/// Check if pattern looks like a foreign/English word
/// Based on 8 patterns from GoNhanh auto-restore analysis
pub fn is_foreign_word_pattern(buffer: &str, modifier_key: Option<char>) -> bool {
    let lower = buffer.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();

    // Skip check if buffer has Vietnamese diacritics (horn, circumflex, breve)
    // User is typing Vietnamese intentionally
    if has_vietnamese_diacritics(&lower) {
        return false;
    }

    // Skip check for valid typing sequences like "dodo" (typing đô)
    if is_valid_typing_sequence(&lower) {
        return false;
    }

    // ============================================================
    // 8 ENGLISH AUTO-RESTORE PATTERNS (from GoNhanh analysis)
    // ============================================================

    // PATTERN 1: MODIFIER + CONSONANT (not sonorant)
    // "text" → x+t, "expect" → x+p → English
    // Exception: Modifier + sonorant (m,n,ng,nh) → Vietnamese "làm"
    if let Some(mod_key) = modifier_key {
        if is_tone_modifier(mod_key) && chars.len() >= 2 {
            let last = chars[chars.len() - 1];
            // Allow sonorants (m, n) as they're valid Vietnamese endings
            if !matches!(last, 'm' | 'n') && crate::chars::is_consonant(last) {
                // Check if last char would be added after a vowel (vietnamese) or consonant (english)
                if chars.len() >= 2 {
                    let second_last = chars[chars.len() - 2];
                    if crate::chars::is_consonant(second_last) {
                        return true; // Consonant cluster after modifier = English
                    }
                }
            }
        }
    }

    // PATTERN 2: W AT START + CONSONANT
    // "water", "window", "world" → English
    // Exception: "ưng", "ưn" (w + sonorant final) → Vietnamese
    if lower.starts_with('w') && chars.len() > 1 {
        let second = chars[1];
        if crate::chars::is_consonant(second) && second != 'h' {
            return true;
        }
    }

    // PATTERN 3: EI VOWEL PAIR
    // "their", "weird" → English (Vietnamese doesn't have "ei" diphthong)
    if lower.contains("ei") {
        return true;
    }

    // PATTERN 4: P INITIAL + AI PATTERN
    // "pair", "paint" → English (P initial is rare in pure Vietnamese)
    if lower.starts_with('p') && !lower.starts_with("ph") && lower.contains("ai") {
        return true;
    }

    // PATTERN 5: W AS FINAL
    // "raw", "law", "saw" → English (W cannot be final in Vietnamese)
    if lower.ends_with('w') {
        return true;
    }

    // PATTERN 6: F INITIAL
    // "fix", "file", "focus" → English (Vietnamese uses PH for /f/)
    if lower.starts_with('f') {
        return true;
    }

    // PATTERN 7: MODIFIER + K ENDING
    // "risk", "disk", "task" → English
    // Exception: Ethnic minority with breve (Đắk, Lắk) - but those have diacritics so skip
    if modifier_key.map(is_tone_modifier).unwrap_or(false) && lower.ends_with('k') {
        // Check if NOT ethnic minority pattern (starts with đ, l, b)
        if !matches!(chars.first(), Some('đ') | Some('l') | Some('b')) {
            return true;
        }
    }

    // PATTERN 8: DOUBLE VOWEL + CONSONANT (English "looks", "took")
    // Different from Telex "aa" → "â" which creates circumflex
    if lower.contains("oo") && chars.len() > 2 {
        // If followed by consonant (not another vowel), likely English
        if let Some(pos) = lower.find("oo") {
            if pos + 2 < chars.len() {
                let after = chars[pos + 2];
                if crate::chars::is_consonant(after) {
                    return true;
                }
            }
        }
    }

    // ============================================================
    // EXISTING CHECKS (kept from before)
    // ============================================================

    // Check for invalid vowel patterns (eư, oư, etc)
    for pattern in INVALID_VOWEL_PATTERNS {
        if lower.contains(pattern) {
            return true;
        }
    }

    // Check for foreign consonant clusters (not at start)
    for cluster in FOREIGN_CLUSTERS {
        if lower.contains(cluster) {
            // Allow "tr" at the start (Vietnamese "tr")
            if *cluster == "tr" && lower.starts_with("tr") {
                continue;
            }
            // Check if cluster is found
            if let Some(pos) = lower.find(cluster) {
                if pos == 0 {
                    return true;
                }

                // Check if cluster is after a vowel (foreign pattern)
                let prev_char = lower.chars().nth(pos - 1);
                if prev_char.map(crate::chars::is_vowel).unwrap_or(false) {
                    return true;
                }
            }
        }
    }

    // English prefix patterns: de+s (describe, design)
    if lower.starts_with("de") && modifier_key == Some('s') && lower.len() > 3 {
        return true;
    }

    // pr- prefix not valid in Vietnamese
    if lower.starts_with("pr") && lower.len() > 3 {
        return true;
    }

    // -tion, -sion endings (clearly English)
    if lower.ends_with("tion") || lower.ends_with("sion") {
        return true;
    }

    false
}

/// Check if character is a tone modifier key (Telex)
fn is_tone_modifier(c: char) -> bool {
    matches!(c, 's' | 'f' | 'r' | 'x' | 'j')
}

// ============================================================
// HELPER FUNCTIONS
// ============================================================

/// Check if string has any vowel
fn has_vowel(chars: &[char]) -> bool {
    chars.iter().any(|&c| crate::chars::is_vowel(c))
}

/// Check if string has Vietnamese diacritics (horn, circumflex, breve)
fn has_vietnamese_diacritics(s: &str) -> bool {
    for c in s.chars() {
        if let Some(&(_, modifier, tone)) = REVERSE_MAP.get(&c) {
            if modifier != VowelMod::None || tone != crate::chars::ToneMark::None {
                return true;
            }
        }
        // Direct check for Vietnamese special chars
        if matches!(c, 'ư' | 'ơ' | 'â' | 'ê' | 'ô' | 'ă' | 'đ') {
            return true;
        }
    }
    false
}

/// Check if this is a valid Vietnamese typing sequence
/// e.g., "dodo" could be typing "đô" (d→o→d→o where second d triggers stroke)
fn is_valid_typing_sequence(s: &str) -> bool {
    let lower = s.to_lowercase();

    // Patterns that look foreign but are valid Vietnamese typing sequences:

    // "dodo", "dodi" - typing đô, đồ, etc. with dd for đ
    if lower.starts_with("do") && lower.chars().nth(2) == Some('d') {
        return true;
    }

    // "soso", "lolo" - typing có repeated consonants for emphasis
    // But these could also be valid Vietnamese with tones added later
    let chars: Vec<char> = lower.chars().collect();
    if chars.len() >= 4 {
        // Check if it's a simple repetition pattern like "xoxo"
        if chars[0] == chars[2] && chars[1] == chars[3] {
            // If first char is consonant and second is vowel, could be valid
            if !crate::chars::is_vowel(chars[0]) && crate::chars::is_vowel(chars[1]) {
                return true;
            }
        }
    }

    false
}

/// Check for foreign word patterns
fn is_foreign_pattern(s: &str) -> bool {
    // Consonant clusters that don't exist in Vietnamese
    for cluster in FOREIGN_CLUSTERS {
        if s.contains(cluster) {
            // But allow "tr" at the start (Vietnamese "tr")
            if *cluster == "tr" && s.starts_with("tr") {
                continue;
            }
            return true;
        }
    }
    false
}

/// Parse syllable into (initial, vowel, final) components
fn parse_syllable(s: &str) -> Option<(&str, &str, &str)> {
    // Try longest initial first
    let mut initials: Vec<&&str> = VALID_INITIALS.iter().collect();
    initials.sort_by_key(|b| std::cmp::Reverse(b.len()));

    for initial in initials {
        if let Some(rest) = s.strip_prefix(initial) {
            // Try to find vowel + final
            if let Some((vowel, final_c)) = parse_vowel_final(rest) {
                return Some((initial, vowel, final_c));
            }
        }
    }

    None
}

/// Parse vowel and final consonant from string
fn parse_vowel_final(s: &str) -> Option<(&str, &str)> {
    if s.is_empty() {
        return None;
    }

    // Try longest final first
    let mut finals: Vec<&&str> = VALID_FINALS.iter().collect();
    finals.sort_by_key(|b| std::cmp::Reverse(b.len()));

    for final_c in finals {
        if s.ends_with(final_c) {
            let vowel_len = s.len() - final_c.len();
            if vowel_len > 0 {
                let vowel = &s[..vowel_len];
                // Check if all chars in vowel part are vowels (ignoring tones)
                if vowel
                    .chars()
                    .all(|c| crate::chars::is_vowel(c) || is_glide(c))
                {
                    return Some((vowel, final_c));
                }
            } else if final_c.is_empty() {
                // No final, entire string is vowel
                if s.chars().all(|c| crate::chars::is_vowel(c) || is_glide(c)) {
                    return Some((s, ""));
                }
            }
        }
    }

    None
}

/// Check if character is a glide (y/w used as consonant)
fn is_glide(c: char) -> bool {
    matches!(c, 'y' | 'w')
}

/// Normalize vowel pattern (remove tones but keep modifiers for pattern matching)
/// e.g., "ều" → "êu" (keeps circumflex, removes grave)
fn normalize_vowel(vowel: &str) -> String {
    vowel
        .chars()
        .map(|c| {
            // Get the character with modifier but without tone
            // Get the character with modifier but without tone
            chars::REVERSE_MAP
                .get(&c.to_ascii_lowercase())
                .map_or(c, |&(base, modifier, _tone)| {
                    // Look up character with same base+modifier but no tone
                    chars::CHAR_MAP
                        .get(&(base, modifier, chars::ToneMark::None))
                        .copied()
                        .unwrap_or(base)
                })
        })
        .collect()
}

/// Check if vowel pattern is valid
fn is_valid_vowel_pattern(vowel_base: &str) -> bool {
    // Single vowels are always valid
    if vowel_base.len() == 1 {
        return true;
    }

    // Check against known patterns
    VALID_VOWEL_PATTERNS.contains(&vowel_base)
}

/// Check if breve (ă) is followed by another vowel
/// This is invalid in Vietnamese - ă can only be followed by consonants
/// Valid: ăm, ăn, ăng, ănh, ăp, ăt, ăc (consonant endings)
/// Valid: oă (in "xoăn" etc. - o is before ă)
/// Invalid: ăi, ăo, ău, ăy (breve + vowel)
fn is_breve_followed_by_vowel(vowel: &str) -> bool {
    let chars: Vec<char> = vowel.chars().collect();
    for i in 0..chars.len().saturating_sub(1) {
        // Check if current char is ă (breve)
        if chars[i] == 'ă' || chars::get_base(chars[i]) == 'a' && has_breve(chars[i]) {
            // And next char is a vowel
            if crate::chars::is_vowel(chars[i + 1]) {
                return true;
            }
        }
    }
    false
}

/// Check if character has breve modifier
fn has_breve(c: char) -> bool {
    matches!(
        c,
        'ă' | 'ắ' | 'ằ' | 'ẳ' | 'ẵ' | 'ặ' | 'Ă' | 'Ắ' | 'Ằ' | 'Ẳ' | 'Ẵ' | 'Ặ'
    )
}

/// Check Vietnamese spelling rules
fn check_spelling_rules(initial: &str, vowel: &str) -> bool {
    let vowel_first = vowel.chars().next().unwrap_or(' ');
    let vowel_base = chars::get_base(vowel_first);

    match initial {
        // "c" can only precede non e/ê/i/y vowels (use "k" for those)
        "c" => !matches!(vowel_base, 'e' | 'ê' | 'i' | 'y'),

        // "k" can only precede e/ê/i/y vowels
        "k" => matches!(vowel_base, 'e' | 'ê' | 'i' | 'y'),

        // "g" cannot precede e/ê/i (use "gh" for those)
        "g" => !matches!(vowel_base, 'e' | 'ê' | 'i'),

        // "gh" can only precede e/ê/i
        "gh" => matches!(vowel_base, 'e' | 'ê' | 'i'),

        // "ng" cannot precede e/ê/i (use "ngh" for those)
        "ng" => !matches!(vowel_base, 'e' | 'ê' | 'i'),

        // "ngh" can only precede e/ê/i
        "ngh" => matches!(vowel_base, 'e' | 'ê' | 'i'),

        _ => true,
    }
}

/// Check if character sequence could become valid Vietnamese
/// Used during typing to allow incomplete words
pub fn is_potentially_valid(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }

    let lower = s.to_lowercase();

    // Check if it's clearly a foreign word
    if is_foreign_word_pattern(&lower, None) {
        return false;
    }

    // Check if any valid syllable could start with this
    for initial in VALID_INITIALS {
        if initial.starts_with(&lower) || lower.starts_with(initial) {
            return true;
        }
    }

    // Could be mid-vowel typing
    true
}

/// Check if word has 'gi' initial
pub fn has_gi_initial(s: &str) -> bool {
    let lower = s.to_lowercase();
    if !lower.starts_with("gi") {
        return false;
    }
    // Must be followed by a vowel or nothing (though "gi" alone is valid)
    // Actually "gi" + vowel is the main case where tone shifts
    // e.g. "gia", "giá", "giờ"
    // If just "gi", tone is on 'i' (handled normally)
    true
}

/// Check if word has 'qu' initial
pub fn has_qu_initial(s: &str) -> bool {
    s.to_lowercase().starts_with("qu")
}

/// Check if word ends at a valid boundary
pub fn is_word_boundary(ch: char) -> bool {
    ch.is_whitespace()
        || matches!(
            ch,
            '.' | ','
                | '!'
                | '?'
                | ':'
                | ';'
                | '"'
                | '\''
                | '('
                | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | '<'
                | '>'
                | '/'
                | '\\'
                | '='
                | '+'
                | '-'
                | '*'
                | '@'
                | '#'
                | '$'
                | '%'
                | '^'
                | '&'
                | '|'
                | '~'
                | '`'
        )
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_syllables() {
        assert!(is_valid_syllable("an"));
        assert!(is_valid_syllable("em"));
        assert!(is_valid_syllable("anh"));
        assert!(is_valid_syllable("nam"));
        assert!(is_valid_syllable("xin"));
    }

    #[test]
    fn test_invalid_syllables() {
        assert!(!is_valid_syllable("xyz"));
        assert!(!is_valid_syllable("bcd"));
        assert!(!is_valid_syllable("mmm"));
    }

    #[test]
    fn test_foreign_word_detection() {
        assert!(is_foreign_word_pattern("programming", None));
        assert!(is_foreign_word_pattern("describe", Some('s')));
        assert!(is_foreign_word_pattern("spectrum", None));
        assert!(!is_foreign_word_pattern("việt", None)); // Has horn
        assert!(!is_foreign_word_pattern("xin", None)); // Valid Vietnamese
    }

    #[test]
    fn test_spelling_rules() {
        // c vs k
        assert!(check_spelling_rules("c", "a")); // ca ✓
        assert!(!check_spelling_rules("c", "e")); // ce ✗ (should be ke)
        assert!(check_spelling_rules("k", "e")); // ke ✓

        // g vs gh
        assert!(check_spelling_rules("g", "a")); // ga ✓
        assert!(!check_spelling_rules("g", "e")); // ge ✗ (should be ghe)
        assert!(check_spelling_rules("gh", "e")); // ghe ✓
    }

    #[test]
    fn test_word_boundary() {
        assert!(is_word_boundary(' '));
        assert!(is_word_boundary('.'));
        assert!(is_word_boundary('=')); // Programming
        assert!(is_word_boundary('{')); // Code
        assert!(!is_word_boundary('a'));
    }

    // ============================================================
    // NEW TESTS FOR PHASE 1 ENHANCEMENTS
    // ============================================================

    #[test]
    fn test_ethnic_minority_k_final() {
        // Ethnic minority words with 'k' as final consonant
        assert!(is_valid_syllable("đắk")); // Đắk Lắk province
        assert!(is_valid_syllable("lắk")); // Lắk
        assert!(is_valid_syllable("búk")); // Búk district
    }

    #[test]
    fn test_new_triphthongs() {
        // New triphthong patterns from GoNhanh analysis
        assert!(is_valid_syllable("khuỷu")); // khuỷu tay (elbow) - pattern uyu
        assert!(is_valid_syllable("nguều")); // nguều ngoào - pattern uêu
        assert!(is_valid_syllable("ngoào")); // ngoào - pattern oao
    }

    #[test]
    fn test_breve_followed_by_vowel_invalid() {
        // Breve (ă) cannot be followed by another vowel
        // Valid: ăm, ăn, ăng (consonant endings)
        // Valid: oă (o before ă, as in xoăn)
        // Invalid: ăi, ăo, ău, ăy
        assert!(!is_valid_syllable("tăi")); // Invalid: ă + i
        assert!(!is_valid_syllable("băo")); // Invalid: ă + o
        assert!(!is_valid_syllable("tău")); // Invalid: ă + u

        // But these should be valid (ă + consonant)
        assert!(is_valid_syllable("ăn")); // Valid: eat
        assert!(is_valid_syllable("tăm")); // Valid: toothpick
        assert!(is_valid_syllable("tắc")); // Valid: blocked

        // oă pattern is valid (o before ă)
        assert!(is_valid_syllable("xoăn")); // Valid: curly
    }

    // ============================================================
    // 8 ENGLISH AUTO-RESTORE PATTERN TESTS
    // ============================================================

    #[test]
    fn test_english_pattern_w_initial() {
        // PATTERN 2: W at start + consonant
        assert!(is_foreign_word_pattern("wn", None)); // window start
        assert!(is_foreign_word_pattern("wr", None)); // write
        assert!(is_foreign_word_pattern("wl", None)); // world
    }

    #[test]
    fn test_english_pattern_ei_pair() {
        // PATTERN 3: EI vowel pair (not in Vietnamese)
        assert!(is_foreign_word_pattern("their", None));
        assert!(is_foreign_word_pattern("weird", None));
        assert!(is_foreign_word_pattern("receive", None));
    }

    #[test]
    fn test_english_pattern_f_initial() {
        // PATTERN 6: F initial (Vietnamese uses PH)
        assert!(is_foreign_word_pattern("fix", None));
        assert!(is_foreign_word_pattern("file", None));
        assert!(is_foreign_word_pattern("focus", None));
    }

    #[test]
    fn test_english_pattern_w_final() {
        // PATTERN 5: W as final
        assert!(is_foreign_word_pattern("raw", None));
        assert!(is_foreign_word_pattern("law", None));
        assert!(is_foreign_word_pattern("saw", None));
    }

    #[test]
    fn test_english_pattern_p_ai() {
        // PATTERN 4: P initial + AI pattern
        assert!(is_foreign_word_pattern("pair", None));
        assert!(is_foreign_word_pattern("paint", None));
        // But "phai" without diacritics could still trigger
    }

    #[test]
    fn test_english_pattern_double_oo() {
        // PATTERN 8: Double vowel oo + consonant
        assert!(is_foreign_word_pattern("look", None));
        assert!(is_foreign_word_pattern("took", None));
        assert!(is_foreign_word_pattern("book", None));
    }
}
