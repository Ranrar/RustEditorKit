// Selection logic for EditorBuffer
use crate::core::EditorBuffer;

impl EditorBuffer {
    /// Select all text in the buffer
    pub fn select_all(&mut self) {
        if !self.lines.is_empty() {
            self.selection_start = Some((0, 0));
            let last_row = self.lines.len() - 1;
            let last_col = self.lines[last_row].len();
            self.selection_end = Some((last_row, last_col));
        }
    }
}
