//! Text editing operations for EditorBuffer
//!
//! This module contains all text insertion, deletion, and modification operations.

use super::buffer::EditorBuffer;

impl EditorBuffer {
    /// Delete character before cursor (backspace)
    pub fn backspace(&mut self) {
        // If there's a selection, delete it instead of just the character
        if self.delete_selection() {
            return;
        }
        
        if self.cursor.col > 0 {
            self.push_undo();
            let line = &mut self.lines[self.cursor.row];
            
            // Convert cursor.col to byte index for the character before cursor
            let chars: Vec<(usize, char)> = line.char_indices().collect();
            if let Some((byte_idx, _)) = chars.get(self.cursor.col - 1) {
                line.remove(*byte_idx);
            }
            self.cursor.col -= 1;
        } else if self.cursor.row > 0 {
            self.push_undo();
            let prev_len = self.lines[self.cursor.row - 1].chars().count();
            let current = self.lines.remove(self.cursor.row);
            self.cursor.row -= 1;
            self.cursor.col = prev_len;
            self.lines[self.cursor.row].push_str(&current);
        }
    }

    /// Delete character at cursor (delete)
    pub fn delete(&mut self) {
        // If there's a selection, delete it instead of just the character
        if self.delete_selection() {
            return;
        }
        
        if self.cursor.row < self.lines.len() {
            if self.cursor.col < self.lines[self.cursor.row].chars().count() {
                self.push_undo();
                let line = &mut self.lines[self.cursor.row];
                
                // Convert cursor.col to byte index for the character at cursor
                let chars: Vec<(usize, char)> = line.char_indices().collect();
                if let Some((byte_idx, _)) = chars.get(self.cursor.col) {
                    line.remove(*byte_idx);
                }
            } else if self.cursor.row + 1 < self.lines.len() {
                self.push_undo();
                let next_line = self.lines.remove(self.cursor.row + 1);
                self.lines[self.cursor.row].push_str(&next_line);
            }
        }
    }

    /// Insert text at current cursor position
    pub fn insert_text(&mut self, text: &str) {
        // If there's a selection, delete it first (typing replaces selection)
        self.delete_selection();
        
        self.push_undo();
        
        // Handle newline insertions
        if text.contains('\n') {
            let lines: Vec<&str> = text.split('\n').collect();
            let current_line = &mut self.lines[self.cursor.row];
            
            // Convert cursor.col (char index) to byte index safely
            let cursor_byte_idx = current_line.char_indices()
                .nth(self.cursor.col)
                .map(|(idx, _)| idx)
                .unwrap_or(current_line.len());
            
            // Split current line at cursor using byte index
            let after_cursor = current_line.split_off(cursor_byte_idx);
            current_line.push_str(lines[0]);
            
            // Insert intermediate lines
            for (i, line) in lines.iter().enumerate().skip(1) {
                let new_line = if i == lines.len() - 1 {
                    format!("{}{}", line, after_cursor)
                } else {
                    line.to_string()
                };
                self.lines.insert(self.cursor.row + i, new_line);
            }
            
            // Update cursor position
            self.cursor.row += lines.len() - 1;
            self.cursor.col = if lines.len() > 1 {
                lines.last().unwrap().chars().count()
            } else {
                self.cursor.col + text.chars().count()
            };
        } else {
            // Simple text insertion
            let line = &mut self.lines[self.cursor.row];
            
            // Convert cursor.col (char index) to byte index safely
            let cursor_byte_idx = line.char_indices()
                .nth(self.cursor.col)
                .map(|(idx, _)| idx)
                .unwrap_or(line.len());
            
            line.insert_str(cursor_byte_idx, text);
            self.cursor.col += text.chars().count();
        }
    }

    /// Insert a newline at current cursor position
    pub fn insert_newline(&mut self) {
        // If there's a selection, delete it first
        self.delete_selection();
        
        self.push_undo();
        let current_line = &mut self.lines[self.cursor.row];
        
        // Convert cursor.col (char index) to byte index safely
        let cursor_byte_idx = current_line.char_indices()
            .nth(self.cursor.col)
            .map(|(idx, _)| idx)
            .unwrap_or(current_line.len());
        
        let after_cursor = current_line.split_off(cursor_byte_idx);
        
        self.cursor.row += 1;
        self.cursor.col = 0;
        self.lines.insert(self.cursor.row, after_cursor);
    }

    /// Paste text at cursor
    pub fn paste(&mut self, text: &str) {
        self.insert_text(text);
    }

