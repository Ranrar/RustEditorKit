use serde::Deserialize;
use crate::corelogic::gutter::GutterConfig;
use crate::corelogic::font::FontConfig;

/// Configuration for text selection appearance
#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct SelectionConfig {
    pub selection_toggle: bool,
    pub selection_bg_color: String,
    pub selection_opacity: f64,
    pub selection_text_color: String,
}

impl Default for SelectionConfig {
    fn default() -> Self {
        Self {
            selection_toggle: true,
            selection_bg_color: "#0050aa".to_string(),
            selection_opacity: 0.3,
            selection_text_color: "#ffffff".to_string(),
        }
    }
}

/// Configuration for editor appearance and behavior. All fields are RON-serializable.
use crate::corelogic::cursor::CursorConfig;

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct EditorConfig {
    pub font: FontConfig,
    pub cursor: CursorConfig,
    pub editor_bg_color: String,
    pub gutter: GutterConfig,
    pub selection: SelectionConfig,
    /// Number of spaces that a tab represents (used to compute tab stops)
    pub tab_width_spaces: u32,

    // Search and whitespace guides
    pub search_match_color: String,
    pub whitespace_guide_color: String,
    pub show_whitespace_guides: bool,

    // Feature toggles
    pub syntax_highlighting: bool,
    pub auto_indent_enabled: bool,
    pub comment_enabled: bool,

    // Margins and spacing
    pub margin_left: f64,
    pub margin_right: f64,
    pub margin_top: f64,
    pub margin_bottom: f64,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            font: FontConfig::default(),
            cursor: CursorConfig::default(),
            editor_bg_color: "#f8f8ff".to_string(),
            gutter: GutterConfig {
                toggle: true,
                ltr_width: 50,
                padding: 4,
                bg_color: "#1e1e1e".to_string(),
                border: crate::corelogic::gutter::GutterBorderConfig {
                    toggle: true,
                    color: "#444".to_string(),
                    width: 1,
                    style: "solid".to_string(),
                },
                line_numbers: crate::corelogic::gutter::GutterLineNumbersConfig {
                    color: "#aaa".to_string(),
                    ltr_width: 35,
                    align: "right".to_string(),
                    padding: 6,
                    hover_color: "#fff".to_string(),
                    active_clickable: false,
                },
                font_size: 11,
                font_weight: "normal".to_string(),
                active_line: crate::corelogic::gutter::GutterActiveLineConfig {
                    line_number_color: "#fff".to_string(),
                    highlight_toggle: true,
                    highlight_color: "#333".to_string(),
                    highlight_opacity: 0.6,
                },
                markers: crate::corelogic::gutter::GutterMarkersConfig {
                    enabled: true,
                    icon_char: "●".to_string(),
                    icon_path: "res/icons/breakpoint.svg".to_string(),
                    icon_size: 12,
                    color: "#e06c75".to_string(),
                    hover_color: "#ffb3b3".to_string(),
                    spacing: 4,
                    position: "left".to_string(),
                },
            },
            selection: SelectionConfig::default(),
            tab_width_spaces: 4,

            // Search and whitespace guides
            search_match_color: "#ffff99".to_string(),
            whitespace_guide_color: "#e0e0e0".to_string(),
            show_whitespace_guides: false,

            // Feature toggles
            syntax_highlighting: true,
            auto_indent_enabled: true,
            comment_enabled: true,

            // Margins and spacing
            margin_left: 8.0,
            margin_right: 8.0,
            margin_top: 4.0,
            margin_bottom: 4.0,
        }
    }
}

// Modular API for EditorConfig
impl EditorConfig {
    pub fn set_font(&mut self, font: FontConfig) { self.font = font; }
    pub fn font(&self) -> &FontConfig { &self.font }
    pub fn set_editor_bg_color(&mut self, color: &str) { self.editor_bg_color = color.to_string(); }
    pub fn editor_bg_color(&self) -> &str { &self.editor_bg_color }
    pub fn set_gutter(&mut self, gutter: GutterConfig) { self.gutter = gutter; }
    pub fn gutter(&self) -> &GutterConfig { &self.gutter }
    pub fn set_search_match_color(&mut self, c: &str) { self.search_match_color = c.to_string(); }
    pub fn search_match_color(&self) -> &str { &self.search_match_color }
    pub fn set_whitespace_guide_color(&mut self, c: &str) { self.whitespace_guide_color = c.to_string(); }
    pub fn whitespace_guide_color(&self) -> &str { &self.whitespace_guide_color }
    pub fn set_show_whitespace_guides(&mut self, v: bool) { self.show_whitespace_guides = v; }
    pub fn show_whitespace_guides(&self) -> bool { self.show_whitespace_guides }
    pub fn set_syntax_highlighting(&mut self, v: bool) { self.syntax_highlighting = v; }
    pub fn syntax_highlighting(&self) -> bool { self.syntax_highlighting }
    pub fn set_auto_indent_enabled(&mut self, v: bool) { self.auto_indent_enabled = v; }
    pub fn auto_indent_enabled(&self) -> bool { self.auto_indent_enabled }
    pub fn set_comment_enabled(&mut self, v: bool) { self.comment_enabled = v; }
    pub fn comment_enabled(&self) -> bool { self.comment_enabled }
    pub fn set_tab_width_spaces(&mut self, v: u32) { self.tab_width_spaces = v; }
    pub fn tab_width_spaces(&self) -> u32 { self.tab_width_spaces }
    pub fn set_margin_left(&mut self, v: f64) { self.margin_left = v; }
    pub fn margin_left(&self) -> f64 { self.margin_left }
    pub fn set_margin_right(&mut self, v: f64) { self.margin_right = v; }
    pub fn margin_right(&self) -> f64 { self.margin_right }
    pub fn set_margin_top(&mut self, v: f64) { self.margin_top = v; }
    pub fn margin_top(&self) -> f64 { self.margin_top }
    pub fn set_margin_bottom(&mut self, v: f64) { self.margin_bottom = v; }
    pub fn margin_bottom(&self) -> f64 { self.margin_bottom }
    
    // Selection configuration methods
    pub fn set_selection(&mut self, selection: SelectionConfig) { self.selection = selection; }
    pub fn selection(&self) -> &SelectionConfig { &self.selection }
    pub fn set_selection_toggle(&mut self, v: bool) { self.selection.selection_toggle = v; }
    pub fn selection_toggle(&self) -> bool { self.selection.selection_toggle }
    pub fn set_selection_bg_color(&mut self, color: &str) { self.selection.selection_bg_color = color.to_string(); }
    pub fn selection_bg_color(&self) -> &str { &self.selection.selection_bg_color }
    pub fn set_selection_opacity(&mut self, v: f64) { self.selection.selection_opacity = v; }
    pub fn selection_opacity(&self) -> f64 { self.selection.selection_opacity }
    pub fn set_selection_text_color(&mut self, color: &str) { self.selection.selection_text_color = color.to_string(); }
    pub fn selection_text_color(&self) -> &str { &self.selection.selection_text_color }
}
