
//! EditorCursor: Encapsulates cursor position and movement logic for RustEditorKit

/// Represents the position of the cursor in the editor (row, col).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditorCursor {
    /// The line index (0-based)
    pub row: usize,
    /// The column index (0-based, in chars)
    pub col: usize,
}

impl EditorCursor {
    /// Creates a new cursor at the given (row, col) position.
    ///
    /// # Example
    /// ```
    /// use rusteditorkit::cursor::EditorCursor;
    /// let cursor = EditorCursor::new(2, 5);
    /// assert_eq!(cursor.row, 2);
    /// assert_eq!(cursor.col, 5);
    /// ```
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Moves the cursor left by one character, or to the end of the previous line if at column 0.
    pub fn move_left(&mut self, lines: &[String]) {
        if self.col > 0 {
            self.col -= 1;
        } else if self.row > 0 {
            self.row -= 1;
            self.col = lines[self.row].len();
        }
    }

    /// Moves the cursor right by one character, or to the start of the next line if at end of line.
    pub fn move_right(&mut self, lines: &[String]) {
        if self.col < lines[self.row].len() {
            self.col += 1;
        } else if self.row + 1 < lines.len() {
            self.row += 1;
            self.col = 0;
        }
    }

    /// Moves the cursor up by one line, keeping column within line bounds.
    pub fn move_up(&mut self, lines: &[String]) {
        if self.row > 0 {
            self.row -= 1;
            self.col = self.col.min(lines[self.row].len());
        }
    }

    /// Moves the cursor down by one line, keeping column within line bounds.
    pub fn move_down(&mut self, lines: &[String]) {
        if self.row + 1 < lines.len() {
            self.row += 1;
            self.col = self.col.min(lines[self.row].len());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cursor_movement() {
        let lines = vec!["abc".to_string(), "de".to_string()];
        let mut cursor = EditorCursor::new(0, 0);
        // Move right within first line
        cursor.move_right(&lines);
        assert_eq!(cursor, EditorCursor::new(0, 1));
        // Move to end of first line, then to start of second
        cursor.move_right(&lines);
        cursor.move_right(&lines);
        cursor.move_right(&lines);
        assert_eq!(cursor, EditorCursor::new(1, 0));
        // Move left to end of previous line
        cursor.move_left(&lines);
        assert_eq!(cursor, EditorCursor::new(0, 3));
        // Move down, column clamps to line length
        cursor.move_down(&lines);
        assert_eq!(cursor, EditorCursor::new(1, 2));
        // Set cursor column to max before moving up
        cursor.col = lines[1].len();
        cursor.move_up(&lines);
        assert_eq!(cursor, EditorCursor::new(0, 2));
    }
}