    /// Delete the current line
    pub fn delete_line(&mut self) {
        if self.lines.len() > 1 {
            self.push_undo();
            self.lines.remove(self.cursor.row);
            
            // Adjust cursor if we deleted the last line
            if self.cursor.row >= self.lines.len() {
                self.cursor.row = self.lines.len() - 1;
            }
            
            // Clamp column to line length
            self.cursor.col = self.cursor.col.min(self.lines[self.cursor.row].len());
        } else {
            // If only one line, just clear it
            self.push_undo();
            self.lines[0].clear();
            self.cursor.col = 0;
        }
    }

    /// Duplicate the current line
    pub fn duplicate_line(&mut self) {
        self.push_undo();
        let line_content = self.lines[self.cursor.row].clone();
        self.lines.insert(self.cursor.row + 1, line_content);
        self.cursor.row += 1;
    }

    /// Delete selected text if any selection exists
    pub fn delete_selection(&mut self) -> bool {
        if let Some(sel) = self.selection.clone() {
            self.push_undo();
            
            let ((start_row, start_col), (end_row, end_col)) = sel.normalized();
            
            if start_row == end_row {
                // Single line deletion - use character-based operations
                let line = &mut self.lines[start_row];
                let chars: Vec<char> = line.chars().collect();
                
                // Rebuild the line without the selected characters
                let before: String = chars.get(..start_col).unwrap_or(&[]).iter().collect();
                let after: String = chars.get(end_col..).unwrap_or(&[]).iter().collect();
                *line = format!("{}{}", before, after);
                
                self.cursor.row = start_row;
                self.cursor.col = start_col;
            } else {
                // Multi-line deletion - use character-based operations
                let end_line = &self.lines[end_row];
                let end_chars: Vec<char> = end_line.chars().collect();
                let end_part: String = end_chars.get(end_col..).unwrap_or(&[]).iter().collect();
                
                let start_line = &mut self.lines[start_row];
                let start_chars: Vec<char> = start_line.chars().collect();
                let before_part: String = start_chars.get(..start_col).unwrap_or(&[]).iter().collect();
                
                *start_line = format!("{}{}", before_part, end_part);
                
                // Remove intermediate lines
                for _ in start_row + 1..=end_row {
                    self.lines.remove(start_row + 1);
                }
                
                self.cursor.row = start_row;
                self.cursor.col = start_col;
            }
            
            self.selection = None;
            true
        } else {
            false
        }
    }

    /// Clear selection without deleting text - used for navigation and input keys
    pub fn clear_selection_if_exists(&mut self) {
        if self.selection.is_some() {
            self.selection = None;
        }
    }

    // ...existing code...
    // Pointer/mouse logic is now delegated to corelogic::pointer
    pub fn screen_to_buffer_position(&self, x: f64, y: f64, layout: &crate::render::layout::LayoutMetrics, pango_ctx: &gtk4::pango::Context, font_desc: &gtk4::pango::FontDescription) -> (usize, usize) {
        crate::corelogic::mouse::screen_to_buffer_position(self, x, y, layout, pango_ctx, font_desc)
    }

    pub fn handle_mouse_click(&mut self, x: f64, y: f64, shift_held: bool, layout: &crate::render::layout::LayoutMetrics, pango_ctx: &gtk4::pango::Context, font_desc: &gtk4::pango::FontDescription) {
        crate::corelogic::mouse::handle_mouse_click(self, x, y, shift_held, layout, pango_ctx, font_desc)
    }

    pub fn handle_mouse_drag(&mut self, x: f64, y: f64, layout: &crate::render::layout::LayoutMetrics, pango_ctx: &gtk4::pango::Context, font_desc: &gtk4::pango::FontDescription) {
        crate::corelogic::mouse::handle_mouse_drag(self, x, y, layout, pango_ctx, font_desc)
    }

    pub fn handle_mouse_release(&mut self) {
        crate::corelogic::mouse::handle_mouse_release(self)
    }

    pub fn handle_double_click(&mut self, x: f64, y: f64, layout: &crate::render::layout::LayoutMetrics, pango_ctx: &gtk4::pango::Context, font_desc: &gtk4::pango::FontDescription) {
        crate::corelogic::mouse::handle_double_click(self, x, y, layout, pango_ctx, font_desc)
    }

    pub fn handle_triple_click(&mut self, x: f64, y: f64, layout: &crate::render::layout::LayoutMetrics, pango_ctx: &gtk4::pango::Context, font_desc: &gtk4::pango::FontDescription) {
        crate::corelogic::mouse::handle_triple_click(self, x, y, layout, pango_ctx, font_desc)
    }

