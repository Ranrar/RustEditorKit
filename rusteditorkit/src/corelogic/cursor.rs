use std::time::{Duration, Instant};

/// Tracks runtime state for cursor rendering and behavior
#[derive(Debug, Clone)]
pub struct CursorState {
    pub visible: bool,
    pub last_blink: Instant,
    pub blink_enabled: bool,
    pub blink_rate: u64,
    pub hide_when_typing: bool,
    pub unicode_fallback: bool,
    pub last_typing: Option<Instant>,
}

impl CursorState {
    pub fn new(cfg: &CursorConfig) -> Self {
        let mut state = Self {
            visible: true,
            last_blink: Instant::now(),
            blink_enabled: cfg.cursor_blink,
            blink_rate: cfg.cursor_blink_rate,
            hide_when_typing: cfg.cursor_hide_when_typing,
            unicode_fallback: cfg.cursor_unicode_fallback,
            last_typing: None,
        };
        // If blinking is disabled, always keep cursor visible
        if !cfg.cursor_blink {
            state.visible = true;
            state.last_typing = None;
        }
        state
    }

    /// Call this on every timer tick to update blink state
    pub fn tick_blink(&mut self) {
        if self.blink_enabled {
            let now = Instant::now();
            if now.duration_since(self.last_blink) >= Duration::from_millis(self.blink_rate) {
                self.visible = !self.visible;
                self.last_blink = now;
            }
        } else {
            self.visible = true;
        }
    }

    /// Call this on key event to hide cursor if needed
    pub fn on_key_event(&mut self) {
        // println!("[CURSOR DEBUG] on_key_event: blink_enabled = {}, hide_when_typing = {}", self.blink_enabled, self.hide_when_typing);
        if self.hide_when_typing {
            if self.blink_enabled {
                self.visible = false;
                self.last_typing = Some(Instant::now());
                // println!("[CURSOR DEBUG] on_key_event: set visible = false (blinking enabled, hide_when_typing)");
            } else {
                // If blinking is disabled, always keep cursor visible
                self.visible = true;
                self.last_typing = None;
                // println!("[CURSOR DEBUG] on_key_event: set visible = true (blinking disabled, hide_when_typing)");
            }
        } else {
            // If not hiding when typing, ensure cursor is visible if blinking is disabled
            if !self.blink_enabled {
                self.visible = true;
                // println!("[CURSOR DEBUG] on_key_event: set visible = true (blinking disabled, not hiding when typing)");
            }
        }
    }

    /// Call this periodically to restore cursor after typing
    pub fn check_restore_after_typing(&mut self) {
        if self.hide_when_typing {
            if let Some(last) = self.last_typing {
                if Instant::now().duration_since(last) > Duration::from_millis(self.blink_rate) {
                    self.visible = true;
                    self.last_typing = None;
                }
            }
        }
    }

    /// Returns true if cursor should be drawn
    pub fn is_cursor_visible(&self) -> bool {
        // println!("[CURSOR DEBUG] is_cursor_visible: visible = {}, blink_enabled = {}, hide_when_typing = {}", self.visible, self.blink_enabled, self.hide_when_typing);
        self.visible
    }

    /// Returns true if unicode fallback should be used
    pub fn use_unicode_fallback(&self) -> bool {
        self.unicode_fallback
    }
}
// Cursor movement and selection logic for EditorBuffer
//
// This module contains all cursor movement, selection, and multi-cursor functionality.
/// Modular cursor configuration for the editor
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CursorConfig {
    pub cursor_type: String,            // "bar", "block", "underline", or custom shape
    pub cursor_color: String,           // Hex or RGBA color
    pub cursor_blink: bool,             // Enables blinking behavior
    pub cursor_blink_rate: u64,         // Milliseconds for blink interval
    pub cursor_thickness: f64,          // Line width for "bar" or underline type
    pub cursor_padding_x: f64,          // Extra spacing to left/right of cursor
    pub cursor_padding_y: f64,          // Vertical offset, useful for line alignment
    pub cursor_roundness: f64,          // Rounded corners for block cursor
    pub cursor_anti_alias: bool,        // Enable smooth edges on custom-drawn cursors
    pub cursor_unicode_fallback: bool,  // Ensure cursor handles multibyte/emoji width
    pub cursor_hide_when_typing: bool,  // Auto-hide cursor while typing (like some IDEs)
}

