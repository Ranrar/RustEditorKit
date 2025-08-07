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

pub fn screen_to_buffer_position(
    buffer: &EditorBuffer,
    x: f64,
    y: f64,
    layout: &LayoutMetrics,
    pango_ctx: &Context,
    font_desc: &FontDescription,
) -> (usize, usize) {
    let line_height = layout.line_height;
    let left_margin = layout.text_left_offset;
    let top_margin = layout.top_offset;
    // ...moved logic from editing.rs...
    let mut row = 0;
    let mut found = false;
    for i in 0..buffer.lines.len() {
        let line_top = top_margin + line_height * i as f64;
        let line_bottom = line_top + line_height;
        if y >= line_top && y < line_bottom {
            row = i;
            found = true;
            break;
        }
    }
    if !found {
        let mut nearest_row = 0;
        let mut min_dist = f64::MAX;
        for i in 0..buffer.lines.len() {
            let center = top_margin + line_height * i as f64 + line_height / 2.0;
            let dist = (y - center).abs();
            if dist < min_dist {
                min_dist = dist;
                nearest_row = i;
            }
        }
        row = nearest_row;
    }
    let line = if row < buffer.lines.len() { &buffer.lines[row] } else { "" };
    let pango_layout = gtk4::pango::Layout::new(pango_ctx);
    pango_layout.set_text(line);
    pango_layout.set_font_description(Some(font_desc));
    const PANGO_SCALE: f64 = 1024.0;
    let x_pango = ((x - left_margin) * PANGO_SCALE) as i32;
    let (success, byte_index, _trailing) = pango_layout.xy_to_index(x_pango, 0);
    let byte_index = if success { byte_index } else { 0 };
    // Clamp byte_index to valid range
    let byte_index_usize = byte_index.min(line.len() as i32).max(0) as usize;
    // Find the character index corresponding to the byte index
    let col = line.char_indices().enumerate().find_map(|(i, (b, _))| if b == byte_index_usize { Some(i) } else { None })
        .unwrap_or_else(|| line.chars().count());
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
    let line_height = layout.line_height;
    let top_margin = layout.top_offset;
    let mut snapped_y = y;
    let mut found = false;
    for i in 0..buffer.lines.len() {
        let line_top = top_margin + line_height * i as f64;
        let line_bottom = line_top + line_height;
        if y >= line_top && y < line_bottom {
            snapped_y = line_top;
            found = true;
            break;
        }
    }
    if !found {
        let mut min_dist = f64::MAX;
        for i in 0..buffer.lines.len() {
            let line_top = top_margin + line_height * i as f64;
            let dist = (y - line_top).abs();
            if dist < min_dist {
                min_dist = dist;
                snapped_y = line_top;
            }
        }
    }
    let (row, col) = screen_to_buffer_position(buffer, x, snapped_y, layout, pango_ctx, font_desc);
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
