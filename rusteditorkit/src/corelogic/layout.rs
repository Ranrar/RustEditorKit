//! Layout and A4 page functionality for EditorBuffer
//!
//! This module contains A4 page layout, margins, tab width, line wrapping, and spacing calculations.

use super::buffer::EditorBuffer;
use gtk4::cairo::Context;

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
    /// Update page margins for A4 mode (stubbed for now)
    pub fn update_margins(&mut self, _top: f64, _bottom: f64, _left: f64, _right: f64) {
        // TODO: Implement with new config structure
        println!("[DEBUG] update_margins called but not implemented yet");
    }

    /// Clamp margin value to valid range for A4 (in cm)
    pub fn clamp_margin(cm: f64, min: f64, max: f64) -> f64 {
        cm.max(min).min(max)
    }

    /// Set top margin (constrained by A4 page size) - stubbed
    pub fn set_top_margin_cm(&mut self, _cm: f64) {
        println!("[DEBUG] set_top_margin_cm called but not implemented yet");
    }

    /// Set bottom margin (constrained by A4 page size) - stubbed  
    pub fn set_bottom_margin_cm(&mut self, _cm: f64) {
        println!("[DEBUG] set_bottom_margin_cm called but not implemented yet");
    }

    /// Set left margin (constrained by A4 page size)
    pub fn set_left_margin_cm(&mut self, cm: f64) {
        let max: f64 = 21.0 - self.config.right_margin_cm;
        self.config.left_margin_cm = Self::clamp_margin(cm, 0.0, max.max(0.0));
    }

    /// Set right margin (constrained by A4 page size)
    pub fn set_right_margin_cm(&mut self, cm: f64) {
        let max: f64 = 21.0 - self.config.left_margin_cm;
        self.config.right_margin_cm = Self::clamp_margin(cm, 0.0, max.max(0.0));
    }

    /// Calculate max lines for A4 page layout
    pub fn max_lines_on_page(&self) -> usize {
        if !self.config.a4_mode { 
            return self.lines.len();
        }
        let a4_height_cm = 29.7;
        let usable_height_cm = a4_height_cm - self.config.top_margin_cm - self.config.bottom_margin_cm;
        let line_height_cm = self.font_line_height() / 96.0 * 2.54;
        let max_lines = (usable_height_cm / line_height_cm) as usize;
        max_lines.max(1) // At least 1 line
    }

    /// Insert page break for A4 mode
    pub fn insert_page_break(&mut self) {
        if self.config.a4_mode {
            let line = self.cursor.row;
            self.lines.insert(line, "
[PAGE BREAK]
".to_string());
            self.cursor.row += 1;
        }
    }

    /// Check if we need a page break for A4 mode
    pub fn check_auto_page_break(&mut self) {
        if self.config.a4_mode {
            let max_lines = self.max_lines_on_page();
            if self.cursor.row >= max_lines {
                self.insert_page_break();
            }
        }
    }

    /// Toggle A4 mode on/off
    pub fn toggle_a4_mode(&mut self) {
        self.config.a4_mode = !self.config.a4_mode;
        self.request_redraw();
        println!("[DEBUG] A4 mode: {}", if self.config.a4_mode { "enabled" } else { "disabled" });
    }

    /// Calculate A4 page layout dimensions
    pub fn a4_layout_dimensions(&self) -> (f64, f64, f64, f64) {
        if !self.config.a4_mode {
            return (0.0, 0.0, 800.0, 600.0); // Default fallback
        }

        let a4_width_cm = 21.0;
        let a4_height_cm = 29.7;
        let usable_width_cm = a4_width_cm - self.config.left_margin_cm - self.config.right_margin_cm;
        let usable_height_cm = a4_height_cm - self.config.top_margin_cm - self.config.bottom_margin_cm;

        (a4_width_cm, a4_height_cm, usable_width_cm, usable_height_cm)
    }

    /// Get page margins in pixels for A4 mode  
    pub fn a4_page_margins_px(&self) -> (f64, f64, f64, f64) {
        if !self.config.a4_mode {
            return (0.0, 0.0, 0.0, 0.0);
        }

        let cm_to_px = |cm: f64| cm / 2.54 * 96.0;
        (
            cm_to_px(self.config.left_margin_cm),
            cm_to_px(self.config.right_margin_cm),
            cm_to_px(self.config.top_margin_cm),
            cm_to_px(self.config.bottom_margin_cm)
        )
    }

    /// Check if position is within A4 page bounds
    pub fn within_a4_bounds(&self, x: f64, y: f64) -> bool {
        if self.config.a4_mode {
            let (_, _, usable_width_cm, usable_height_cm) = self.a4_layout_dimensions();
            let cm_to_px = |cm: f64| cm / 2.54 * 96.0;
            x >= 0.0 && x <= cm_to_px(usable_width_cm) && y >= 0.0 && y <= cm_to_px(usable_height_cm)
        } else {
            true
        }
    }

    /// Move cursor down by one A4 page
    pub fn move_a4_page_down(&mut self) {
        if self.config.a4_mode {
            let lines = self.max_lines_on_page();
            self.cursor.row = (self.cursor.row + lines).min(self.lines.len().saturating_sub(1));
            self.cursor.col = self.cursor.col.min(self.lines[self.cursor.row].len());
            println!("[DEBUG] A4 page down: moved to line {}", self.cursor.row);
        } else {
            // Fallback to regular page down
            self.move_page_down(25); // Standard page size
        }
    }

    /// Move cursor up by one A4 page
    pub fn move_a4_page_up(&mut self) {
        if self.config.a4_mode {
            let lines = self.max_lines_on_page();
            self.cursor.row = self.cursor.row.saturating_sub(lines);
            self.cursor.col = self.cursor.col.min(self.lines[self.cursor.row].len());
            println!("[DEBUG] A4 page up: moved to line {}", self.cursor.row);
        } else {
            // Fallback to regular page up
            self.move_page_up(25); // Standard page size
        }
    }

    /// Get the usable text area dimensions in A4 mode (in pixels)
    pub fn get_a4_text_area(&self) -> (f64, f64) {
        if !self.config.a4_mode {
            return (800.0, 600.0); // Default fallback
        }

        let a4_width_cm = 21.0;
        let a4_height_cm = 29.7;
        let usable_width_cm = a4_width_cm - self.config.left_margin_cm - self.config.right_margin_cm;
        let usable_height_cm = a4_height_cm - self.config.top_margin_cm - self.config.bottom_margin_cm;

        let cm_to_px = |cm: f64| cm / 2.54 * 96.0;
        (cm_to_px(usable_width_cm), cm_to_px(usable_height_cm))
    }

    /// Calculate how many characters fit on one line in A4 mode
    pub fn chars_per_a4_line(&self) -> usize {
        if !self.config.a4_mode {
            return 80; // Default fallback
        }
        
        let (text_width, _) = self.get_a4_text_area();
        let char_width = self.font_character_spacing();
        
        (text_width / char_width).floor() as usize
    }

    /// Check if word wrapping should occur at current cursor position
    pub fn should_wrap_line(&self) -> bool {
        if !self.word_wrap {
            return false;
        }
        
        if self.config.a4_mode {
            let max_chars = self.chars_per_a4_line();
            self.cursor.col >= max_chars
        } else {
            // Use a default wrap column
            self.cursor.col >= 120
        }
    }

    /// Auto-wrap current line if needed
    pub fn auto_wrap_line(&mut self) {
        if !self.should_wrap_line() {
            return;
        }
        
        // Find a good break point (space, punctuation)
        let line = &self.lines[self.cursor.row];
        let mut wrap_pos = self.cursor.col;
        
        // Look backward for a good break point
        while wrap_pos > 0 && !line.chars().nth(wrap_pos).map_or(false, |c| c.is_whitespace()) {
            wrap_pos -= 1;
        }
        
        // If no good break point found, force wrap at cursor
        if wrap_pos == 0 {
            wrap_pos = self.cursor.col;
        }
        
        // Split the line
        let current_line = &mut self.lines[self.cursor.row];
        let new_line = current_line.split_off(wrap_pos);
        
        // Insert the new line
        self.lines.insert(self.cursor.row + 1, new_line.trim_start().to_string());
        
        // Update cursor
        self.cursor.row += 1;
        self.cursor.col = 0;
        
        println!("[DEBUG] Auto-wrapped line at position {}", wrap_pos);
    }
}

