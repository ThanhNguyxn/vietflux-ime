//! Input Buffer Management
//!
//! Zero-copy buffer for efficient keystroke processing.
//! Stores raw input and transformed Vietnamese text.

/// Maximum buffer size (one Vietnamese word rarely exceeds 10 chars)
pub const MAX_BUFFER_SIZE: usize = 32;

/// A single character in the buffer with metadata
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BufferChar {
    /// The actual character
    pub ch: char,
    /// Original input key that produced this char
    pub raw: char,
    /// Whether this char has been transformed
    pub transformed: bool,
}

impl BufferChar {
    pub fn new(ch: char, raw: char) -> Self {
        Self {
            ch,
            raw,
            transformed: ch != raw,
        }
    }
    
    pub fn simple(ch: char) -> Self {
        Self::new(ch, ch)
    }
}

/// Input buffer for IME processing
#[derive(Debug, Clone)]
pub struct Buffer {
    /// Characters in buffer
    chars: Vec<BufferChar>,
    /// Current cursor position (reserved for future use)
    #[allow(dead_code)]
    cursor: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            chars: Vec::with_capacity(MAX_BUFFER_SIZE),
            cursor: 0,
        }
    }
    
    /// Push a character to buffer
    pub fn push(&mut self, ch: char, raw: char) {
        if self.chars.len() < MAX_BUFFER_SIZE {
            self.chars.push(BufferChar::new(ch, raw));
            self.cursor = self.chars.len();
        }
    }
    
    /// Push a simple character (raw == transformed)
    pub fn push_simple(&mut self, ch: char) {
        self.push(ch, ch);
    }
    
    /// Pop the last character
    pub fn pop(&mut self) -> Option<BufferChar> {
        let result = self.chars.pop();
        self.cursor = self.chars.len();
        result
    }
    
    /// Clear the buffer
    pub fn clear(&mut self) {
        self.chars.clear();
        self.cursor = 0;
    }
    
    /// Get buffer length
    pub fn len(&self) -> usize {
        self.chars.len()
    }
    
    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }
    
    /// Get character at index
    pub fn get(&self, index: usize) -> Option<&BufferChar> {
        self.chars.get(index)
    }
    
    /// Get mutable character at index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut BufferChar> {
        self.chars.get_mut(index)
    }
    
    /// Get last character
    pub fn last(&self) -> Option<&BufferChar> {
        self.chars.last()
    }
    
    /// Get last mutable character
    pub fn last_mut(&mut self) -> Option<&mut BufferChar> {
        self.chars.last_mut()
    }
    
    /// Replace character at index
    pub fn replace(&mut self, index: usize, ch: char) -> bool {
        if let Some(bc) = self.chars.get_mut(index) {
            bc.ch = ch;
            bc.transformed = true;
            true
        } else {
            false
        }
    }
    
    /// Get the transformed text
    pub fn get_text(&self) -> String {
        self.chars.iter().map(|bc| bc.ch).collect()
    }
    
    /// Get the raw input
    pub fn get_raw(&self) -> String {
        self.chars.iter().map(|bc| bc.raw).collect()
    }
    
    /// Find last vowel index in buffer
    pub fn find_last_vowel(&self) -> Option<usize> {
        for i in (0..self.chars.len()).rev() {
            if crate::chars::is_vowel(self.chars[i].ch) {
                return Some(i);
            }
        }
        None
    }
    
    /// Find all vowel indices
    pub fn find_vowels(&self) -> Vec<usize> {
        self.chars
            .iter()
            .enumerate()
            .filter(|(_, bc)| crate::chars::is_vowel(bc.ch))
            .map(|(i, _)| i)
            .collect()
    }
    
    /// Get characters as slice
    pub fn as_slice(&self) -> &[BufferChar] {
        &self.chars
    }
    
    /// Iterator over characters
    pub fn iter(&self) -> impl Iterator<Item = &BufferChar> {
        self.chars.iter()
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_basic() {
        let mut buf = Buffer::new();
        buf.push_simple('v');
        buf.push_simple('i');
        buf.push_simple('e');
        buf.push_simple('t');
        
        assert_eq!(buf.len(), 4);
        assert_eq!(buf.get_text(), "viet");
    }

    #[test]
    fn test_buffer_transform() {
        let mut buf = Buffer::new();
        buf.push_simple('v');
        buf.push_simple('i');
        buf.push('ệ', 'e');
        buf.push_simple('t');
        
        assert_eq!(buf.get_text(), "việt");
        assert_eq!(buf.get_raw(), "viet");
    }

    #[test]
    fn test_find_vowels() {
        let mut buf = Buffer::new();
        buf.push_simple('v');
        buf.push_simple('i');
        buf.push_simple('e');
        buf.push_simple('t');
        
        let vowels = buf.find_vowels();
        assert_eq!(vowels, vec![1, 2]); // 'i' and 'e'
    }
}
