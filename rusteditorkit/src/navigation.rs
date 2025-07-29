// Navigation logic for EditorBuffer
use crate::core::EditorBuffer;

impl EditorBuffer {
    // Move cursor left (with bounds checking)
    pub fn move_left(&mut self) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        } else if self.cursor.row > 0 {
            self.cursor.row -= 1;
            self.cursor.col = self.lines[self.cursor.row].len();
        }
    }

    // Move cursor right (with bounds checking)
    pub fn move_right(&mut self) {
        if self.cursor.col < self.lines[self.cursor.row].len() {
            self.cursor.col += 1;
        } else if self.cursor.row + 1 < self.lines.len() {
            self.cursor.row += 1;
            self.cursor.col = 0;
        }
    }

    // Move cursor up (with bounds checking)
    pub fn move_up(&mut self) {
        if self.cursor.row > 0 {
            self.cursor.row -= 1;
            self.cursor.col = self.cursor.col.min(self.lines[self.cursor.row].len());
        }
    }

    // Move cursor down (with bounds checking)
    pub fn move_down(&mut self) {
        if self.cursor.row + 1 < self.lines.len() {
            self.cursor.row += 1;
            self.cursor.col = self.cursor.col.min(self.lines[self.cursor.row].len());
        }
    }

    // Move cursor to start of line
    pub fn move_home(&mut self) {
        self.cursor.col = 0;
    }

    // Move cursor to end of line
    pub fn move_end(&mut self) {
        self.cursor.col = self.lines[self.cursor.row].len();
    }
}
