//! Vietnamese IME Processor
//!
//! Main input processing pipeline for VietFlux.

use super::buffer::{CompositionBuffer, InputUnit};
use super::data::keycodes::{self, keys};
use super::data::unicode::{tone, mark};
use super::transform::{apply_tone_modifier, apply_mark, apply_stroke};

/// Input method type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputMethod {
    Telex,
    Vni,
}

/// Processing result
#[derive(Debug)]
pub struct ProcessResult {
    /// Action type
    pub action: ProcessAction,
    /// Characters to output
    pub output: String,
    /// Number of backspaces to send
    pub backspace: usize,
}

/// Action type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessAction {
    /// Pass through (no IME processing)
    Passthrough,
    /// Commit output to application
    Commit,
    /// Clear buffer
    Clear,
}

impl ProcessResult {
    pub fn passthrough() -> Self {
        Self {
            action: ProcessAction::Passthrough,
            output: String::new(),
            backspace: 0,
        }
    }

    pub fn commit(output: String, backspace: usize) -> Self {
        Self {
            action: ProcessAction::Commit,
            output,
            backspace,
        }
    }
}

/// Main Vietnamese IME processor
pub struct Processor {
    /// Composition buffer
    buffer: CompositionBuffer,
    /// Input method (Telex or VNI)
    method: InputMethod,
    /// Whether IME is enabled
    enabled: bool,
    /// Use modern tone placement (hoà vs hòa)
    use_modern_style: bool,
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}

impl Processor {
    /// Create a new processor
    pub fn new() -> Self {
        Self {
            buffer: CompositionBuffer::new(),
            method: InputMethod::Telex,
            enabled: true,
            use_modern_style: true,
        }
    }

    /// Set input method
    pub fn set_method(&mut self, method: InputMethod) {
        self.method = method;
    }

