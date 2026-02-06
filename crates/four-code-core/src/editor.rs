//! Editor state combining buffer and cursor
//!
//! The Editor struct manages the text buffer, cursor, and viewport.

use crate::{Buffer, Cursor};
use std::path::PathBuf;

/// Viewport for scrolling
#[derive(Debug, Clone, Default)]
pub struct Viewport {
    /// First visible line
    pub top_line: usize,
    /// Number of visible lines
    pub height: usize,
    /// Number of visible columns
    pub width: usize,
}

impl Viewport {
    /// Create a new viewport
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            top_line: 0,
            height,
            width,
        }
    }

    /// Ensure a line is visible, scrolling if necessary
    pub fn ensure_visible(&mut self, line: usize) {
        if self.height == 0 {
            return;
        }
        if line < self.top_line {
            self.top_line = line;
        } else if line >= self.top_line + self.height {
            self.top_line = line.saturating_sub(self.height.saturating_sub(1));
        }
    }
}

/// Editor state
#[derive(Debug)]
pub struct Editor {
    /// Text buffer
    pub buffer: Buffer,

    /// Cursor position
    pub cursor: Cursor,

    /// Viewport for scrolling
    pub viewport: Viewport,
}

impl Editor {
    /// Create a new empty editor
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            cursor: Cursor::new(),
            viewport: Viewport::default(),
        }
    }

    /// Create editor with content
    pub fn with_content(text: &str) -> Self {
        Self {
            buffer: Buffer::with_content(text),
            cursor: Cursor::new(),
            viewport: Viewport::default(),
        }
    }

    /// Open a file
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, crate::BufferError> {
        Ok(Self {
            buffer: Buffer::from_file(path)?,
            cursor: Cursor::new(),
            viewport: Viewport::default(),
        })
    }

    /// Set viewport size
    pub fn set_viewport_size(&mut self, height: usize, width: usize) {
        self.viewport.height = height;
        self.viewport.width = width;
    }

    /// Get current line length
    fn current_line_len(&self) -> usize {
        self.buffer.line_len(self.cursor.position.line).unwrap_or(0)
    }

    /// Get line length for a specific line
    fn line_len(&self, line: usize) -> usize {
        self.buffer.line_len(line).unwrap_or(0)
    }

    // === Cursor Movement ===

    /// Move cursor up
    pub fn move_up(&mut self) {
        self.cursor
            .move_up(1, |line| self.buffer.line_len(line).unwrap_or(0));
        self.viewport.ensure_visible(self.cursor.position.line);
    }

    /// Move cursor down
    pub fn move_down(&mut self) {
        self.cursor.move_down(1, self.buffer.len_lines(), |line| {
            self.buffer.line_len(line).unwrap_or(0)
        });
        self.viewport.ensure_visible(self.cursor.position.line);
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        self.cursor
            .move_left(1, |line| self.buffer.line_len(line).unwrap_or(0));
        self.viewport.ensure_visible(self.cursor.position.line);
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        self.cursor.move_right(1, self.buffer.len_lines(), |line| {
            self.buffer.line_len(line).unwrap_or(0)
        });
        self.viewport.ensure_visible(self.cursor.position.line);
    }

    /// Move to start of line
    pub fn move_to_line_start(&mut self) {
        self.cursor.move_to_line_start();
    }

    /// Move to end of line
    pub fn move_to_line_end(&mut self) {
        let line_len = self.current_line_len();
        self.cursor.move_to_line_end(line_len);
    }

    /// Move to start of document
    pub fn move_to_start(&mut self) {
        self.cursor.move_to_start();
        self.viewport.top_line = 0;
    }

    /// Move to end of document
    pub fn move_to_end(&mut self) {
        let total_lines = self.buffer.len_lines();
        let last_line_len = self.line_len(total_lines.saturating_sub(1));
        self.cursor.move_to_end(total_lines, last_line_len);
        self.viewport.ensure_visible(self.cursor.position.line);
    }

    /// Page up
    pub fn page_up(&mut self) {
        let lines = self.viewport.height.saturating_sub(2).max(1);
        for _ in 0..lines {
            self.move_up();
        }
    }

    /// Page down
    pub fn page_down(&mut self) {
        let lines = self.viewport.height.saturating_sub(2).max(1);
        for _ in 0..lines {
            self.move_down();
        }
    }

    // === Text Editing ===

    /// Insert a character at cursor position
    pub fn insert_char(&mut self, ch: char) {
        if let Some(char_idx) = self
            .buffer
            .line_col_to_char(self.cursor.position.line, self.cursor.position.column)
        {
            self.buffer.insert_char(char_idx, ch);

            if ch == '\n' {
                // Move to start of new line
                self.cursor.position.line += 1;
                self.cursor.position.column = 0;
            } else {
                self.cursor.position.column += 1;
            }
            self.viewport.ensure_visible(self.cursor.position.line);
        }
    }

    /// Insert a string at cursor position
    pub fn insert_str(&mut self, text: &str) {
        for ch in text.chars() {
            self.insert_char(ch);
        }
    }

    /// Delete character before cursor (backspace)
    pub fn backspace(&mut self) {
        if self.cursor.position.column > 0 {
            // Delete character before cursor on same line
            if let Some(char_idx) = self
                .buffer
                .line_col_to_char(self.cursor.position.line, self.cursor.position.column - 1)
            {
                self.buffer.remove(char_idx, char_idx + 1);
                self.cursor.position.column -= 1;
            }
        } else if self.cursor.position.line > 0 {
            // At start of line - join with previous line
            let prev_line_len = self.line_len(self.cursor.position.line - 1);
            if let Some(char_idx) = self.buffer.line_col_to_char(self.cursor.position.line, 0) {
                // Remove the newline character at end of previous line
                self.buffer.remove(char_idx - 1, char_idx);
                self.cursor.position.line -= 1;
                self.cursor.position.column = prev_line_len;
            }
            self.viewport.ensure_visible(self.cursor.position.line);
        }
    }

    /// Delete character at cursor (delete key)
    pub fn delete(&mut self) {
        let line_len = self.current_line_len();

        if self.cursor.position.column < line_len {
            // Delete character at cursor
            if let Some(char_idx) = self
                .buffer
                .line_col_to_char(self.cursor.position.line, self.cursor.position.column)
            {
                self.buffer.remove(char_idx, char_idx + 1);
            }
        } else if self.cursor.position.line < self.buffer.len_lines() - 1 {
            // At end of line - join with next line (delete newline)
            if let Some(char_idx) = self
                .buffer
                .line_col_to_char(self.cursor.position.line, self.cursor.position.column)
            {
                self.buffer.remove(char_idx, char_idx + 1);
            }
        }
    }

    /// Insert a new line (Enter key)
    pub fn insert_newline(&mut self) {
        self.insert_char('\n');
    }

    // === Selection ===

    /// Start or extend selection
    pub fn start_selection(&mut self) {
        if !self.cursor.has_selection() {
            self.cursor.start_selection();
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.cursor.clear_selection();
    }

    /// Move with selection (Shift+Arrow)
    pub fn move_up_select(&mut self) {
        self.start_selection();
        self.move_up();
    }

    pub fn move_down_select(&mut self) {
        self.start_selection();
        self.move_down();
    }

    pub fn move_left_select(&mut self) {
        self.start_selection();
        self.move_left();
    }

    pub fn move_right_select(&mut self) {
        self.start_selection();
        self.move_right();
    }

    pub fn move_to_line_start_select(&mut self) {
        self.start_selection();
        self.move_to_line_start();
    }

    pub fn move_to_line_end_select(&mut self) {
        self.start_selection();
        self.move_to_line_end();
    }

    pub fn move_to_start_select(&mut self) {
        self.start_selection();
        self.move_to_start();
    }

    pub fn move_to_end_select(&mut self) {
        self.start_selection();
        self.move_to_end();
    }

    /// Select all text
    pub fn select_all(&mut self) {
        self.move_to_start();
        self.cursor.start_selection();
        self.move_to_end();
    }

    /// Get selected text
    pub fn get_selected_text(&self) -> Option<String> {
        let (start, end) = self.cursor.selection_range()?;

        let start_idx = self.buffer.line_col_to_char(start.line, start.column)?;
        let end_idx = self.buffer.line_col_to_char(end.line, end.column)?;

        Some(self.buffer.rope().slice(start_idx..end_idx).to_string())
    }

    /// Delete selected text
    pub fn delete_selection(&mut self) -> bool {
        if let Some((start, end)) = self.cursor.selection_range() {
            if let (Some(start_idx), Some(end_idx)) = (
                self.buffer.line_col_to_char(start.line, start.column),
                self.buffer.line_col_to_char(end.line, end.column),
            ) {
                self.buffer.remove(start_idx, end_idx);
                self.cursor.position = start;
                self.cursor.clear_selection();
                self.viewport.ensure_visible(self.cursor.position.line);
                return true;
            }
        }
        false
    }

    /// Replace selection with text (or just insert if no selection)
    pub fn replace_selection(&mut self, text: &str) {
        self.delete_selection();
        self.insert_str(text);
    }

    // === File Operations ===

    /// Save the file
    pub fn save(&mut self) -> Result<(), crate::BufferError> {
        self.buffer.save()
    }

    /// Check if modified
    pub fn is_modified(&self) -> bool {
        self.buffer.is_modified()
    }

    /// Get file path
    pub fn path(&self) -> Option<&PathBuf> {
        self.buffer.path()
    }

    /// Get filename for display
    pub fn filename(&self) -> String {
        self.buffer
            .path()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "[untitled]".to_string())
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Position;

    #[test]
    fn test_insert_char() {
        let mut editor = Editor::new();
        editor.insert_char('H');
        editor.insert_char('i');

        assert_eq!(editor.buffer.text(), "Hi");
        assert_eq!(editor.cursor.position, Position::new(0, 2));
    }

    #[test]
    fn test_insert_newline() {
        let mut editor = Editor::new();
        editor.insert_str("Hello");
        editor.insert_newline();
        editor.insert_str("World");

        assert_eq!(editor.buffer.text(), "Hello\nWorld");
        assert_eq!(editor.cursor.position, Position::new(1, 5));
    }

    #[test]
    fn test_backspace() {
        let mut editor = Editor::with_content("Hello");
        editor.cursor.move_to(0, 5);

        editor.backspace();
        assert_eq!(editor.buffer.text(), "Hell");
        assert_eq!(editor.cursor.position.column, 4);
    }

    #[test]
    fn test_backspace_join_lines() {
        let mut editor = Editor::with_content("Hello\nWorld");
        editor.cursor.move_to(1, 0); // Start of "World"

        editor.backspace();
        assert_eq!(editor.buffer.text(), "HelloWorld");
        assert_eq!(editor.cursor.position, Position::new(0, 5));
    }

    #[test]
    fn test_delete() {
        let mut editor = Editor::with_content("Hello");
        editor.cursor.move_to(0, 0);

        editor.delete();
        assert_eq!(editor.buffer.text(), "ello");
    }

    #[test]
    fn test_delete_join_lines() {
        let mut editor = Editor::with_content("Hello\nWorld");
        editor.cursor.move_to(0, 5); // End of "Hello"

        editor.delete();
        assert_eq!(editor.buffer.text(), "HelloWorld");
    }

    #[test]
    fn test_viewport_scrolling() {
        let mut editor = Editor::with_content("Line1\nLine2\nLine3\nLine4\nLine5");
        editor.set_viewport_size(3, 80);

        // Move down past viewport
        editor.move_down();
        editor.move_down();
        editor.move_down();

        assert_eq!(editor.cursor.position.line, 3);
        assert!(editor.viewport.top_line > 0);
    }

    #[test]
    fn test_selection_get_text() {
        let mut editor = Editor::with_content("Hello World");
        editor.cursor.move_to(0, 0);
        editor.cursor.start_selection();
        editor.cursor.move_to(0, 5);

        let selected = editor.get_selected_text();
        assert_eq!(selected, Some("Hello".to_string()));
    }

    #[test]
    fn test_selection_multiline() {
        let mut editor = Editor::with_content("Hello\nWorld\nTest");
        editor.cursor.move_to(0, 3); // "Hel|lo"
        editor.cursor.start_selection();
        editor.cursor.move_to(1, 3); // "Wor|ld"

        let selected = editor.get_selected_text();
        assert_eq!(selected, Some("lo\nWor".to_string()));
    }

    #[test]
    fn test_delete_selection() {
        let mut editor = Editor::with_content("Hello World");
        editor.cursor.move_to(0, 0);
        editor.cursor.start_selection();
        editor.cursor.move_to(0, 6); // Select "Hello "

        let deleted = editor.delete_selection();
        assert!(deleted);
        assert_eq!(editor.buffer.text(), "World");
        assert_eq!(editor.cursor.position, Position::new(0, 0));
    }

    #[test]
    fn test_replace_selection() {
        let mut editor = Editor::with_content("Hello World");
        editor.cursor.move_to(0, 6);
        editor.cursor.start_selection();
        editor.cursor.move_to(0, 11); // Select "World"

        editor.replace_selection("Rust");
        assert_eq!(editor.buffer.text(), "Hello Rust");
    }

    #[test]
    fn test_select_all() {
        let mut editor = Editor::with_content("Hello\nWorld");
        editor.select_all();

        let selected = editor.get_selected_text();
        assert_eq!(selected, Some("Hello\nWorld".to_string()));
    }
}
