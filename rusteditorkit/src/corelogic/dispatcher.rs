//! Centralized command dispatcher for RustEditorKit
//! 
//! This module provides a unified command system that:
//! - Centralizes all editor actions in one place
//! - Provides consistent error handling and validation
//! - Enables undo/redo for all operations
//! - Supports command queuing and batch operations
//! - Makes the system extensible for plugins

use super::buffer::EditorBuffer;
use crate::keybinds::editor_action::EditorAction;
use std::fmt;

/// Result type for command execution
pub type CommandResult<T = ()> = Result<T, CommandError>;

/// Errors that can occur during command execution
#[derive(Debug, Clone)]
pub enum CommandError {
    /// Command cannot be executed in current state
    InvalidState(String),
    /// Command parameters are invalid
    InvalidParameters(String),
    /// Buffer operation failed
    BufferError(String),
    /// Clipboard operation failed
    ClipboardError(String),
    /// File operation failed
    FileError(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            CommandError::InvalidParameters(msg) => write!(f, "Invalid parameters: {}", msg),
            CommandError::BufferError(msg) => write!(f, "Buffer error: {}", msg),
            CommandError::ClipboardError(msg) => write!(f, "Clipboard error: {}", msg),
            CommandError::FileError(msg) => write!(f, "File error: {}", msg),
        }
    }
}

impl std::error::Error for CommandError {}

/// Parameters for commands that need additional data
#[derive(Debug, Clone)]
pub enum CommandParams {
    None,
    PageLines(usize),
    Text(String),
    Position { row: usize, col: usize },
    FilePath(String),
}

/// Centralized command dispatcher for all editor actions
pub struct CommandDispatcher {
    /// Enable debug logging for commands
    debug_mode: bool,
    /// Command history for debugging
    command_history: Vec<(EditorAction, CommandParams)>,
}

impl CommandDispatcher {
    /// Create a new command dispatcher
    pub fn new() -> Self {
        Self {
            debug_mode: false,
            command_history: Vec::new(),
        }
    }

    /// Enable/disable debug mode
    pub fn set_debug_mode(&mut self, enabled: bool) {
        self.debug_mode = enabled;
    }

    /// Get command history
    pub fn command_history(&self) -> &[(EditorAction, CommandParams)] {
        &self.command_history
    }

    /// Validate buffer state before command execution
    fn validate_buffer_state(&self, buffer: &EditorBuffer) -> CommandResult {
        if buffer.lines.is_empty() {
            return Err(CommandError::InvalidState("Buffer has no lines".to_string()));
        }

        if buffer.cursor.row >= buffer.lines.len() {
            return Err(CommandError::InvalidState(format!(
                "Cursor row {} out of bounds (max {})", 
                buffer.cursor.row, 
                buffer.lines.len() - 1
            )));
        }

        if buffer.cursor.col > buffer.lines[buffer.cursor.row].len() {
            return Err(CommandError::InvalidState(format!(
                "Cursor col {} out of bounds for line {} (max {})", 
                buffer.cursor.col, 
                buffer.cursor.row,
                buffer.lines[buffer.cursor.row].len()
            )));
        }

        Ok(())
    }

    /// Check if selection should be cleared for a given action
    /// Returns true if selection should be cleared, false if it should be preserved
    fn should_clear_selection_for_action(action: &EditorAction) -> bool {
        match action {
            // Movement keys without Shift - clear selection
            EditorAction::MoveCursorLeft | EditorAction::MoveCursorRight |
            EditorAction::MoveCursorUp | EditorAction::MoveCursorDown |
            EditorAction::MoveCursorHome | EditorAction::MoveCursorStartOfLine |
            EditorAction::MoveCursorEnd | EditorAction::MoveCursorEndOfLine |
            EditorAction::MoveCursorPageUp | EditorAction::MoveCursorPageDown => true,
            
            // Input and editing keys - clear selection (handled in their methods)
            EditorAction::InsertText | EditorAction::InsertNewline |
            EditorAction::Backspace | EditorAction::Delete |
            EditorAction::DeleteLeft | EditorAction::DeleteRight => false, // These handle selection themselves
            
            // Escape key - clear selection
            EditorAction::Escape | EditorAction::ClearSelection => true,
            
            // Selection keys - preserve selection (extend it)
            EditorAction::SelectLeft | EditorAction::SelectRight |
            EditorAction::SelectUp | EditorAction::SelectDown |
            EditorAction::SelectAll => false,
            
            // Copy/paste operations - preserve selection
            EditorAction::CopySelection | EditorAction::CutSelection |
            EditorAction::PasteClipboard => false,
            
            // Indent/unindent operations - preserve selection (they work on selected lines)
            EditorAction::Indent | EditorAction::Unindent => false,
            
            // File operations - preserve selection
            EditorAction::OpenFile | EditorAction::SaveFile | EditorAction::SaveAs |
            EditorAction::NewFile => false,
            
            // Search operations - clear selection (will create new selection if found)
            EditorAction::Find | EditorAction::FindNext | EditorAction::Replace => true,
            
            // Other operations - preserve selection by default
            _ => false,
        }
    }

