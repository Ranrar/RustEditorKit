//! Clipboard operations for EditorBuffer
//!
//! This module contains copy, cut, and paste operations with system clipboard integration.

use super::buffer::EditorBuffer;
use gtk4::gdk;
use gtk4::prelude::DisplayExt;

impl EditorBuffer {
    /// Return selected text or current line if no selection
    pub fn copy(&self) -> String {
        if let Some(sel) = &self.selection {
            let ((start_row, start_col), (end_row, end_col)) = sel.normalized();
            println!("[DEBUG] copy: selection=({},{}) to ({},{})", start_row, start_col, end_row, end_col);
            
            if start_row == end_row && start_row < self.lines.len() && end_col > start_col {
                // Single line selection - use character-based slicing
                let line = &self.lines[start_row];
                let chars: Vec<char> = line.chars().collect();
                let selected_chars = chars.get(start_col..end_col).unwrap_or(&[]);
                return selected_chars.iter().collect::<String>();
            } else if start_row < self.lines.len() && end_row < self.lines.len() {
                // Multi-line selection
                let mut result = String::new();
                for row in start_row..=end_row {
                    let line: &String = &self.lines[row];
                    if row == start_row && row == end_row {
                        // Convert character indices to byte indices safely
                        let chars: Vec<char> = line.chars().collect();
                        let start_chars = chars.get(start_col..).unwrap_or(&[]);
                        let selected_chars = start_chars.get(..end_col.saturating_sub(start_col)).unwrap_or(start_chars);
                        result.push_str(&selected_chars.iter().collect::<String>());
                    } else if row == start_row {
                        // Get text from start_col to end of line
                        let chars: Vec<char> = line.chars().collect();
                        let selected_chars = chars.get(start_col..).unwrap_or(&[]);
                        result.push_str(&selected_chars.iter().collect::<String>());
                        result.push('\n');
                    } else if row == end_row {
                        // Get text from start of line to end_col
                        let chars: Vec<char> = line.chars().collect();
                        let selected_chars = chars.get(..end_col).unwrap_or(&chars);
                        result.push_str(&selected_chars.iter().collect::<String>());
                    } else {
                        result.push_str(line);
                        result.push('\n');
                    }
                }
                return result;
            }
        }
        
        // No selection - return current line
        self.lines.get(self.cursor.row).cloned().unwrap_or_default()
    }

    /// Copy selected text to system clipboard (GTK4 GDK API)
    pub fn copy_to_clipboard(&self) {
        let text = self.copy();
        if let Some(display) = gdk::Display::default() {
            let clipboard = display.clipboard();
            clipboard.set_text(&text);
            println!("[DEBUG] Copied to clipboard: {:?}", text);
        } else {
            eprintln!("[ERROR] No display found for clipboard access");
        }
    }

    /// Cut selected text to clipboard and delete it from buffer
    pub fn cut_to_clipboard(&mut self) {
        let text = self.copy();
        if let Some(display) = gdk::Display::default() {
            let clipboard = display.clipboard();
            clipboard.set_text(&text);
            
            // Delete the selected text or current line
            if self.selection.is_some() {
                self.delete_selection();
            } else {
                self.delete_line();
            }
            
            println!("[DEBUG] Cut to clipboard: {:?}", text);
        } else {
            eprintln!("[ERROR] No display found for clipboard access");
        }
    }

    /// Request paste from system clipboard
    /// Note: Due to async nature of GTK4 clipboard, actual implementation
    /// should be handled at the widget level with proper async handling
    pub fn paste_from_clipboard(&mut self) {
        println!("[DEBUG] Paste from clipboard requested");
        
        // The async clipboard access should be implemented at the widget level
        // For now, indicate that this needs to be handled elsewhere
        println!("[DEBUG] Clipboard paste requires widget-level async handling");
    }

    /// Synchronous paste operation (requires clipboard text to be provided)
    pub fn paste_text(&mut self, text: &str) {
        if !text.is_empty() {
            // Delete selection if any
            if self.selection.is_some() {
                self.delete_selection();
            }
            
            // Insert the text at cursor
            self.insert_text(text);
            println!("[DEBUG] Pasted text: {:?}", text);
        }
    }

    /// Check if there's text selected that can be copied
    pub fn has_selection(&self) -> bool {
        self.selection.is_some()
    }

    /// Get the text that would be copied (for preview purposes)
    pub fn get_copy_preview(&self) -> String {
        let text = self.copy();
        if text.len() > 50 {
            format!("{}...", &text[..47])
        } else {
            text
        }
    }
}
