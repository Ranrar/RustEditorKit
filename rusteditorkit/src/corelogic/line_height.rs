//! Unified line height calculation utility for RustEditorKit
//!
//! This module provides a single, documented API for calculating line height
//! for all layout, rendering, and hit-testing logic. Always use this function
//! to ensure consistency across the editor.
//!
//! # Usage
//! Use `unified_line_height(text_height, gutter_height, paragraph_spacing)`
//! wherever line height is needed for layout, rendering, or mouse hit-testing.
//!
//! # Formula
//! line_height = max(text_height, gutter_height, glyph_height) + paragraph_spacing

/// Returns the unified line height for editor layout, rendering, and hit-testing.
///
/// # Arguments
/// * `text_height` - The pixel height of the text font (from Pango metrics)
/// * `gutter_height` - The pixel height of the gutter font (from Pango metrics)
/// * `glyph_height` - The pixel height of the tallest glyph in the line (from Pango metrics)
/// * `paragraph_spacing` - Extra spacing between lines (from config)
///
/// # Returns
/// * `f64` - The total line height in pixels
pub fn unified_line_height(text_height: f64, gutter_height: f64, glyph_height: f64, paragraph_spacing: f64) -> f64 {
    text_height.max(gutter_height).max(glyph_height) + paragraph_spacing
}