    /// Execute an editor action with the given parameters
    pub fn execute(&mut self, buffer: &mut EditorBuffer, action: EditorAction, params: CommandParams) -> CommandResult {
        // Log command if debug mode is enabled
        if self.debug_mode {
            println!("[COMMAND] Executing {:?} with params {:?}", action, params);
        }

        // Add to history
        self.command_history.push((action, params.clone()));

        // Validate buffer state
        self.validate_buffer_state(buffer)?;

        // Auto-clear selection for appropriate actions
        if Self::should_clear_selection_for_action(&action) {
            buffer.clear_selection_if_exists();
        }

        // Execute the command
        let result = match action {
            // === Navigation Commands ===
            EditorAction::MoveCursorLeft => {
                buffer.move_left();
                Ok(())
            },
            EditorAction::MoveCursorRight => {
                buffer.move_right();
                Ok(())
            },
            EditorAction::MoveCursorUp => {
                buffer.move_up();
                Ok(())
            },
            EditorAction::MoveCursorDown => {
                buffer.move_down();
                Ok(())
            },
            EditorAction::MoveCursorHome | EditorAction::MoveCursorStartOfLine => {
                buffer.move_home();
                Ok(())
            },
            EditorAction::MoveCursorEnd | EditorAction::MoveCursorEndOfLine => {
                buffer.move_end();
                Ok(())
            },
            EditorAction::MoveCursorPageUp => {
                match params {
                    CommandParams::PageLines(lines) => {
                        buffer.move_page_up(lines);
                        Ok(())
                    },
                    CommandParams::None => {
                        buffer.move_page_up(25); // Default page size
                        Ok(())
                    },
                    _ => Err(CommandError::InvalidParameters("PageUp requires PageLines or None".to_string()))
                }
            },
            EditorAction::MoveCursorPageDown => {
                match params {
                    CommandParams::PageLines(lines) => {
                        buffer.move_page_down(lines);
                        Ok(())
                    },
                    CommandParams::None => {
                        buffer.move_page_down(25); // Default page size
                        Ok(())
                    },
                    _ => Err(CommandError::InvalidParameters("PageDown requires PageLines or None".to_string()))
                }
            },

            // === Selection Commands ===
            EditorAction::SelectLeft => {
                buffer.select_left();
                Ok(())
            },
            EditorAction::SelectRight => {
                buffer.select_right();
                Ok(())
            },
            EditorAction::SelectUp => {
                buffer.select_up();
                Ok(())
            },
            EditorAction::SelectDown => {
                buffer.select_down();
                Ok(())
            },
            EditorAction::SelectAll => {
                buffer.select_all();
                Ok(())
            },
            EditorAction::ClearSelection => {
                buffer.clear_selection();
                Ok(())
            },

            // === Editing Commands ===
            EditorAction::Backspace => {
                buffer.backspace();
                Ok(())
            },
            EditorAction::Delete => {
                buffer.delete();
                Ok(())
            },
            EditorAction::DeleteLeft => {
                buffer.backspace();
                Ok(())
            },
            EditorAction::DeleteRight => {
                buffer.delete();
                Ok(())
            },
            EditorAction::InsertNewline => {
                buffer.insert_newline();
                Ok(())
            },
            EditorAction::InsertText => {
                match params {
                    CommandParams::Text(text) => {
                        buffer.insert_text(&text);
                        Ok(())
                    },
                    _ => Err(CommandError::InvalidParameters("InsertText requires Text parameter".to_string()))
                }
            },
            EditorAction::Indent => {
                buffer.indent();
                Ok(())
            },
            EditorAction::Unindent => {
                buffer.unindent();
                Ok(())
            },

            // === Clipboard Commands ===
            EditorAction::CopySelection => {
                buffer.copy_to_clipboard();
                Ok(())
            },
            EditorAction::CutSelection => {
                buffer.cut_to_clipboard();
                Ok(())
            },
            EditorAction::PasteClipboard => {
                buffer.paste_from_clipboard();
                Ok(())
            },

            // === Undo/Redo Commands ===
            EditorAction::Undo => {
                buffer.undo();
                Ok(())
            },
            EditorAction::Redo => {
                buffer.redo();
                Ok(())
            },

            // === File Operations ===
            EditorAction::OpenFile => {
                match params {
                    CommandParams::FilePath(path) => {
                        buffer.open_file(&path)
                            .map_err(|e| CommandError::FileError(e))
                    },
                    _ => Err(CommandError::InvalidParameters("OpenFile requires FilePath parameter".to_string()))
                }
            },
            EditorAction::SaveFile => {
                match params {
                    CommandParams::FilePath(path) => {
                        buffer.save_file(&path)
                            .map_err(|e| CommandError::FileError(e))
                    },
                    _ => Err(CommandError::InvalidParameters("SaveFile requires FilePath parameter".to_string()))
                }
            },

            // === Layout Commands ===
            EditorAction::ToggleA4Mode => {
                buffer.toggle_a4_mode();
                Ok(())
            },

            // === Search Commands ===
            EditorAction::FindNext => {
                match params {
                    CommandParams::Text(query) => {
                        if let Some((row, col)) = buffer.find_next(&query, None) {
                            buffer.cursor.row = row;
                            buffer.cursor.col = col;
                        }
                        Ok(())
                    },
                    _ => Err(CommandError::InvalidParameters("FindNext requires Text parameter".to_string()))
                }
            },

            // === Multi-cursor Commands ===
            EditorAction::AddCursor => {
                match params {
                    CommandParams::Position { row, col } => {
                        if row < buffer.lines.len() && col <= buffer.lines[row].len() {
                            buffer.multi_cursors.push((row, col));
                            Ok(())
                        } else {
                            Err(CommandError::InvalidParameters("Cursor position out of bounds".to_string()))
                        }
                    },
                    _ => Err(CommandError::InvalidParameters("AddCursor requires Position parameter".to_string()))
                }
            },

            // === Catch-all for unimplemented actions ===
            _ => {
                Err(CommandError::InvalidState(format!("Command {:?} not yet implemented", action)))
            }
        };

        // Log result if debug mode is enabled
        if self.debug_mode {
            match &result {
                Ok(_) => println!("[COMMAND] Successfully executed {:?}", action),
                Err(e) => println!("[COMMAND] Failed to execute {:?}: {}", action, e),
            }
        }

        // Request redraw for commands that modify the buffer
        if self.should_redraw_after_command(&action) {
            buffer.request_redraw();
        }

        result
    }

