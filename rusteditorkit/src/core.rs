
// --- Corelogic main types and functions ---
pub use crate::corelogic::buffer::*;
pub use crate::corelogic::font::*;
pub use crate::corelogic::cursor::*;
pub use crate::corelogic::gutter::*;
pub use crate::corelogic::undo::*;
pub use crate::corelogic::search::*;
pub use crate::corelogic::fileio::*;
pub use crate::corelogic::dispatcher::*;
// Layout is currently disabled in mod.rs, but can be re-exported if needed:
// pub use crate::corelogic::layout::*;

// --- Keybinds main types and functions ---
pub use crate::keybinds::editor_action::*;
#[cfg(target_os = "linux")]
pub use crate::keybinds::linux::*;
#[cfg(target_os = "windows")]
pub use crate::keybinds::win::*;
#[cfg(target_os = "macos")]
pub use crate::keybinds::mac::*;

impl LegacyEditorBuffer {
    pub fn move_to_line_start(&mut self) {
        println!("[DEBUG] move_to_line_start");
        self.cursor.col = 0;
    }

    pub fn move_to_line_end(&mut self) {
        println!("[DEBUG] move_to_line_end");
        if self.cursor.row < self.lines.len() {
            self.cursor.col = self.lines[self.cursor.row].len();
        }
    }


    pub fn select_left(&mut self) {
        // Start or extend selection to the left
        let prev_cursor = self.cursor;
        self.move_left();
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
                    let mut sel = crate::corelogic::selection::Selection::new(prev_cursor.row, prev_cursor.col);
                    sel.set(prev_cursor.row, prev_cursor.col, new_cursor.row, new_cursor.col);
                    self.selection = Some(sel);
                }
            }
        }
        println!("[DEBUG] select_left: {:?}", self.selection);
    }

    pub fn select_right(&mut self) {
        // Start or extend selection to the right
        let prev_cursor = self.cursor;
        self.move_right();
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
                    let mut sel = crate::corelogic::selection::Selection::new(prev_cursor.row, prev_cursor.col);
                    sel.set(prev_cursor.row, prev_cursor.col, new_cursor.row, new_cursor.col);
                    self.selection = Some(sel);
                }
            }
        }
        println!("[DEBUG] select_right: {:?}", self.selection);
    }

    pub fn select_up(&mut self) {
        // Start or extend selection up
        let prev_cursor = self.cursor;
        self.move_up();
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
                    let mut sel = crate::corelogic::selection::Selection::new(prev_cursor.row, prev_cursor.col);
                    sel.set(prev_cursor.row, prev_cursor.col, new_cursor.row, new_cursor.col);
                    self.selection = Some(sel);
                }
            }
        }
        println!("[DEBUG] select_up: {:?}", self.selection);
    }

    pub fn select_down(&mut self) {
        // Start or extend selection down
        let prev_cursor = self.cursor;
        self.move_down();
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
                    let mut sel = crate::corelogic::selection::Selection::new(prev_cursor.row, prev_cursor.col);
                    sel.set(prev_cursor.row, prev_cursor.col, new_cursor.row, new_cursor.col);
                    self.selection = Some(sel);
                }
            }
        }
        println!("[DEBUG] select_down: {:?}", self.selection);
    }
    /// Move cursor left (with bounds checking)
    pub fn move_left(&mut self) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        } else if self.cursor.row > 0 {
            self.cursor.row -= 1;
            self.cursor.col = self.lines[self.cursor.row].len();
        }
    }

    /// Move cursor right (with bounds checking)
    pub fn move_right(&mut self) {
        if self.cursor.col < self.lines[self.cursor.row].len() {
            self.cursor.col += 1;
        } else if self.cursor.row + 1 < self.lines.len() {
            self.cursor.row += 1;
            self.cursor.col = 0;
        }
    }

    /// Move cursor up (with bounds checking)
    pub fn move_up(&mut self) {
        if self.cursor.row > 0 {
            self.cursor.row -= 1;
            self.cursor.col = self.cursor.col.min(self.lines[self.cursor.row].len());
        }
    }

    /// Move cursor down (with bounds checking)
    pub fn move_down(&mut self) {
        if self.cursor.row + 1 < self.lines.len() {
            self.cursor.row += 1;
            self.cursor.col = self.cursor.col.min(self.lines[self.cursor.row].len());
        }
    }

    /// Move cursor to start of line
    pub fn move_home(&mut self) {
        self.cursor.col = 0;
    }

    /// Move cursor to end of line
    pub fn move_end(&mut self) {
        self.cursor.col = self.lines[self.cursor.row].len();
    }
}
/// Thread safety and async message passing example for GTK4 Rust
///
/// All GTK4 widget creation, updates, and signal handling must occur on the main thread.
/// For background/async work (e.g., Rayon, std::thread, async tasks), use message passing to communicate results back to the main thread.
/// Use glib::Sender and glib::MainContext::channel() to send messages from worker threads to the main thread.
/// Use glib::idle_add_local() to schedule closures on the main thread for UI updates.
/// Never touch GTK widgets directly from non-main threads.
/// For shared state, use Arc<Mutex<T>> or Arc<RwLock<T>> for data, but always update GTK widgets via main thread message passing.
/// Example pattern:
///
/// use glib::MainContext;
/// use std::sync::{Arc, Mutex};
/// use rayon::prelude::*;
///
/// let (sender, receiver) = MainContext::channel(glib::PRIORITY_DEFAULT);
///
/// // In worker thread or rayon pool:
/// rayon::spawn(move || {
///     let result = do_expensive_work();
///     sender.send(result).unwrap();
/// });
///
/// // In main thread:
/// receiver.attach(None, move |result| {
///     // Update GTK widgets here
///     update_ui(result);
///     glib::Continue(true)
/// });
///
/// For async/await, use glib::MainContext::spawn_local() to run futures on the main thread.
/// GTK4-rs Editor Skeleton with Gutter, Syntax Highlighting, and Advanced Features

