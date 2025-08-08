//! Core EditorBuffer struct and basic functionality
//!
//! This module contains the main EditorBuffer struct and basic methods
//! that don't fit into other specific categories.


use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use crate::corelogic::pointer::MouseState;

/// Represents the position of the cursor in the editor (row, col).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EditorCursor {
    pub row: usize,
    pub col: usize,
}

impl EditorCursor {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}


/// The main buffer struct for the custom code editor.
/// Holds all text, cursor, selection, undo/redo, theme, and rendering state.
pub struct EditorBuffer {
    /// Modular config for all editor appearance and behavior
    pub config: crate::config::configuration::EditorConfig,
    /// Lines of text in the buffer
    pub lines: Vec<String>,
    /// Cursor position
    pub cursor: EditorCursor,
    /// Scroll offset for vertical scrolling
    pub scroll_offset: usize,
    /// Whether to highlight the current line
    /// Selection (start/end)
    pub selection: Option<crate::corelogic::selection::Selection>,
    /// List of additional cursors (row, col)
    pub multi_cursors: Vec<(usize, usize)>,
    /// List of additional selections (start, end)
    pub multi_selections: Vec<(Option<(usize, usize)>, Option<(usize, usize)>)>,
    /// Current theme for syntax highlighting
    pub theme: syntect::highlighting::Theme,
    /// Syntax set for highlighting
    pub syntax_set: SyntaxSet,
    /// Undo stack for buffer edits, selection, and cursor
    pub undo_stack: Vec<super::undo::BufferState>,
    /// Redo stack for buffer edits, selection, and cursor
    pub redo_stack: Vec<super::undo::BufferState>,
    /// Word wrap enabled
    pub word_wrap: bool,
    /// Gutter width in pixels (calculated from config)
    pub gutter_width: i32,
    /// Diagnostics messages (row, message, kind)
    pub diagnostics: Vec<(usize, String, String)>,
    /// Debug mode flag for verbose logging
    pub debug_mode: bool,
    /// Optional redraw callback for GTK UI
    #[allow(clippy::type_complexity)]
    pub redraw_callback: Option<Box<dyn Fn()>>,
    /// Cursor runtime state (blinking, visibility, etc)
    pub cursor_state: crate::corelogic::cursor::CursorState,
    /// Mouse interaction state for selection
    pub mouse_state: MouseState,
    /// Desired visual X position for vertical cursor movement (in pixels)
    pub desired_x: Option<f64>,
    /// Last mouse X position for debugging
    pub last_mouse_x: f64,
    /// Last mouse Y position for debugging
    pub last_mouse_y: f64,
}

impl EditorBuffer {
    /// Dispatches an editor action using the CommandDispatcher
    pub fn handle_editor_action(&mut self, action: crate::keybinds::EditorAction) {
        use crate::corelogic::dispatcher::{CommandParams, CommandDispatcher};
        
        // Create a temporary dispatcher for this operation
        let mut dispatcher = CommandDispatcher::new();
        if self.debug_mode {
            dispatcher.set_debug_mode(true);
        }
        
        // Convert EditorAction to dispatcher action and parameters
        let params = match action {
            // Actions that need text parameter
            crate::keybinds::EditorAction::InsertText => {
                // This should be called with actual text, but for now we'll handle it separately
                CommandParams::None
            },
            // Actions that need file path parameter  
            crate::keybinds::EditorAction::OpenFile | 
            crate::keybinds::EditorAction::SaveFile => {
                // These should be called with actual file paths
                CommandParams::None  
            },
            // Actions that need position parameter
            crate::keybinds::EditorAction::AddCursor => {
                // This should be called with actual position
                CommandParams::None
            },
            // Most actions don't need parameters
            _ => CommandParams::None,
        };

        // Execute the action via the dispatcher
        if let Err(e) = dispatcher.execute(self, action, params) {
            if self.debug_mode {
                println!("[ERROR] Failed to execute action {:?}: {}", action, e);
            }
        }
    }

    /// Insert text at cursor position (for text input)
    pub fn handle_text_input(&mut self, text: &str) {
        use crate::corelogic::dispatcher::{CommandParams, CommandDispatcher};
        
        let mut dispatcher = CommandDispatcher::new();
        if self.debug_mode {
            dispatcher.set_debug_mode(true);
        }
        
        if let Err(e) = dispatcher.execute(
            self, 
            crate::keybinds::EditorAction::InsertText, 
            CommandParams::Text(text.to_string())
        ) {
            if self.debug_mode {
                println!("[ERROR] Failed to insert text '{}': {}", text, e);
            }
        }
    }

    /// Open file (for file operations)
    pub fn handle_open_file(&mut self, file_path: &str) {
        use crate::corelogic::dispatcher::{CommandParams, CommandDispatcher};
        
        let mut dispatcher = CommandDispatcher::new();
        if self.debug_mode {
            dispatcher.set_debug_mode(true);
        }
        
        if let Err(e) = dispatcher.execute(
            self, 
            crate::keybinds::EditorAction::OpenFile, 
            CommandParams::FilePath(file_path.to_string())
        ) {
            if self.debug_mode {
                println!("[ERROR] Failed to open file '{}': {}", file_path, e);
            }
        }
    }