    /// Determine if a redraw is needed after executing a command
    fn should_redraw_after_command(&self, action: &EditorAction) -> bool {
        match action {
            // Navigation and selection always need redraw
            EditorAction::MoveCursorLeft | EditorAction::MoveCursorRight |
            EditorAction::MoveCursorUp | EditorAction::MoveCursorDown |
            EditorAction::MoveCursorHome | EditorAction::MoveCursorStartOfLine |
            EditorAction::MoveCursorEnd | EditorAction::MoveCursorEndOfLine |
            EditorAction::MoveCursorPageUp | EditorAction::MoveCursorPageDown |
            EditorAction::SelectLeft | EditorAction::SelectRight |
            EditorAction::SelectUp | EditorAction::SelectDown |
            EditorAction::SelectAll | EditorAction::ClearSelection => true,

            // Editing operations need redraw
            EditorAction::Backspace | EditorAction::Delete |
            EditorAction::DeleteLeft | EditorAction::DeleteRight |
            EditorAction::InsertNewline | EditorAction::InsertText |
            EditorAction::Indent | EditorAction::Unindent |
            EditorAction::PasteClipboard => true,

            // Undo/Redo need redraw
            EditorAction::Undo | EditorAction::Redo => true,

            // File operations that change content need redraw
            EditorAction::OpenFile => true,

            // Layout changes need redraw
            EditorAction::ToggleA4Mode => true,

            // Search operations need redraw
            EditorAction::FindNext => true,

            // Copy operations don't need redraw
            EditorAction::CopySelection | EditorAction::CutSelection => false,

            // Save operations don't need redraw
            EditorAction::SaveFile => false,

            // Multi-cursor operations need redraw
            EditorAction::AddCursor => true,

            // Default to no redraw for unknown actions
            _ => false,
        }
    }

    /// Clear command history
    pub fn clear_history(&mut self) {
        self.command_history.clear();
        if self.debug_mode {
            println!("[COMMAND] Command history cleared");
        }
    }

    /// Get the last executed command
    pub fn last_command(&self) -> Option<&(EditorAction, CommandParams)> {
        self.command_history.last()
    }

    /// Check if a command can be executed (validation only, doesn't execute)
    pub fn can_execute(&self, buffer: &EditorBuffer, action: &EditorAction, params: &CommandParams) -> bool {
        // Basic buffer validation
        if buffer.lines.is_empty() {
            return false;
        }

        // Action-specific validation
        match action {
            EditorAction::MoveCursorLeft => buffer.cursor.col > 0 || buffer.cursor.row > 0,
            EditorAction::MoveCursorRight => {
                buffer.cursor.col < buffer.lines[buffer.cursor.row].len() || 
                buffer.cursor.row + 1 < buffer.lines.len()
            },
            EditorAction::MoveCursorUp => buffer.cursor.row > 0,
            EditorAction::MoveCursorDown => buffer.cursor.row + 1 < buffer.lines.len(),
            EditorAction::Undo => buffer.can_undo(),
            EditorAction::Redo => buffer.can_redo(),
            EditorAction::CopySelection | EditorAction::CutSelection => buffer.has_selection(),
            
            // File operations need valid paths
            EditorAction::OpenFile | EditorAction::SaveFile => {
                matches!(params, CommandParams::FilePath(_))
            },
            
            // Text operations need text parameter
            EditorAction::InsertText | EditorAction::FindNext => {
                matches!(params, CommandParams::Text(_))
            },
            
            // Position operations need valid position
            EditorAction::AddCursor => {
                if let CommandParams::Position { row, col } = params {
                    *row < buffer.lines.len() && *col <= buffer.lines[*row].len()
                } else {
                    false
                }
            },
            
            // Most other commands can always be executed
            _ => true,
        }
    }
}