impl Default for CursorConfig {
    fn default() -> Self {
        Self {
            cursor_type: "bar".to_string(),
            cursor_color: "#000000".to_string(),
            cursor_blink: true,
            cursor_blink_rate: 500,
            cursor_thickness: 2.0,
            cursor_padding_x: 1.0,
            cursor_padding_y: 0.5,
            cursor_roundness: 0.0,
            cursor_anti_alias: true,
            cursor_unicode_fallback: true,
            cursor_hide_when_typing: false,
        }
    }
}

use super::buffer::EditorBuffer;
use crate::corelogic::Selection;

impl EditorBuffer {
    /// Update cursor state from the latest config (call after config changes)
    pub fn update_cursor_state_from_config(&mut self) {
        self.cursor_state = CursorState::new(&self.config.cursor);
        // If blinking is disabled, always keep cursor visible
        if !self.config.cursor.cursor_blink {
            self.cursor_state.visible = true;
            self.cursor_state.last_typing = None;
        }
    }
    /// Get mutable reference to cursor state
    pub fn cursor_state_mut(&mut self) -> &mut CursorState {
        &mut self.cursor_state
    }
    /// Get reference to cursor state
    pub fn cursor_state(&self) -> &CursorState {
        &self.cursor_state
    }
}

impl EditorBuffer {
    /// Move cursor to start of line
    pub fn move_to_line_start(&mut self) {
        println!("[DEBUG] move_to_line_start");
        self.cursor.col = 0;
    }

    /// Move cursor to end of line
    pub fn move_to_line_end(&mut self) {
        println!("[DEBUG] move_to_line_end");
        if self.cursor.row < self.lines.len() {
            self.cursor.col = self.lines[self.cursor.row].len();
        }
    }

    /// Move cursor left (with bounds checking)
    pub fn move_left(&mut self) {
        // Clear selection on movement (non-Shift movement)
        self.clear_selection();
        self.move_left_internal();
    }

