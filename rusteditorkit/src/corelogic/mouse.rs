//! Pointer logic: mouse state and core buffer functions

/// Mouse interaction state for selection handling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseState {
    Idle,
    Selecting { start_row: usize, start_col: usize },
    ExtendingSelection,
}

impl Default for MouseState {
    fn default() -> Self {
        MouseState::Idle
    }
}

use crate::corelogic::buffer::EditorBuffer;
use gtk4::pango::{Context, FontDescription};
use crate::render::layout::LayoutMetrics;
use crate::corelogic::line_height::unified_line_height;

pub fn screen_to_buffer_position(
    buffer: &EditorBuffer,
    x: f64,
    y: f64,
    layout: &LayoutMetrics,
    pango_ctx: &Context,
    font_desc: &FontDescription,
) -> (usize, usize) {
    println!("[DEBUG] Mouse click at x={}, y={}", x, y);
    println!("[DEBUG] Layout top_offset: {}", layout.top_offset);
    for (i, lm) in layout.line_metrics.iter().enumerate() {
        println!("[DEBUG] Line {}: y_top={}, height={}", i, lm.y_top, lm.height);
    }
    let left_margin = layout.text_left_offset;
    println!("[DEBUG] Mouse click at x={}, y={}", x, y);
    println!("[DEBUG] Layout top_offset: {}", layout.top_offset);
    for (i, lm) in layout.line_metrics.iter().enumerate() {
        println!("[DEBUG] Line {}: y_top={}, height={}", i, lm.y_top, lm.height);
    }
    
    // Use unified line height for row calculation
    let text_height = layout.text_metrics.height;
    let gutter_height = layout.gutter_metrics.height;
    let paragraph_spacing = buffer.config.font.font_paragraph_spacing();
    // Fallback: use text_height as glyph_height if not available
    let line_height = unified_line_height(text_height, gutter_height, text_height, paragraph_spacing);
    let row_calc = ((y - layout.top_offset) / line_height).floor() as isize;
    let row = row_calc.clamp(0, (buffer.lines.len() - 1) as isize) as usize;
    println!("[DEBUG] Direct row calculation: y={}, top_offset={}, line_height={}, row_calc={}, clamped_row={}", y, layout.top_offset, line_height, row_calc, row);
    let line = if row < buffer.lines.len() { &buffer.lines[row] } else { "" };
    let pango_layout = gtk4::pango::Layout::new(pango_ctx);
    pango_layout.set_text(line);
    pango_layout.set_font_description(Some(font_desc));
    // Match render settings
    pango_layout.set_spacing(buffer.config.font.font_character_spacing() as i32);
    let ctx = pango_layout.context();
    ctx.set_round_glyph_positions(true);
    // Apply same tab stops as rendering for correct hit-testing across tabs
    let tabs = layout.build_tab_array(&buffer.config);
    pango_layout.set_tabs(Some(&tabs));
    const PANGO_SCALE: f64 = 1024.0;
    let x_pango = ((x - left_margin) * PANGO_SCALE) as i32;
    let (success, byte_index, trailing) = pango_layout.xy_to_index(x_pango, 0);
    let byte_index = if success { byte_index } else { 0 };
    // Clamp byte_index to valid range
    let byte_index_usize = byte_index.min(line.len() as i32).max(0) as usize;
    // Find the character index corresponding to the byte index, then apply trailing
    let mut col = 0usize;
    let mut found_b = false;
    for (i, (b, _ch)) in line.char_indices().enumerate() {
        if b == byte_index_usize {
            col = i;
            found_b = true;
            break;
        }
        if b > byte_index_usize {
            col = i; // byte index lies between chars; snap to this char
            found_b = true;
            break;
        }
        col = i; // will end up last char if no break
    }
    if !found_b {
        // If we didn't find a matching/in-between byte, set to end
        col = line.chars().count();
    }
    if trailing > 0 {
        col = (col + 1).min(line.chars().count());
    }
    if row >= layout.line_metrics.len() {
        println!("[DEBUG] No line metrics for row {}, using Pango layout for column mapping at x={}, mapped col={}", row, x, col);
    }
    println!("[DEBUG] Mapped to row={}, col={}, line='{}'", row, col, line);
    // Clamp row and col to valid buffer bounds
    let row = row.min(buffer.lines.len().saturating_sub(1));
    let col = col.min(line.chars().count());
    (row, col)
}

