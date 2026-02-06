//! Cursor and position handling
//!
//! Provides cursor movement and position tracking for the editor.

/// A position in the buffer (line, column)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    /// Line number (0-indexed)
    pub line: usize,
    /// Column number (0-indexed, in characters not bytes)
    pub column: usize,
}

impl Position {
    /// Create a new position
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    /// Position at the start of the document
    pub fn zero() -> Self {
        Self { line: 0, column: 0 }
    }
}

/// A cursor in the buffer with optional selection
#[derive(Debug, Clone)]
pub struct Cursor {
    /// Current cursor position
    pub position: Position,

    /// Selection anchor (if any)
    /// When set, text between anchor and position is selected
    pub anchor: Option<Position>,

    /// Preferred column for vertical movement
    /// Remembers the column when moving through shorter lines
    preferred_column: Option<usize>,
}

impl Cursor {
    /// Create a new cursor at position (0, 0)
    pub fn new() -> Self {
        Self {
            position: Position::zero(),
            anchor: None,
            preferred_column: None,
        }
    }

    /// Create a cursor at a specific position
    pub fn at(line: usize, column: usize) -> Self {
        Self {
            position: Position::new(line, column),
            anchor: None,
            preferred_column: None,
        }
    }

    /// Move the cursor to a new position
    pub fn move_to(&mut self, line: usize, column: usize) {
        self.position = Position::new(line, column);
        self.preferred_column = None;
    }

    /// Move cursor up by n lines
    pub fn move_up(&mut self, n: usize, line_lengths: impl Fn(usize) -> usize) {
        if self.position.line == 0 {
            return;
        }

        // Remember preferred column
        if self.preferred_column.is_none() {
            self.preferred_column = Some(self.position.column);
        }

        let new_line = self.position.line.saturating_sub(n);
        let line_len = line_lengths(new_line);
        let preferred = self.preferred_column.unwrap_or(self.position.column);

        self.position.line = new_line;
        self.position.column = preferred.min(line_len);
    }

    /// Move cursor down by n lines
    pub fn move_down(
        &mut self,
        n: usize,
        total_lines: usize,
        line_lengths: impl Fn(usize) -> usize,
    ) {
        if self.position.line >= total_lines.saturating_sub(1) {
            return;
        }

        // Remember preferred column
        if self.preferred_column.is_none() {
            self.preferred_column = Some(self.position.column);
        }

        let new_line = (self.position.line + n).min(total_lines.saturating_sub(1));
        let line_len = line_lengths(new_line);
        let preferred = self.preferred_column.unwrap_or(self.position.column);

        self.position.line = new_line;
        self.position.column = preferred.min(line_len);
    }

    /// Move cursor left by n characters
    pub fn move_left(&mut self, n: usize, line_lengths: impl Fn(usize) -> usize) {
        self.preferred_column = None;

        for _ in 0..n {
            if self.position.column > 0 {
                self.position.column -= 1;
            } else if self.position.line > 0 {
                // Wrap to end of previous line
                self.position.line -= 1;
                self.position.column = line_lengths(self.position.line);
            }
        }
    }

    /// Move cursor right by n characters
    pub fn move_right(
        &mut self,
        n: usize,
        total_lines: usize,
        line_lengths: impl Fn(usize) -> usize,
    ) {
        self.preferred_column = None;

        for _ in 0..n {
            let line_len = line_lengths(self.position.line);
            if self.position.column < line_len {
                self.position.column += 1;
            } else if self.position.line < total_lines.saturating_sub(1) {
                // Wrap to start of next line
                self.position.line += 1;
                self.position.column = 0;
            }
        }
    }

    /// Start a selection at the current position
    pub fn start_selection(&mut self) {
        self.anchor = Some(self.position);
    }

    /// Clear the selection
    pub fn clear_selection(&mut self) {
        self.anchor = None;
    }

    /// Check if there is an active selection
    pub fn has_selection(&self) -> bool {
        self.anchor.is_some()
    }

    /// Get the selection range (start, end) sorted
    pub fn selection_range(&self) -> Option<(Position, Position)> {
        self.anchor.map(|anchor| {
            if anchor.line < self.position.line
                || (anchor.line == self.position.line && anchor.column <= self.position.column)
            {
                (anchor, self.position)
            } else {
                (self.position, anchor)
            }
        })
    }

    /// Move to the start of the current line
    pub fn move_to_line_start(&mut self) {
        self.position.column = 0;
        self.preferred_column = None;
    }

    /// Move to the end of the current line
    pub fn move_to_line_end(&mut self, line_length: usize) {
        self.position.column = line_length;
        self.preferred_column = None;
    }

    /// Move to the start of the document
    pub fn move_to_start(&mut self) {
        self.position = Position::zero();
        self.preferred_column = None;
    }

    /// Move to the end of the document
    pub fn move_to_end(&mut self, total_lines: usize, last_line_length: usize) {
        self.position.line = total_lines.saturating_sub(1);
        self.position.column = last_line_length;
        self.preferred_column = None;
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line_lengths(line: usize) -> usize {
        // Simulate lines: "Hello" (5), "World" (5), "!" (1)
        match line {
            0 => 5,
            1 => 5,
            2 => 1,
            _ => 0,
        }
    }

    #[test]
    fn test_cursor_movement() {
        let mut cursor = Cursor::new();

        // Move right
        cursor.move_right(3, 3, line_lengths);
        assert_eq!(cursor.position, Position::new(0, 3));

        // Move down
        cursor.move_down(1, 3, line_lengths);
        assert_eq!(cursor.position, Position::new(1, 3));

        // Move left
        cursor.move_left(2, line_lengths);
        assert_eq!(cursor.position, Position::new(1, 1));

        // Move up
        cursor.move_up(1, line_lengths);
        assert_eq!(cursor.position, Position::new(0, 1));
    }

    #[test]
    fn test_line_wrapping() {
        let mut cursor = Cursor::at(0, 5);

        // Move right should wrap to next line
        cursor.move_right(1, 3, line_lengths);
        assert_eq!(cursor.position, Position::new(1, 0));

        // Move left should wrap to previous line
        cursor.move_left(1, line_lengths);
        assert_eq!(cursor.position, Position::new(0, 5));
    }

    #[test]
    fn test_preferred_column() {
        let mut cursor = Cursor::at(0, 5);

        // Move down to shorter line - should clamp
        cursor.move_down(2, 3, line_lengths);
        assert_eq!(cursor.position, Position::new(2, 1)); // "!" has length 1

        // Move up - should restore preferred column
        cursor.move_up(2, line_lengths);
        assert_eq!(cursor.position, Position::new(0, 5));
    }

    #[test]
    fn test_selection() {
        let mut cursor = Cursor::at(1, 2);

        cursor.start_selection();
        // Move right 5 times: (1,2) → (1,3) → (1,4) → (1,5) → (2,0) → (2,1)
        cursor.move_right(5, 3, line_lengths);

        let (start, end) = cursor.selection_range().unwrap();
        assert_eq!(start, Position::new(1, 2));
        assert_eq!(end, Position::new(2, 1)); // "!" has length 1, so we end at column 1

        cursor.clear_selection();
        assert!(!cursor.has_selection());
    }
}