    /// Save file (for file operations)
    pub fn handle_save_file(&mut self, file_path: &str) {
        use crate::corelogic::dispatcher::{CommandParams, CommandDispatcher};
        
        let mut dispatcher = CommandDispatcher::new();
        if self.debug_mode {
            dispatcher.set_debug_mode(true);
        }
        
        if let Err(e) = dispatcher.execute(
            self, 
            crate::keybinds::EditorAction::SaveFile, 
            CommandParams::FilePath(file_path.to_string())
        ) {
            if self.debug_mode {
                println!("[ERROR] Failed to save file '{}': {}", file_path, e);
            }
        }
    }
    /// Returns the unified line height for rendering (max of text font size, gutter font size, font_paragraph_spacing)
    pub fn unified_line_height(&self) -> f64 {
        let text_size = self.font_size();
        let gutter_size = self.config.gutter.font_size as f64;
        let line_height = self.font_paragraph_spacing();
        text_size.max(gutter_size).max(line_height)
    }
    /// Create a new empty EditorBuffer with default configuration
    pub fn new() -> Self {
        let config = crate::config::configuration::EditorConfig::default();
        EditorBuffer {
            cursor_state: crate::corelogic::cursor::CursorState::new(&config.cursor),
            config,
            lines: vec![
                "Welcome to RustEditorKit!".to_string(),
                "".to_string(),
                "This is a new Rust-based code editor.".to_string(),
                "You can type here and use arrow keys to move around.".to_string(),
                "".to_string(),
                "Features:".to_string(),
                "- Basic text editing".to_string(),
                "- Cursor movement".to_string(),
                "- Line numbers".to_string(),
                "- Configurable themes".to_string(),
            ],
            cursor: EditorCursor::new(0, 0),
            scroll_offset: 0,
            selection: None,
            multi_cursors: Vec::new(),
            multi_selections: Vec::new(),
            theme: ThemeSet::load_defaults().themes["base16-ocean.dark"].clone(),
            syntax_set: SyntaxSet::load_defaults_newlines(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            word_wrap: false,
            gutter_width: 0,
            diagnostics: Vec::new(),
            debug_mode: false,
            redraw_callback: None,
            mouse_state: MouseState::default(),
            desired_x: None,
            last_mouse_x: 0.0,
            last_mouse_y: 0.0,
        }
    }

    /// Request a redraw of the editor UI (calls the redraw_callback if set)
    pub fn request_redraw(&self) {
        if let Some(ref cb) = self.redraw_callback {
            println!("[DEBUG] EditorBuffer::redraw_callback executing");
            cb();
        }
        else {
            println!("[DEBUG] EditorBuffer::redraw_callback is None");
        }
        println!("[DEBUG] EditorBuffer::request_redraw called");
    }

    /// Apply settings from EditorConfig to this buffer
    pub fn apply_config(&mut self, config: crate::config::configuration::EditorConfig) {
        // Store the entire config - all settings are now accessed through this
        self.config = config.clone();
        // Update derived fields that need to be cached for performance
        self.gutter_width = config.gutter.ltr_width;
        // Update runtime cursor state from config
        self.update_cursor_state_from_config();
    }

    /// Get a reference to the font config
    pub fn font_config(&self) -> &crate::corelogic::font::FontConfig {
        &self.config.font
    }

    /// Get mutable reference to the font config
    pub fn font_config_mut(&mut self) -> &mut crate::corelogic::font::FontConfig {
        &mut self.config.font
    }

    /// Convenience: get font name
    pub fn font_name(&self) -> &str {
        self.config.font.font_name()
    }

    /// Convenience: get font size
    pub fn font_size(&self) -> f64 {
        self.config.font.font_size()
    }

    /// Convenience: get font color
    pub fn font_color(&self) -> &str {
        self.config.font.font_color()
    }

    /// Convenience: get line height
    pub fn font_paragraph_spacing(&self) -> f64 {
        self.config.font.font_paragraph_spacing()
    }

    /// Convenience: get character spacing
    pub fn font_character_spacing(&self) -> f64 {
        self.config.font.font_character_spacing()
    }

    /// Toggle A4 mode (stubbed for now)
    pub fn toggle_a4_mode(&mut self) {
        println!("[DEBUG] toggle_a4_mode called but not implemented yet");
    }
    
    /// Ensure cursor is visible by scrolling if needed
    /// This should be called after any cursor movement operation
    pub fn ensure_cursor_visible(&mut self) {
        if self.lines.is_empty() {
            self.scroll_offset = 0;
            return;
        }
        
        // If cursor is above the current scroll position, scroll up to show it
        if self.cursor.row < self.scroll_offset {
            self.scroll_offset = self.cursor.row;
            println!("[SCROLL DEBUG] Scrolled up to make cursor visible at row {}", self.cursor.row);
        } 
        // If cursor is below the current scroll position + visible lines, scroll down
        // Note: This is an approximation since we don't know the exact number of visible lines
        // We'll assume about 10 lines are visible at a time
        else {
            let estimated_visible_lines = 10; // This should ideally be calculated based on editor height
            let last_visible_line = self.scroll_offset + estimated_visible_lines;
            
            if self.cursor.row >= last_visible_line {
                // Scroll down to make cursor visible, keeping some context
                self.scroll_offset = self.cursor.row.saturating_sub(estimated_visible_lines - 2);
                self.scroll_offset = self.scroll_offset.min(self.lines.len().saturating_sub(1));
                println!("[SCROLL DEBUG] Scrolled down to make cursor visible at row {}", self.cursor.row);
            }
        }
    }
}
