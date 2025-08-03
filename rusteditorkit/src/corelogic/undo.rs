//! Undo/redo functionality for EditorBuffer
//!
//! This module contains all undo/redo stack management and state operations.

use super::buffer::{EditorBuffer, EditorCursor};

/// State for undo/redo selection and cursor
#[derive(Clone, Debug)]
pub struct BufferState {
    pub lines: Vec<String>,
    pub selection: Option<crate::corelogic::selection::Selection>,
    pub cursor: EditorCursor,
}

impl EditorBuffer {
    /// Push current buffer state to undo stack and clear redo stack
    pub fn push_undo(&mut self) {
        let state = BufferState {
            lines: self.lines.clone(),
            selection: self.selection.clone(),
            cursor: self.cursor,
        };
        self.undo_stack.push(state);
        self.redo_stack.clear();
        
        // Limit undo stack size to prevent memory issues
        const MAX_UNDO_STACK_SIZE: usize = 100;
        if self.undo_stack.len() > MAX_UNDO_STACK_SIZE {
            self.undo_stack.remove(0);
        }
    }

    /// Undo last buffer state
    pub fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            let current_state = BufferState {
                lines: self.lines.clone(),
                selection: self.selection.clone(),
                cursor: self.cursor,
            };
            self.redo_stack.push(current_state);
            
            self.lines = prev.lines;
            self.selection = prev.selection;
            self.cursor = prev.cursor;
            
            println!("[DEBUG] Undo applied - cursor: {:?}", self.cursor);
        }
    }

    /// Redo last buffer state
    pub fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            let current_state = BufferState {
                lines: self.lines.clone(),
                selection: self.selection.clone(),
                cursor: self.cursor,
            };
            self.undo_stack.push(current_state);
            
            self.lines = next.lines;
            self.selection = next.selection;
            self.cursor = next.cursor;
            
            println!("[DEBUG] Redo applied - cursor: {:?}", self.cursor);
        }
    }

    /// Undo selection and cursor position only (legacy method)
    pub fn undo_selection_cursor(&mut self) {
        self.undo();
    }

    /// Redo selection and cursor position only (legacy method)
    pub fn redo_selection_cursor(&mut self) {
        self.redo();
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clear undo/redo stacks
    pub fn clear_undo_history(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        println!("[DEBUG] Undo history cleared");
    }

    /// Get undo stack size for debugging
    pub fn undo_stack_size(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get redo stack size for debugging
    pub fn redo_stack_size(&self) -> usize {
        self.redo_stack.len()
    }
}
