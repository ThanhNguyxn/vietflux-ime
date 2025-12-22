//! Diacritic Transformations
//!
//! Handles applying tone modifiers, marks, and strokes to the buffer.

use super::buffer::CompositionBuffer;
use super::data::keycodes::keys;
use super::data::unicode::{tone, mark};
use super::phonology::{find_mark_position, find_horn_positions, VowelInfo};

/// Transformation result
#[derive(Debug)]
pub struct TransformResult {
    /// Positions that were modified
    pub modified_positions: Vec<usize>,
    /// Whether transformation was applied
    pub applied: bool,
}

impl TransformResult {
    pub fn none() -> Self {
        Self {
            modified_positions: vec![],
            applied: false,
        }
    }

    pub fn success(positions: Vec<usize>) -> Self {
        Self {
            modified_positions: positions,
            applied: true,
        }
    }
}

/// Apply tone modifier (circumflex, horn, breve) to buffer
///
/// # Arguments
/// * `buffer` - The composition buffer
/// * `tone_value` - Tone modifier value (CIRCUMFLEX or HORN)
/// * `trigger_key` - The key that triggered this transformation
pub fn apply_tone_modifier(
    buffer: &mut CompositionBuffer,
    tone_value: u8,
    trigger_key: u16,
) -> TransformResult {
    let vowel_positions = buffer.find_vowels();
    if vowel_positions.is_empty() {
        return TransformResult::none();
    }

    let vowel_keys: Vec<u16> = vowel_positions
        .iter()
        .map(|&pos| buffer.get(pos).unwrap().key)
        .collect();

    // For circumflex triggered by same vowel (aa, ee, oo)
    if tone_value == tone::CIRCUMFLEX {
        if let Some(&last_pos) = vowel_positions.last() {
            if let Some(unit) = buffer.get(last_pos) {
                if unit.key == trigger_key && unit.tone_mod == tone::NONE {
                    // Apply circumflex to last matching vowel
                    if let Some(unit) = buffer.get_mut(last_pos) {
                        unit.tone_mod = tone::CIRCUMFLEX;
                        return TransformResult::success(vec![last_pos]);
                    }
                }
            }
        }
    }

    // For horn modifier (w key)
    if tone_value == tone::HORN {
        let targets = find_horn_positions(&vowel_keys, &vowel_positions);
        if targets.is_empty() {
            return TransformResult::none();
        }

        let mut modified = vec![];
        for pos in targets {
            if let Some(unit) = buffer.get_mut(pos) {
                if unit.tone_mod == tone::NONE {
                    unit.tone_mod = tone::HORN;
                    modified.push(pos);
                }
            }
        }

        if modified.is_empty() {
            TransformResult::none()
        } else {
            TransformResult::success(modified)
        }
    } else {
        TransformResult::none()
    }
}

/// Apply tone mark (acute, grave, hook, tilde, dot) to buffer
///
/// # Arguments
/// * `buffer` - The composition buffer
/// * `mark_value` - Mark value (ACUTE, GRAVE, HOOK, TILDE, DOT)
/// * `use_modern_style` - true for modern tone placement
pub fn apply_mark(
    buffer: &mut CompositionBuffer,
    mark_value: u8,
    use_modern_style: bool,
) -> TransformResult {
    let vowel_positions = buffer.find_vowels();
    if vowel_positions.is_empty() {
        return TransformResult::none();
    }

    // Collect vowel info for phonology analysis
    let vowels: Vec<VowelInfo> = vowel_positions
        .iter()
        .map(|&pos| {
            let unit = buffer.get(pos).unwrap();
            VowelInfo {
                key: unit.key,
                pos,
                has_tone_mod: unit.tone_mod != tone::NONE,
            }
        })
        .collect();

    // Check for qu/gi initial
    let keys = buffer.keys();
    let has_qu_initial = keys.len() >= 2 && keys[0] == keys::Q && keys[1] == keys::U;
    let has_gi_initial = keys.len() >= 2 && keys[0] == keys::G && keys[1] == keys::I;

    // Check for final consonant
    let last_vowel_pos = vowel_positions.last().copied().unwrap_or(0);
    let has_final = buffer.len() > last_vowel_pos + 1;

    // Find position using phonology rules
    let pos = find_mark_position(&vowels, has_final, has_qu_initial, has_gi_initial, use_modern_style);

    // Clear any existing marks
    for &vpos in &vowel_positions {
        if let Some(unit) = buffer.get_mut(vpos) {
            unit.mark_type = mark::NONE;
        }
    }

    // Apply new mark
    if let Some(unit) = buffer.get_mut(pos) {
        unit.mark_type = mark_value;
        return TransformResult::success(vec![pos]);
    }

    TransformResult::none()
}

/// Apply stroke transformation (d → đ)
pub fn apply_stroke(buffer: &mut CompositionBuffer) -> TransformResult {
    // Find first un-stroked 'd'
    for i in 0..buffer.len() {
        if let Some(unit) = buffer.get_mut(i) {
            if unit.key == keys::D && !unit.is_stroked {
                unit.is_stroked = true;
                return TransformResult::success(vec![i]);
            }
        }
    }
    TransformResult::none()
}

/// Revert stroke transformation
pub fn revert_stroke(buffer: &mut CompositionBuffer) -> TransformResult {
    // Find stroked 'd' and un-stroke it
    for i in 0..buffer.len() {
        if let Some(unit) = buffer.get_mut(i) {
            if unit.key == keys::D && unit.is_stroked {
                unit.is_stroked = false;
                return TransformResult::success(vec![i]);
            }
        }
    }
    TransformResult::none()
}

/// Revert tone modifier (circumflex/horn)
pub fn revert_tone(buffer: &mut CompositionBuffer, target_key: u16) -> TransformResult {
    let vowel_positions = buffer.find_vowels();

    for pos in vowel_positions.iter().rev() {
        if let Some(unit) = buffer.get_mut(*pos) {
            if unit.key == target_key && unit.tone_mod != tone::NONE {
                unit.tone_mod = tone::NONE;
                return TransformResult::success(vec![*pos]);
            }
        }
    }

    TransformResult::none()
}

/// Revert mark
pub fn revert_mark(buffer: &mut CompositionBuffer) -> TransformResult {
    let vowel_positions = buffer.find_vowels();

    for pos in vowel_positions.iter().rev() {
        if let Some(unit) = buffer.get_mut(*pos) {
            if unit.mark_type != mark::NONE {
                unit.mark_type = mark::NONE;
                return TransformResult::success(vec![*pos]);
            }
        }
    }

    TransformResult::none()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_buffer(s: &str) -> CompositionBuffer {
        let mut buf = CompositionBuffer::new();
        for c in s.chars() {
            if let Some(key) = super::super::data::keycodes::from_char(c) {
                buf.push(InputUnit::new(key, c.is_uppercase()), c);
            }
        }
        buf
    }

    #[test]
    fn test_apply_stroke() {
        let mut buf = make_buffer("do");
        let result = apply_stroke(&mut buf);
        assert!(result.applied);
        assert!(buf.get(0).unwrap().is_stroked);
        assert_eq!(buf.to_string(), "đo");
    }

    #[test]
    fn test_apply_mark() {
        let mut buf = make_buffer("an");
        let result = apply_mark(&mut buf, mark::ACUTE, true);
        assert!(result.applied);
        assert_eq!(buf.to_string(), "án");
    }
}
