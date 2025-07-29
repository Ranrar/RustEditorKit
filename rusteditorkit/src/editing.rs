// Editing logic for EditorBuffer
use crate::core::EditorBuffer;

impl EditorBuffer {
    /// Delete character before cursor (backspace)
    pub fn backspace(&mut self) {
        if self.cursor.col > 0 {
            self.push_undo();
            let line = &mut self.lines[self.cursor.row];
            line.remove(self.cursor.col - 1);
            self.cursor.col -= 1;
        } else if self.cursor.row > 0 {
            self.push_undo();
            let prev_len = self.lines[self.cursor.row - 1].len();
            let current = self.lines.remove(self.cursor.row);
            self.cursor.row -= 1;
            self.cursor.col = prev_len;
            self.lines[self.cursor.row].push_str(&current);
        }
    }

    /// Delete character at cursor (delete)
    pub fn delete(&mut self) {
        if self.cursor.row < self.lines.len() {
            if self.cursor.col < self.lines[self.cursor.row].len() {
                self.push_undo();
                self.lines[self.cursor.row].remove(self.cursor.col);
            } else if self.cursor.row + 1 < self.lines.len() {
                self.push_undo();
                let next_line = self.lines.remove(self.cursor.row + 1);
                self.lines[self.cursor.row].push_str(&next_line);
            }
        }
    }

    /// Paste text at cursor
    pub fn paste(&mut self, text: &str) {
        self.push_undo();
        let row = self.cursor.row;
        let col = self.cursor.col;
        if let Some(line) = self.lines.get_mut(row) {
            line.insert_str(col, text);
            self.cursor.col += text.len();
        }
    }
}