/// The main buffer struct for the custom code editor.
/// Holds all text, cursor, selection, undo/redo, theme, and rendering state.
pub struct LegacyEditorBuffer {
    /// Modular config for all editor appearance and behavior
    pub config: crate::config::configuration::EditorConfig,
    /// Lines of text in the buffer
    pub lines: Vec<String>,
    /// Cursor position
    pub cursor: EditorCursor,
    /// Scroll offset for vertical scrolling
    pub scroll_offset: usize,
    /// Whether to highlight the current line
    pub highlight_line: bool,
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
    /// Word wrap enabled
    pub word_wrap: bool,
    /// Modular font configuration for all font properties
    pub font: crate::corelogic::font::FontConfig,
    /// Gutter width in pixels
    pub gutter_width: i32,
    /// Left margin in pixels
    pub margin_left: f64,
    /// Right margin in pixels
    pub margin_right: f64,
    /// Top margin in pixels
    pub margin_top: f64,
    /// Bottom margin in pixels
    pub margin_bottom: f64,
    /// Foreground color
    /// Background color
    pub editor_bg_color: String,
    /// Gutter color
    pub gutter_color: String,
    /// Line number color
    pub line_number_color: String,
    /// Selected line number color
    pub selected_line_number_color: String,
    /// Highlight color
    pub highlight_color: String,
    /// Syntax highlighting enabled
    pub syntax_highlighting: bool,
    /// Cursor color
    pub cursor_color: String,
    /// Markdown header color
    pub markdown_header_color: String,
    /// Markdown bold color
    pub markdown_bold_color: String,
    /// Markdown italic color
    pub markdown_italic_color: String,
    /// Markdown code color
    pub markdown_code_color: String,
    /// Markdown link color
    pub markdown_link_color: String,
    /// Markdown quote color
    pub markdown_quote_color: String,
    /// Markdown list color
    pub markdown_list_color: String,
    /// Markdown syntax coloring enabled
    pub markdown_syntax_coloring: bool,
    /// Error color for diagnostics
    pub error_color: String,
    /// Warning color for diagnostics
    pub warning_color: String,
    /// Diagnostics highlighting enabled
    pub diagnostics_highlighting: bool,
    /// Search match color
    pub search_match_color: String,
    /// Whitespace guide color
    pub whitespace_guide_color: String,
    /// Show whitespace guides
    pub show_whitespace_guides: bool,
    /// Active line background color
    pub active_line_bg_color: String,
    /// Show active line background
    pub show_active_line_bg: bool,
    /// Inactive line background color
    pub inactive_line_bg_color: String,
    /// Show inactive line background
    pub show_inactive_line_bg: bool,
    /// Debug mode enabled
    pub debug_mode: bool,
    /// Diagnostics messages (row, message, kind)
    pub diagnostics: Vec<(usize, String, String)>,
    /// A4 page mode enabled
    pub a4_mode: bool,
    /// Top margin in centimeters
    pub top_margin_cm: f64,
    /// Bottom margin in centimeters
    pub bottom_margin_cm: f64,
    /// Left margin in centimeters
    pub left_margin_cm: f64,
    /// Right margin in centimeters
    pub right_margin_cm: f64,
    /// Optional redraw callback for GTK UI
    #[allow(clippy::type_complexity)]
    pub redraw_callback: Option<Box<dyn Fn()>>,
}