    /// Set enabled state
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.buffer.clear();
        }
    }

    /// Set modern style preference
    pub fn set_modern_style(&mut self, modern: bool) {
        self.use_modern_style = modern;
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Get current buffer content as string
    pub fn get_buffer(&self) -> String {
        self.buffer.to_string()
    }

    /// Process a key event
    ///
    /// # Arguments
    /// * `key` - Virtual keycode
    /// * `uppercase` - Whether shift/caps is active
    /// * `ctrl` - Whether ctrl/cmd is pressed
    pub fn process_key(&mut self, key: u16, uppercase: bool, ctrl: bool) -> ProcessResult {
        // If disabled or ctrl held, pass through
        if !self.enabled || ctrl {
            self.buffer.clear();
            return ProcessResult::passthrough();
        }

        // Handle clear on word boundary
        if key == keys::SPACE {
            let result = self.commit_buffer();
            self.buffer.clear();
            return result;
        }

        // Handle backspace
        if key == keys::BACKSPACE {
            if self.buffer.is_empty() {
                return ProcessResult::passthrough();
            }
            self.buffer.pop();
            return ProcessResult::commit(self.buffer.to_string(), 1);
        }

        // Handle ESC (clear buffer)
        if key == keys::ESC {
            let raw = self.buffer.raw_string();
            self.buffer.clear();
            return ProcessResult::commit(raw, 0);
        }

        // Process based on input method
        self.process_input(key, uppercase)
    }

    /// Process input based on method
    fn process_input(&mut self, key: u16, uppercase: bool) -> ProcessResult {
        match self.method {
            InputMethod::Telex => self.process_telex(key, uppercase),
            InputMethod::Vni => self.process_vni(key, uppercase),
        }
    }

    /// Process Telex input
    fn process_telex(&mut self, key: u16, uppercase: bool) -> ProcessResult {
        let raw_char = keycodes::to_char(key, uppercase).unwrap_or('?');
        let old_output = self.buffer.to_string();

        // Check for Telex modifiers
        match key {
            // Marks: s=acute, f=grave, r=hook, x=tilde, j=dot
            keys::S => {
                if apply_mark(&mut self.buffer, mark::ACUTE, self.use_modern_style).applied {
                    return self.build_result(&old_output);
                }
            }
            keys::F => {
                if apply_mark(&mut self.buffer, mark::GRAVE, self.use_modern_style).applied {
                    return self.build_result(&old_output);
                }
            }
            keys::R => {
                if apply_mark(&mut self.buffer, mark::HOOK, self.use_modern_style).applied {
                    return self.build_result(&old_output);
                }
            }
            keys::X => {
                if apply_mark(&mut self.buffer, mark::TILDE, self.use_modern_style).applied {
                    return self.build_result(&old_output);
                }
            }
            keys::J => {
                if apply_mark(&mut self.buffer, mark::DOT, self.use_modern_style).applied {
                    return self.build_result(&old_output);
                }
            }

            // Stroke: d → đ (when buffer has d)
            keys::D => {
                // Check if last char is also 'd' (dd → đ)
                if let Some(unit) = self.buffer.last() {
                    if unit.key == keys::D && !unit.is_stroked {
                        if apply_stroke(&mut self.buffer).applied {
                            return self.build_result(&old_output);
                        }
                    }
                }
            }

            // Horn modifier: w → ơ/ư
            keys::W => {
                if apply_tone_modifier(&mut self.buffer, tone::HORN, keys::W).applied {
                    return self.build_result(&old_output);
                }
            }

            // Circumflex: aa→â, ee→ê, oo→ô
            keys::A | keys::E | keys::O => {
                if let Some(unit) = self.buffer.last() {
                    if unit.key == key && unit.tone_mod == tone::NONE {
                        if apply_tone_modifier(&mut self.buffer, tone::CIRCUMFLEX, key).applied {
                            return self.build_result(&old_output);
                        }
                    }
                }
            }

            _ => {}
        }

        // Not a modifier - add as regular character
        self.buffer.push(InputUnit::new(key, uppercase), raw_char);
        self.build_result(&old_output)
    }

    /// Process VNI input
    fn process_vni(&mut self, key: u16, uppercase: bool) -> ProcessResult {
        let raw_char = keycodes::to_char(key, uppercase).unwrap_or('?');
        let old_output = self.buffer.to_string();

        // VNI uses numbers for modifiers
        match key {
            keys::N1 => {
                if apply_mark(&mut self.buffer, mark::ACUTE, self.use_modern_style).applied {
                    return self.build_result(&old_output);
                }
            }
            keys::N2 => {
                if apply_mark(&mut self.buffer, mark::GRAVE, self.use_modern_style).applied {
                    return self.build_result(&old_output);
                }
            }
            keys::N3 => {
                if apply_mark(&mut self.buffer, mark::HOOK, self.use_modern_style).applied {
                    return self.build_result(&old_output);
                }
            }
            keys::N4 => {
                if apply_mark(&mut self.buffer, mark::TILDE, self.use_modern_style).applied {
                    return self.build_result(&old_output);
                }
            }
            keys::N5 => {
                if apply_mark(&mut self.buffer, mark::DOT, self.use_modern_style).applied {
                    return self.build_result(&old_output);
                }
            }
            keys::N6 => {
                // Circumflex
                if apply_tone_modifier(&mut self.buffer, tone::CIRCUMFLEX, key).applied {
                    return self.build_result(&old_output);
                }
            }
            keys::N7 => {
                // Horn
                if apply_tone_modifier(&mut self.buffer, tone::HORN, key).applied {
                    return self.build_result(&old_output);
                }
            }
            keys::N8 => {
                // Breve (for 'a' only)
                if apply_tone_modifier(&mut self.buffer, tone::HORN, key).applied {
                    return self.build_result(&old_output);
                }
            }
            keys::N9 => {
                // Stroke
                if apply_stroke(&mut self.buffer).applied {
                    return self.build_result(&old_output);
                }
            }
            _ => {}
        }

        // Not a modifier - add as regular character
        self.buffer.push(InputUnit::new(key, uppercase), raw_char);
        self.build_result(&old_output)
    }

    /// Build result by comparing old and new output
    fn build_result(&self, old_output: &str) -> ProcessResult {
        let new_output = self.buffer.to_string();

        // Calculate common prefix to minimize backspaces
        let common_len = old_output
            .chars()
            .zip(new_output.chars())
            .take_while(|(a, b)| a == b)
            .count();

        let backspace = old_output.chars().count() - common_len;
        let output: String = new_output.chars().skip(common_len).collect();

        ProcessResult::commit(output, backspace)
    }

    /// Commit buffer content
    fn commit_buffer(&self) -> ProcessResult {
        if self.buffer.is_empty() {
            ProcessResult::passthrough()
        } else {
            ProcessResult::commit(self.buffer.to_string(), 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_telex() {
        let mut processor = Processor::new();
        processor.set_method(InputMethod::Telex);

        // Type "a"
        let result = processor.process_key(keys::A, false, false);
        assert_eq!(processor.get_buffer(), "a");

        // Type "s" (should add acute)
        let result = processor.process_key(keys::S, false, false);
        assert_eq!(processor.get_buffer(), "á");
    }

    #[test]
    fn test_circumflex() {
        let mut processor = Processor::new();
        processor.set_method(InputMethod::Telex);

        processor.process_key(keys::A, false, false);
        processor.process_key(keys::A, false, false);
        assert_eq!(processor.get_buffer(), "â");
    }

    #[test]
    fn test_stroke() {
        let mut processor = Processor::new();
        processor.set_method(InputMethod::Telex);

        processor.process_key(keys::D, false, false);
        processor.process_key(keys::D, false, false);
        assert_eq!(processor.get_buffer(), "đ");
    }
}
