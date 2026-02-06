//! Text buffer implementation using ropey
//!
//! The buffer is the core data structure for storing and manipulating text.
//! It uses a rope data structure for efficient operations on large files.

use ropey::Rope;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BufferError {
    #[error("Failed to read file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Position out of bounds: line {line}, column {column}")]
    OutOfBounds { line: usize, column: usize },
}

/// A text buffer backed by a rope data structure
#[derive(Debug)]
pub struct Buffer {
    /// The rope containing the text
    rope: Rope,

    /// Path to the file (if any)
    path: Option<PathBuf>,

    /// Whether the buffer has been modified since last save
    modified: bool,
}

impl Buffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            path: None,
            modified: false,
        }
    }

    /// Create a buffer with initial content
    pub fn with_content(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            path: None,
            modified: false,
        }
    }

    /// Load a buffer from a file
    pub fn from_file(path: impl Into<PathBuf>) -> Result<Self, BufferError> {
        let path = path.into();
        let text = std::fs::read_to_string(&path)?;
        Ok(Self {
            rope: Rope::from_str(&text),
            path: Some(path),
            modified: false,
        })
    }

    /// Save the buffer to its file path
    pub fn save(&mut self) -> Result<(), BufferError> {
        if let Some(path) = &self.path {
            std::fs::write(path, self.rope.to_string())?;
            self.modified = false;
        }
        Ok(())
    }

    /// Save the buffer to a new path
    pub fn save_as(&mut self, path: impl Into<PathBuf>) -> Result<(), BufferError> {
        let path = path.into();
        std::fs::write(&path, self.rope.to_string())?;
        self.path = Some(path);
        self.modified = false;
        Ok(())
    }

    /// Get the total number of lines
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Get the total number of characters
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Get a specific line (0-indexed)
    pub fn line(&self, line_idx: usize) -> Option<ropey::RopeSlice<'_>> {
        if line_idx < self.rope.len_lines() {
            Some(self.rope.line(line_idx))
        } else {
            None
        }
    }

    /// Get the length of a specific line (excluding newline)
    pub fn line_len(&self, line_idx: usize) -> Option<usize> {
        self.line(line_idx).map(|line| {
            let len = line.len_chars();
            // Subtract 1 for newline if present (except for last line)
            if len > 0 && line.char(len - 1) == '\n' {
                len - 1
            } else {
                len
            }
        })
    }

    /// Insert text at a character index
    pub fn insert(&mut self, char_idx: usize, text: &str) {
        self.rope.insert(char_idx, text);
        self.modified = true;
    }

    /// Insert a single character at a character index
    pub fn insert_char(&mut self, char_idx: usize, ch: char) {
        self.rope.insert_char(char_idx, ch);
        self.modified = true;
    }

    /// Remove a range of characters
    pub fn remove(&mut self, start: usize, end: usize) {
        self.rope.remove(start..end);
        self.modified = true;
    }

    /// Convert line/column to character index
    pub fn line_col_to_char(&self, line: usize, col: usize) -> Option<usize> {
        if line >= self.rope.len_lines() {
            return None;
        }
        let line_start = self.rope.line_to_char(line);
        let line_len = self.line_len(line).unwrap_or(0);
        let col = col.min(line_len);
        Some(line_start + col)
    }

    /// Convert character index to line/column
    pub fn char_to_line_col(&self, char_idx: usize) -> (usize, usize) {
        let line = self.rope.char_to_line(char_idx);
        let line_start = self.rope.line_to_char(line);
        let col = char_idx - line_start;
        (line, col)
    }

    /// Get the file path
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// Check if the buffer has been modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Get a reference to the underlying rope
    pub fn rope(&self) -> &Rope {
        &self.rope
    }

    /// Get the entire text as a string
    pub fn text(&self) -> String {
        self.rope.to_string()
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
    fn test_new_buffer() {
        let buffer = Buffer::new();
        assert!(buffer.is_empty());
        assert_eq!(buffer.len_lines(), 1); // Empty buffer has 1 line
    }

    #[test]
    fn test_from_str() {
        let buffer = Buffer::with_content("Hello\nWorld");
        assert_eq!(buffer.len_lines(), 2);
        assert_eq!(buffer.line(0).unwrap().to_string(), "Hello\n");
        assert_eq!(buffer.line(1).unwrap().to_string(), "World");
    }

    #[test]
    fn test_insert() {
        let mut buffer = Buffer::with_content("Hello World");
        buffer.insert(5, ",");
        assert_eq!(buffer.text(), "Hello, World");
        assert!(buffer.is_modified());
    }

    #[test]
    fn test_remove() {
        let mut buffer = Buffer::with_content("Hello, World");
        buffer.remove(5, 7);
        assert_eq!(buffer.text(), "HelloWorld");
    }

    #[test]
    fn test_line_col_conversion() {
        let buffer = Buffer::with_content("Hello\nWorld\nTest");

        assert_eq!(buffer.line_col_to_char(0, 0), Some(0));
        assert_eq!(buffer.line_col_to_char(1, 0), Some(6));
        assert_eq!(buffer.line_col_to_char(2, 2), Some(14));

        assert_eq!(buffer.char_to_line_col(0), (0, 0));
        assert_eq!(buffer.char_to_line_col(6), (1, 0));
        assert_eq!(buffer.char_to_line_col(14), (2, 2));
    }
}