use std::cell::RefCell;
use std::rc::Rc;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, DrawingArea};
use syntect::parsing::SyntaxSet;

impl LegacyEditorBuffer {
    /// Get a reference to the font config
    pub fn font_config(&self) -> &crate::corelogic::font::FontConfig {
        &self.font
    }

    /// Get mutable reference to the font config
    pub fn font_config_mut(&mut self) -> &mut crate::corelogic::font::FontConfig {
        &mut self.font
    }

    /// Convenience: get font name
    pub fn font_name(&self) -> &str {
        self.font.font_name()
    }

    /// Convenience: get font size
    pub fn font_size(&self) -> f64 {
        self.font.font_size()
    }

    /// Convenience: get font color
    pub fn font_color(&self) -> &str {
        self.font.font_color()
    }

    /// Convenience: get line height
    pub fn font_line_height(&self) -> f64 {
        self.font.font_line_height()
    }

    /// Convenience: get character spacing
    pub fn font_character_spacing(&self) -> f64 {
        self.font.font_character_spacing()
    }
    /// Request a redraw of the editor UI (calls the redraw_callback if set)
    pub fn request_redraw(&self) {
        if let Some(ref cb) = self.redraw_callback {
            cb();
        }
    }

    /// Move cursor up by one visible page (PgUp)
    pub fn move_page_up(&mut self, lines_per_page: usize) {
        if self.cursor.row > lines_per_page {
            self.cursor.row -= lines_per_page;
        } else {
            self.cursor.row = 0;
        }
        self.cursor.col = self.cursor.col.min(self.lines[self.cursor.row].len());
    }
    /// Apply settings from EditorConfig to this buffer
    pub fn apply_config(&mut self, config: crate::config::configuration::EditorConfig) {
        // Copy all config fields to buffer fields for runtime use
        self.config = config.clone();
        self.font = config.font.clone();
        self.margin_left = config.margin_left;
        self.margin_right = config.margin_right;
        self.margin_top = config.margin_top;
        self.margin_bottom = config.margin_bottom;

        self.editor_bg_color = config.editor_bg_color.clone();
        self.syntax_highlighting = config.syntax_highlighting;
        self.search_match_color = config.search_match_color.clone();
        self.whitespace_guide_color = config.whitespace_guide_color.clone();
        self.show_whitespace_guides = config.show_whitespace_guides;
        // Gutter and nested config fields are now accessed via self.config.gutter
    }
    /// Move cursor down by one visible page (PgDn)
    pub fn move_page_down(&mut self, lines_per_page: usize) {
        let max_row = self.lines.len().saturating_sub(1);
        self.cursor.row = (self.cursor.row + lines_per_page).min(max_row);
        self.cursor.col = self.cursor.col.min(self.lines[self.cursor.row].len());
    }
    /// Cut selected text (removes and returns it)
    pub fn cut(&mut self) -> String {
        if let Some(sel) = &mut self.selection {
            sel.clamp_to_buffer(&self.lines);
            let ((row_start, col_start), (row_end, col_end)) = sel.normalized();
            println!("[DEBUG] cut: selection=({},{}) to ({},{})", row_start, col_start, row_end, col_end);
            if row_start == row_end && row_start < self.lines.len() && col_end > col_start {
                let cut = self.lines[row_start][col_start..col_end].to_string();
                self.lines[row_start].replace_range(col_start..col_end, "");
                // Clamp cursor
                self.cursor.row = row_start.min(self.lines.len().saturating_sub(1));
                self.cursor.col = col_start.min(self.lines.get(self.cursor.row).map(|l| l.len()).unwrap_or(0));
                // Reset selection
                self.selection = None;
                // Ensure at least one line
                if self.lines.is_empty() { self.lines.push(String::new()); }
                return cut;
            } else if row_start < self.lines.len() && row_end < self.lines.len() {
                let mut result = String::new();
                for row in row_start..=row_end {
                    let line = &self.lines[row];
                    if row == row_start && row == row_end {
                        result.push_str(&line[col_start..col_end]);
                    } else if row == row_start {
                        result.push_str(&line[col_start..]);
                        result.push('\n');
                    } else if row == row_end {
                        result.push_str(&line[..col_end]);
                    } else {
                        result.push_str(line);
                        result.push('\n');
                    }
                }
                // Remove the selected text from the buffer
                self.lines[row_start].replace_range(col_start.., "");
                self.lines[row_end].replace_range(..col_end, "");
                if row_end > row_start + 1 {
                    self.lines.drain(row_start + 1..row_end);
                }
                // Remove empty lines if needed
                if self.lines.is_empty() { self.lines.push(String::new()); }
                // Clamp cursor
                self.cursor.row = row_start.min(self.lines.len().saturating_sub(1));
                self.cursor.col = col_start.min(self.lines.get(self.cursor.row).map(|l| l.len()).unwrap_or(0));
                // Reset selection
                self.selection = None;
                return result;
            }
        }
        // Always ensure at least one line
        if self.lines.is_empty() { self.lines.push(String::new()); }
        self.cursor.row = 0;
        self.cursor.col = 0;
        self.selection = None;
        String::new()
    }