/// Render A4 page boundary and margin guides
pub fn render_a4_boundary(buffer: &EditorBuffer, ctx: &Context, width: i32, height: i32) {
    if !buffer.config.a4_mode { 
        return; 
    }
    
    // A4 size in cm
    let a4_width_cm = 21.0;
    let a4_height_cm = 29.7;
    
    // Convert to px
    let a4_width_px = cm_to_px(a4_width_cm);
    let a4_height_px = cm_to_px(a4_height_cm);
    
    // Margins in px
    let left_px = cm_to_px(buffer.config.left_margin_cm);
    let right_px = cm_to_px(buffer.config.right_margin_cm);
    let top_px = cm_to_px(buffer.config.top_margin_cm);
    let bottom_px = cm_to_px(buffer.config.bottom_margin_cm);
    
    // Center A4 page in viewport
    let x = ((width as f64) - a4_width_px) / 2.0;
    let y = ((height as f64) - a4_height_px) / 2.0;
    
    // Draw A4 page boundary
    ctx.set_source_rgba(0.8, 0.8, 0.8, 1.0); // light gray
    ctx.rectangle(x, y, a4_width_px, a4_height_px);
    ctx.set_line_width(2.0);
    ctx.stroke().unwrap_or(());
    
    // Draw margin guides
    ctx.set_source_rgba(0.2, 0.6, 0.9, 0.5); // blue, semi-transparent
    ctx.set_line_width(1.0);
    
    // Top margin
    ctx.move_to(x, y + top_px);
    ctx.line_to(x + a4_width_px, y + top_px);
    ctx.stroke().unwrap_or(());
    
    // Bottom margin
    ctx.move_to(x, y + a4_height_px - bottom_px);
    ctx.line_to(x + a4_width_px, y + a4_height_px - bottom_px);
    ctx.stroke().unwrap_or(());
    
    // Left margin
    ctx.move_to(x + left_px, y);
    ctx.line_to(x + left_px, y + a4_height_px);
    ctx.stroke().unwrap_or(());
    
    // Right margin
    ctx.move_to(x + a4_width_px - right_px, y);
    ctx.line_to(x + a4_width_px - right_px, y + a4_height_px);
    ctx.stroke().unwrap_or(());
    
    // Draw text area outline
    ctx.set_source_rgba(0.9, 0.9, 0.9, 0.3);
    ctx.rectangle(
        x + left_px, 
        y + top_px, 
        a4_width_px - left_px - right_px, 
        a4_height_px - top_px - bottom_px
    );
    ctx.fill().unwrap_or(());
}