pub fn handle_mouse_click(
    buffer: &mut EditorBuffer,
    x: f64,
    y: f64,
    shift_held: bool,
    layout: &LayoutMetrics,
    pango_ctx: &Context,
    font_desc: &FontDescription,
) {
    let (row, col) = screen_to_buffer_position(buffer, x, y, layout, pango_ctx, font_desc);
    println!("[DEBUG] handle_mouse_click: x={}, y={}, mapped row={}, col={}", x, y, row, col);
       
    // Process the click as before
    if shift_held && buffer.selection.is_some() {
        if let Some(sel) = &mut buffer.selection {
            sel.end_row = row;
            sel.end_col = col;
            sel.clamp_to_buffer(&buffer.lines);
        }
    } else {
        buffer.selection = None;
        buffer.cursor.row = row;
        buffer.cursor.col = col;
    }
    
    // Ensure cursor remains visible after any position change
    buffer.mouse_state = if shift_held {
        MouseState::ExtendingSelection
    } else {
        MouseState::Selecting { start_row: row, start_col: col }
    };
}

pub fn handle_mouse_drag(
    buffer: &mut EditorBuffer,
    x: f64,
    y: f64,
    layout: &LayoutMetrics,
    pango_ctx: &Context,
    font_desc: &FontDescription,
) {
    let (row, col) = screen_to_buffer_position(buffer, x, y, layout, pango_ctx, font_desc);
    match buffer.mouse_state {
        MouseState::Selecting { start_row, start_col } => {
            let mut sel = crate::corelogic::selection::Selection::new(start_row, start_col);
            sel.end_row = row;
            sel.end_col = col;
            sel.clamp_to_buffer(&buffer.lines);
            if sel.is_active() {
                buffer.selection = Some(sel);
            } else {
                buffer.selection = None;
            }
            buffer.cursor.row = row;
            buffer.cursor.col = col;
        },
        MouseState::ExtendingSelection => {
            if let Some(sel) = &mut buffer.selection {
                sel.end_row = row;
                sel.end_col = col;
                sel.clamp_to_buffer(&buffer.lines);
            }
            buffer.cursor.row = row;
            buffer.cursor.col = col;
        },
        MouseState::Idle => {
            buffer.mouse_state = MouseState::Selecting { start_row: row, start_col: col };
        }
    }
}

pub fn handle_mouse_release(buffer: &mut EditorBuffer) {
    buffer.mouse_state = MouseState::Idle;
}

pub fn handle_double_click(
    buffer: &mut EditorBuffer,
    x: f64,
    y: f64,
    layout: &LayoutMetrics,
    pango_ctx: &Context,
    font_desc: &FontDescription,
) {
    let (row, col) = screen_to_buffer_position(buffer, x, y, layout, pango_ctx, font_desc);
    if row < buffer.lines.len() {
        let line = &buffer.lines[row];
        let chars: Vec<char> = line.chars().collect();
        if col < chars.len() {
            let mut start_col = col;
            let mut end_col = col;
            while start_col > 0 && (chars[start_col - 1].is_alphanumeric() || chars[start_col - 1] == '_') {
                start_col -= 1;
            }
            while end_col < chars.len() && (chars[end_col].is_alphanumeric() || chars[end_col] == '_') {
                end_col += 1;
            }
            if start_col < end_col {
                let mut sel = crate::corelogic::selection::Selection::new(row, start_col);
                sel.end_row = row;
                sel.end_col = end_col;
                buffer.selection = Some(sel);
                buffer.cursor.row = row;
                buffer.cursor.col = end_col;
            }
        }
    }
    buffer.mouse_state = MouseState::Idle;
}

pub fn handle_triple_click(
    buffer: &mut EditorBuffer,
    x: f64,
    y: f64,
    layout: &LayoutMetrics,
    pango_ctx: &Context,
    font_desc: &FontDescription,
) {
    let (row, _) = screen_to_buffer_position(buffer, x, y, layout, pango_ctx, font_desc);
    if row < buffer.lines.len() {
        let mut sel = crate::corelogic::selection::Selection::new(row, 0);
        sel.end_row = row;
        sel.end_col = buffer.lines[row].chars().count();
        buffer.selection = Some(sel);
        buffer.cursor.row = row;
        buffer.cursor.col = buffer.lines[row].chars().count();
    }
    buffer.mouse_state = MouseState::Idle;
}