    /// Set markdown syntax colors
    fn set_markdown_colors(&mut self, header: String, bold: String, italic: String, code: String, link: String, quote: String, list: String) {
        self.markdown_header_color = header;
        self.markdown_bold_color = bold;
        self.markdown_italic_color = italic;
        self.markdown_code_color = code;
        self.markdown_link_color = link;
        self.markdown_quote_color = quote;
        self.markdown_list_color = list;
    }

    /// Toggle markdown syntax coloring
    fn toggle_markdown_syntax_coloring(&mut self) {
        self.markdown_syntax_coloring = !self.markdown_syntax_coloring;
    }

    /// Set error and warning colors
    fn set_error_warning_colors(&mut self, error: String, warning: String) {
        self.error_color = error;
        self.warning_color = warning;
    }

    /// Toggle diagnostics highlighting
    fn toggle_diagnostics_highlighting(&mut self) {
        self.diagnostics_highlighting = !self.diagnostics_highlighting;
    }

    /// Set search match highlight color
    fn set_search_match_color(&mut self, color: String) {
        self.search_match_color = color;
    }

    /// Set whitespace/indent guide color
    pub fn set_whitespace_guide_color(&mut self, color: String) {
        self.whitespace_guide_color = color;
    }

    /// Toggle whitespace/indent guides
    pub fn toggle_whitespace_guides(&mut self) {
        self.show_whitespace_guides = !self.show_whitespace_guides;
    }

    /// Toggle syntax highlighting on/off
    fn toggle_syntax_highlighting(&mut self) {
        self.syntax_highlighting = !self.syntax_highlighting;
    }

