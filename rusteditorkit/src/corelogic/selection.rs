//! Selection model for RustEditorKit
//! Supports multi-line, robust text selection.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
}

impl Selection {
    /// Clamp the selection to valid buffer bounds
    pub fn clamp_to_buffer(&mut self, lines: &Vec<String>) {
        let last_row = lines.len().saturating_sub(1);
        self.start_row = self.start_row.min(last_row);
        self.end_row = self.end_row.min(last_row);
        self.start_col = self.start_col.min(lines.get(self.start_row).map(|l| l.len()).unwrap_or(0));
        self.end_col = self.end_col.min(lines.get(self.end_row).map(|l| l.len()).unwrap_or(0));
        // If buffer is empty, reset to 0,0
        if lines.is_empty() {
            self.start_row = 0;
            self.start_col = 0;
            self.end_row = 0;
            self.end_col = 0;
        }
    }
    /// Selects the entire buffer (from 0,0 to last line/col)
    pub fn select_all(&mut self, last_row: usize, last_col: usize) {
        self.start_row = 0;
        self.start_col = 0;
        self.end_row = last_row;
        self.end_col = last_col;
    }
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            start_row: row,
            start_col: col,
            end_row: row,
            end_col: col,
        }
    }
    pub fn set(&mut self, start_row: usize, start_col: usize, end_row: usize, end_col: usize) {
        self.start_row = start_row;
        self.start_col = start_col;
        self.end_row = end_row;
        self.end_col = end_col;
    }
    pub fn clear(&mut self) {
        self.start_row = self.end_row;
        self.start_col = self.end_col;
    }
    pub fn is_active(&self) -> bool {
        self.start_row != self.end_row || self.start_col != self.end_col
    }
    pub fn normalized(&self) -> ((usize, usize), (usize, usize)) {
        if (self.start_row, self.start_col) <= (self.end_row, self.end_col) {
            ((self.start_row, self.start_col), (self.end_row, self.end_col))
        } else {
            ((self.end_row, self.end_col), (self.start_row, self.start_col))
        }
    }
}