    /// Get the currently selected text
    pub fn get_selected_text(&self) -> Option<String> {
        if let Some(sel) = &self.selection {
            let ((start_row, start_col), (end_row, end_col)) = sel.normalized();
            
            if start_row == end_row {
                // Single line selection - use character-based slicing
                let line = &self.lines[start_row];
                let chars: Vec<char> = line.chars().collect();
                let selected_chars = chars.get(start_col..end_col).unwrap_or(&[]);
                Some(selected_chars.iter().collect::<String>())
            } else {
                // Multi-line selection
                let mut result = String::new();
                
                // First line - use character-based slicing
                let first_line = &self.lines[start_row];
                let first_chars: Vec<char> = first_line.chars().collect();
                let first_selected = first_chars.get(start_col..).unwrap_or(&[]);
                result.push_str(&first_selected.iter().collect::<String>());
                result.push('\n');
                
                // Intermediate lines
                for row in start_row + 1..end_row {
                    result.push_str(&self.lines[row]);
                    result.push('\n');
                }
                
                // Last line - use character-based slicing
                let last_line = &self.lines[end_row];
                let last_chars: Vec<char> = last_line.chars().collect();
                let last_selected = last_chars.get(..end_col).unwrap_or(&last_chars);
                result.push_str(&last_selected.iter().collect::<String>());
                
                Some(result)
            }
        } else {
            None
        }
    }

    /// Insert indentation (tab or spaces) at current cursor position or all selected lines
    pub fn indent(&mut self) {
        self.push_undo();
        
        // For now, use hardcoded defaults - can be made configurable later
        // Use 4 spaces as default indent
        let indent_str = "    ";
        
        if let Some(sel) = &self.selection {
            // Indent all selected lines
            let ((start_row, _), (end_row, _)) = sel.normalized();
            
            for row in start_row..=end_row {
                if row < self.lines.len() {
                    self.lines[row].insert_str(0, indent_str);
                }
            }
            
            // Adjust cursor and selection to account for added indentation
            self.cursor.col += indent_str.len();
            
            // Update selection to reflect the new positions
            if let Some(sel) = &mut self.selection {
                sel.start_col += indent_str.len();
                sel.end_col += indent_str.len();
            }
        } else {
            // Single line indent - use existing insert_text logic
            self.insert_text(indent_str);
        }
    }

    /// Remove one level of indentation from current line or all selected lines
    pub fn unindent(&mut self) {
        self.push_undo();
        
        if let Some(sel) = &self.selection {
            // Unindent all selected lines
            let ((start_row, _), (end_row, _)) = sel.normalized();
            let mut total_removed_from_cursor_line = 0;
            let mut total_removed_from_start = 0;
            let mut total_removed_from_end = 0;
            
            for row in start_row..=end_row {
                if row < self.lines.len() {
                    let line = &mut self.lines[row];
                    let removed = unindent_single_line(line);
                    
                    // Track removals for cursor and selection adjustment
                    if row == self.cursor.row {
                        total_removed_from_cursor_line = removed;
                    }
                    if row == start_row {
                        total_removed_from_start = removed;
                    }
                    if row == end_row {
                        total_removed_from_end = removed;
                    }
                }
            }
            
            // Adjust cursor position
            if self.cursor.col >= total_removed_from_cursor_line {
                self.cursor.col -= total_removed_from_cursor_line;
            } else {
                self.cursor.col = 0;
            }
            
            // Adjust selection positions
            if let Some(sel) = &mut self.selection {
                if sel.start_col >= total_removed_from_start {
                    sel.start_col -= total_removed_from_start;
                } else {
                    sel.start_col = 0;
                }
                
                if sel.end_col >= total_removed_from_end {
                    sel.end_col -= total_removed_from_end;
                } else {
                    sel.end_col = 0;
                }
            }
        } else {
            // Single line unindent
            let line = &mut self.lines[self.cursor.row];
            let removed = unindent_single_line(line);
            
            // Adjust cursor position
            if self.cursor.col >= removed {
                self.cursor.col -= removed;
            } else {
                self.cursor.col = 0;
            }
        }
    }
}

/// Helper function to unindent a single line and return the number of characters removed
fn unindent_single_line(line: &mut String) -> usize {
    // Try to remove 4 spaces first
    if line.starts_with("    ") {
        line.drain(..4);
        4
    }
    // If not 4 spaces, try to remove a tab
    else if line.starts_with('\t') {
        line.remove(0);
        1
    }
    // Otherwise, try to remove individual spaces at the beginning
    else if line.starts_with(' ') {
        let spaces_to_remove = line.chars().take_while(|&c| c == ' ').count().min(4);
        line.drain(..spaces_to_remove);
        spaces_to_remove
    } else {
        0
    }
}