    /// Theme switching stub (light/dark/custom)
    fn switch_theme(&mut self, _theme_name: &str) {
        // Stub: apply theme (requires integration with GTK CSS)
    }

    /// Print/export stub (to PDF/HTML/Markdown)
    fn print_export(&self, _format: &str, _path: &str) {
        // Stub: export buffer to file (PDF/HTML/Markdown)
    }

    /// Status bar update stub (show cursor position, diagnostics, etc.)
    fn update_status_bar(&self) {
        // Stub: update status bar (requires integration with GTK UI)
    }

    /// IME (Input Method Editor) support stub
    fn ime_support(&self) {
        // Stub: handle IME events (requires GTK integration)
    }

    /// Accessibility support stub
    fn accessibility_support(&self) {
        // Stub: expose accessibility info (requires GTK integration)
    }

    /// Performance optimization stub
    fn optimize_performance(&mut self) {
        // Stub: optimize buffer and rendering for large files
    }

}

fn main() {
    let app = Application::builder()
        .application_id("com.example.RustEditorKit")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("RustEditorKit")
            .default_width(800)
            .default_height(600)
            .build();

        let rkit = Rc::new(RefCell::new(LegacyEditorBuffer::new()));
        let drawing_area = DrawingArea::new();
        drawing_area.set_draw_func(move |_, ctx, width, height| {
            let rkit = rkit.borrow();
            // TODO: Replace this with the new widget system
            // super::render::render_editor(&rkit, ctx, width, height);
        });

        window.set_child(Some(&drawing_area));
        window.show();
    });

    app.run();
}

impl LegacyEditorBuffer {
    pub fn new() -> Self {
        use syntect::parsing::SyntaxSet;
        use syntect::highlighting::ThemeSet;
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme = ThemeSet::load_defaults().themes["InspiredGitHub"].clone();
        LegacyEditorBuffer {
            lines: Vec::new(),
            cursor: EditorCursor::new(0, 0),
            scroll_offset: 0,
            highlight_line: true,
            selection: None,
            multi_cursors: Vec::new(),
            multi_selections: Vec::new(),
            theme,
            syntax_set,
            word_wrap: true,
            font: crate::corelogic::font::FontConfig::default(),
            editor_bg_color: "#ffffff".to_string(),
            gutter_color: "#e0e0e0".to_string(),
            line_number_color: "#888888".to_string(),
            selected_line_number_color: "#0055aa".to_string(),
            highlight_color: "#cceeff".to_string(),
            syntax_highlighting: true,
            cursor_color: "#0055aa".to_string(),
            markdown_header_color: "#0055aa".to_string(),
            markdown_bold_color: "#222222".to_string(),
            markdown_italic_color: "#444444".to_string(),
            markdown_code_color: "#333333".to_string(),
            markdown_link_color: "#0088cc".to_string(),
            markdown_quote_color: "#888888".to_string(),
            markdown_list_color: "#0055aa".to_string(),
            markdown_syntax_coloring: true,
            error_color: "#ff3333".to_string(),
            warning_color: "#ffaa00".to_string(),
            diagnostics_highlighting: true,
            search_match_color: "#ffff99".to_string(),
            whitespace_guide_color: "#cccccc".to_string(),
            show_whitespace_guides: false,
            active_line_bg_color: "#f0f8ff".to_string(),
            show_active_line_bg: true,
            inactive_line_bg_color: "#f8f8f8".to_string(),
            show_inactive_line_bg: false,
            debug_mode: false,
            diagnostics: Vec::new(),
            a4_mode: false,
            top_margin_cm: 2.0,
            bottom_margin_cm: 2.0,
            left_margin_cm: 2.0,
            right_margin_cm: 2.0,
            gutter_width: 60,
            margin_left: 8.0,
            margin_right: 8.0,
            margin_top: 4.0,
            margin_bottom: 4.0,
            redraw_callback: None,
            config: crate::config::configuration::EditorConfig::default(),
        }
    }
}