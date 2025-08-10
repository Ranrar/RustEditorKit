//! File I/O operations for EditorBuffer
//!
//! This module contains file loading, saving, and cross-platform file operations.

use super::buffer::EditorBuffer;

// Re-export cross-platform file operations
pub use crate::crossplatform::{
    open_file as x_open_file, 
    save_file as x_save_file,
    list_fonts, 
    find_font, 
    open_font, 
    close_font, 
    FontHandle
};

impl EditorBuffer {
    /// Open a file and load its contents into the buffer (cross-platform)
    pub fn open_file(&mut self, path: &str) -> Result<(), String> {
        match x_open_file(path) {
            Ok(lines) => {
                self.lines = lines;
                self.cursor.row = 0;
                self.cursor.col = 0;
                self.selection = None;
                self.undo_stack.clear();
                self.redo_stack.clear();
                
                // Ensure we have at least one line
                if self.lines.is_empty() {
                    self.lines.push(String::new());
                }
                
                println!("[DEBUG] Opened file: {} ({} lines)", path, self.lines.len());
                Ok(())
            }
            Err(e) => {
                eprintln!("[ERROR] Failed to open file '{}': {}", path, e);
                Err(e)
            }
        }
    }

    /// Save buffer contents to a file (cross-platform)
    pub fn save_file(&self, path: &str) -> Result<(), String> {
        match x_save_file(path, &self.lines) {
            Ok(()) => {
                println!("[DEBUG] Saved file: {} ({} lines)", path, self.lines.len());
                Ok(())
            }
            Err(e) => {
                eprintln!("[ERROR] Failed to save file '{}': {}", path, e);
                Err(e)
            }
        }
    }

    /// Create a new empty buffer
    pub fn new_file(&mut self) {
        self.lines = vec![String::new()];
        self.cursor.row = 0;
        self.cursor.col = 0;
        self.selection = None;
        self.undo_stack.clear();
        self.redo_stack.clear();
        println!("[DEBUG] Created new empty file");
    }

    /// Check if the buffer has been modified since last save
    pub fn is_modified(&self) -> bool {
        // A simple heuristic: if there's anything in the undo stack, consider it modified
        !self.undo_stack.is_empty()
    }

    /// Get buffer statistics
    pub fn get_file_stats(&self) -> FileStats {
        let total_chars: usize = self.lines.iter().map(|line| line.len()).sum();
        let non_empty_lines = self.lines.iter().filter(|line| !line.is_empty()).count();
        
        FileStats {
            lines: self.lines.len(),
            non_empty_lines,
            characters: total_chars,
            cursor_line: self.cursor.row + 1, // 1-based for display
            cursor_column: self.cursor.col + 1, // 1-based for display
        }
    }

    /// Export buffer to different formats
    pub fn export_as_text(&self) -> String {
        self.lines.join("\n")
    }

    /// Import text into buffer (replacing current content)
    pub fn import_from_text(&mut self, text: &str) {
        self.push_undo();
        self.lines = text.lines().map(|line| line.to_string()).collect();
        
        // Ensure we have at least one line
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        
        self.cursor.row = 0;
        self.cursor.col = 0;
        self.selection = None;
        
        println!("[DEBUG] Imported text ({} lines)", self.lines.len());
    }

    /// Get the current file content as bytes
    pub fn get_content_bytes(&self) -> Vec<u8> {
        self.export_as_text().into_bytes()
    }

    /// Load content from bytes
    pub fn load_from_bytes(&mut self, bytes: &[u8]) -> Result<(), String> {
        match String::from_utf8(bytes.to_vec()) {
            Ok(text) => {
                self.import_from_text(&text);
                Ok(())
            }
            Err(e) => Err(format!("Invalid UTF-8 content: {}", e))
        }
    }
}

/// File statistics for display and analysis
#[derive(Debug, Clone)]
pub struct FileStats {
    pub lines: usize,
    pub non_empty_lines: usize,
    pub characters: usize,
    pub cursor_line: usize,  // 1-based
    pub cursor_column: usize, // 1-based
}

impl std::fmt::Display for FileStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lines: {}, Chars: {}, Cursor: {}:{}", 
               self.lines, self.characters, self.cursor_line, self.cursor_column)
    }
}