    /// Internal move left without clearing selection
    fn move_left_internal(&mut self) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        } else if self.cursor.row > 0 {
            self.cursor.row -= 1;
            self.cursor.col = self.lines[self.cursor.row].chars().count();
        }
    }

    /// Move cursor right (with bounds checking)
    pub fn move_right(&mut self) {
        // Clear selection on movement (non-Shift movement)
        self.clear_selection();
        self.move_right_internal();
    }

    /// Internal move right without clearing selection
    fn move_right_internal(&mut self) {
        if self.cursor.col < self.lines[self.cursor.row].chars().count() {
            self.cursor.col += 1;
        } else if self.cursor.row + 1 < self.lines.len() {
            self.cursor.row += 1;
            self.cursor.col = 0;
        }
    }

    /// Move cursor up (with bounds checking)
    pub fn move_up(&mut self) {
        // Clear selection on movement (non-Shift movement)
        self.clear_selection();
        self.move_up_internal();
    }

    /// Internal move up without clearing selection
    fn move_up_internal(&mut self) {
        if self.cursor.row > 0 {
            self.cursor.row -= 1;
            self.cursor.col = self.cursor.col.min(self.lines[self.cursor.row].chars().count());
        }
    }

    /// Move cursor down (with bounds checking)
    pub fn move_down(&mut self) {
        // Clear selection on movement (non-Shift movement)
        self.clear_selection();
        self.move_down_internal();
    }

    /// Internal move down without clearing selection
    fn move_down_internal(&mut self) {
        if self.cursor.row + 1 < self.lines.len() {
            self.cursor.row += 1;
            self.cursor.col = self.cursor.col.min(self.lines[self.cursor.row].chars().count());
        }
    }

    /// Move cursor to start of line
    pub fn move_home(&mut self) {
        // Clear selection on movement (non-Shift movement)
        self.clear_selection();
        self.cursor.col = 0;
    }

    /// Move cursor to end of line
    pub fn move_end(&mut self) {
        // Clear selection on movement (non-Shift movement)
        self.clear_selection();
        self.cursor.col = self.lines[self.cursor.row].chars().count();
    }

    /// Move cursor up by one visible page (PgUp)
    pub fn move_page_up(&mut self, lines_per_page: usize) {
        // Clear selection on movement (non-Shift movement)
        self.clear_selection();
        
        if self.cursor.row > lines_per_page {
            self.cursor.row -= lines_per_page;
        } else {
            self.cursor.row = 0;
        }
        self.cursor.col = self.cursor.col.min(self.lines[self.cursor.row].chars().count());
    }

    /// Move cursor down by one visible page (PgDn)
    pub fn move_page_down(&mut self, lines_per_page: usize) {
        // Clear selection on movement (non-Shift movement)
        self.clear_selection();
        
        let max_row = self.lines.len().saturating_sub(1);
        if self.cursor.row + lines_per_page < max_row {
            self.cursor.row += lines_per_page;
        } else {
            self.cursor.row = max_row;
        }
        self.cursor.col = self.cursor.col.min(self.lines[self.cursor.row].chars().count());
    }

    /// Start or extend selection to the left
    pub fn select_left(&mut self) {
        let prev_cursor = self.cursor;
        self.move_left_internal(); // Use internal version that doesn't clear selection
        let new_cursor = self.cursor;
        if prev_cursor != new_cursor {
            match &mut self.selection {
                Some(sel) => {
                    sel.end_row = new_cursor.row;
                    sel.end_col = new_cursor.col;
                    sel.clamp_to_buffer(&self.lines);
                    if sel.start_row == sel.end_row && sel.start_col == sel.end_col {
                        self.selection = None;
                    }
                }
                None => {
                    let mut sel = Selection::new(prev_cursor.row, prev_cursor.col);
                    sel.set(prev_cursor.row, prev_cursor.col, new_cursor.row, new_cursor.col);
                    self.selection = Some(sel);
                }
            }
        }
        println!("[DEBUG] select_left: {:?}", self.selection);
    }

    /// Start or extend selection to the right
    pub fn select_right(&mut self) {
        let prev_cursor = self.cursor;
        self.move_right_internal(); // Use internal version that doesn't clear selection
        let new_cursor = self.cursor;
        if prev_cursor != new_cursor {
            match &mut self.selection {
                Some(sel) => {
                    sel.end_row = new_cursor.row;
                    sel.end_col = new_cursor.col;
                    sel.clamp_to_buffer(&self.lines);
                    if sel.start_row == sel.end_row && sel.start_col == sel.end_col {
                        self.selection = None;
                    }
                }
                None => {
                    let mut sel = Selection::new(prev_cursor.row, prev_cursor.col);
                    sel.set(prev_cursor.row, prev_cursor.col, new_cursor.row, new_cursor.col);
                    self.selection = Some(sel);
                }
            }
        }
        println!("[DEBUG] select_right: {:?}", self.selection);
    }

    /// Start or extend selection up
    pub fn select_up(&mut self) {
        let prev_cursor = self.cursor;
        self.move_up_internal(); // Use internal version that doesn't clear selection
        let new_cursor = self.cursor;
        if prev_cursor != new_cursor {
            match &mut self.selection {
                Some(sel) => {
                    sel.end_row = new_cursor.row;
                    sel.end_col = new_cursor.col;
                    sel.clamp_to_buffer(&self.lines);
                    if sel.start_row == sel.end_row && sel.start_col == sel.end_col {
                        self.selection = None;
                    }
                }
                None => {
                    let mut sel = Selection::new(prev_cursor.row, prev_cursor.col);
                    sel.set(prev_cursor.row, prev_cursor.col, new_cursor.row, new_cursor.col);
                    self.selection = Some(sel);
                }
            }
        }
        println!("[DEBUG] select_up: {:?}", self.selection);
    }

    /// Start or extend selection down
    pub fn select_down(&mut self) {
        let prev_cursor = self.cursor;
        self.move_down_internal(); // Use internal version that doesn't clear selection
        let new_cursor = self.cursor;
        if prev_cursor != new_cursor {
            match &mut self.selection {
                Some(sel) => {
                    sel.end_row = new_cursor.row;
                    sel.end_col = new_cursor.col;
                    sel.clamp_to_buffer(&self.lines);
                    if sel.start_row == sel.end_row && sel.start_col == sel.end_col {
                        self.selection = None;
                    }
                }
                None => {
                    let mut sel = Selection::new(prev_cursor.row, prev_cursor.col);
                    sel.set(prev_cursor.row, prev_cursor.col, new_cursor.row, new_cursor.col);
                    self.selection = Some(sel);
                }
            }
        }
        println!("[DEBUG] select_down: {:?}", self.selection);
    }

    /// Select all text in the buffer
    pub fn select_all(&mut self) {
        if !self.lines.is_empty() {
            let mut sel = Selection::new(0, 0);
            let end_row = self.lines.len() - 1;
            let end_col = self.lines[end_row].len();
            sel.set(0, 0, end_row, end_col);
            self.selection = Some(sel);
            println!("[DEBUG] select_all: {:?}", self.selection);
        }
    }

    /// Clear current selection
    pub fn clear_selection(&mut self) {
        self.selection = None;
        println!("[DEBUG] clear_selection");
    }
}
