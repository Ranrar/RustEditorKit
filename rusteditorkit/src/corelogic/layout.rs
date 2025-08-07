//! Layout and A4 page functionality for EditorBuffer
//!
//! This module contains A4 page layout, margins, tab width, line wrapping, and spacing calculations.

use super::buffer::EditorBuffer;

#[derive(Debug, Clone, Copy)]
pub struct EditorMarginUpdate {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

/// Convert centimeters to pixels (assuming 96 DPI)
pub fn cm_to_px(cm: f64) -> f64 {
    cm * 96.0 / 2.54
}

/// Convert pixels to centimeters (assuming 96 DPI)
pub fn px_to_cm(px: f64) -> f64 {
    px * 2.54 / 96.0
}

impl EditorBuffer {
    /// Returns a Vec of y-offsets for each line, including paragraph spacing
    ///
    /// # Arguments
    /// * `line_height` - Height of each line
    /// * `paragraph_spacing` - Extra space after paragraphs
    /// * `top_margin` - Top margin for the editor
    pub fn line_y_offsets(&self, line_height: f64, paragraph_spacing: f64, top_margin: f64) -> Vec<f64> {
        let mut y_offset = top_margin;
        let mut offsets = Vec::with_capacity(self.lines.len());
        for _line in self.lines.iter() {
            offsets.push(y_offset);
            y_offset += line_height;
            // Add paragraph spacing after every line (including empty lines)
            y_offset += paragraph_spacing;
        }
        offsets
    }
}
