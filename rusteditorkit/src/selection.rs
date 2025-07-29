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
