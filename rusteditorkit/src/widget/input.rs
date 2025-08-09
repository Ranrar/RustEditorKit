//! Input handling for the EditorWidget
//! Handles keyboard input, cursor movement, and text insertion

use gtk4::gdk::Key;
use gtk4::pango;
use crate::corelogic::EditorBuffer;
use crate::render::layout::LayoutMetrics;
use crate::corelogic::selection::Selection;

/// Input handling utilities for the editor
pub struct InputHandler;

impl InputHandler {
    /// Ensure cursor is always valid after buffer changes
    pub fn ensure_cursor_valid(buf: &mut EditorBuffer) {
        if buf.lines.is_empty() {
            buf.cursor.row = 0;
            buf.cursor.col = 0;
        } else {
            if buf.cursor.row >= buf.lines.len() {
                buf.cursor.row = buf.lines.len() - 1;
            }
            buf.cursor.col = buf.cursor.col.min(buf.lines[buf.cursor.row].len());
        }
    }

    /// Create a Pango layout configured exactly like rendering for the given line
    fn configured_layout(
        buf: &EditorBuffer,
        metrics: &LayoutMetrics,
        pango_ctx: &pango::Context,
        row: usize,
    ) -> pango::Layout {
        let font_cfg = &buf.config.font;
        let font_string = format!("{} {}", font_cfg.font_name(), font_cfg.font_size());
        let font_desc = pango::FontDescription::from_string(&font_string);
        let layout = pango::Layout::new(pango_ctx);
        layout.set_font_description(Some(&font_desc));
        let text = buf.lines.get(row).cloned().unwrap_or_default();
        layout.set_text(&text);
    // Match rendering config
    layout.set_spacing(buf.config.font.font_character_spacing() as i32);
        layout.context().set_round_glyph_positions(true);
        let tabs = metrics.build_tab_array(&buf.config);
        layout.set_tabs(Some(&tabs));
        layout
    }

    /// Convert current (row,col) to pango byte index and trailing using the configured layout
    fn current_pango_index(buf: &EditorBuffer, _layout: &pango::Layout) -> (i32, i32) {
        let row = buf.cursor.row.min(buf.lines.len().saturating_sub(1));
        let line = buf.lines.get(row).cloned().unwrap_or_default();
        let mut byte_index = 0i32;
        let mut acc = 0usize;
        for (bi, _) in line.char_indices() {
            if acc == buf.cursor.col { byte_index = bi as i32; break; }
            acc += 1;
            byte_index = bi as i32; // last seen; adjusted below
        }
        if buf.cursor.col >= line.chars().count() {
            byte_index = line.len() as i32;
        }
        // Determine trailing: place caret on leading edge of the grapheme
        let trailing = 0;
        (byte_index, trailing)
    }

    /// Visual-run-aware horizontal move using Pango::Layout::move_cursor_visually
    fn move_horiz_visually(
        buf: &mut EditorBuffer,
        metrics: &LayoutMetrics,
        pango_ctx: &pango::Context,
        dir: i32,
    ) {
        if buf.lines.is_empty() { buf.cursor.row = 0; buf.cursor.col = 0; return; }
        let row = buf.cursor.row.min(buf.lines.len().saturating_sub(1));
        let layout = Self::configured_layout(buf, metrics, pango_ctx, row);
        let (byte_index, trailing) = Self::current_pango_index(buf, &layout);
        let strong = true;
        let (new_index, new_trailing) = layout.move_cursor_visually(strong, byte_index, trailing, dir);
        // Map new_index/new_trailing back to character column
        let line = buf.lines.get(row).cloned().unwrap_or_default();
        let mut col = 0usize;
        let mut set = false;
        for (i, (bi, _)) in line.char_indices().enumerate() {
            if bi as i32 == new_index { col = i; set = true; break; }
            if (bi as i32) > new_index { col = i; set = true; break; }
            col = i;
        }
        if !set { col = line.chars().count(); }
        if new_trailing > 0 { col = (col + 1).min(line.chars().count()); }
        buf.cursor.row = row;
        buf.cursor.col = col.min(line.chars().count());
        // Update desired_x based on new visual position
        let rect = layout.index_to_pos(new_index);
        buf.desired_x = Some(metrics.text_left_offset + rect.x() as f64 / pango::SCALE as f64);
    }

    /// Move cursor left (visual order, bidi-aware)
    pub fn move_cursor_left(buf: &mut EditorBuffer, metrics: &LayoutMetrics, pango_ctx: &pango::Context) {
        Self::move_horiz_visually(buf, metrics, pango_ctx, -1);
        Self::ensure_cursor_valid(buf);
        buf.request_redraw();
    }

    /// Move cursor right (visual order, bidi-aware)
    pub fn move_cursor_right(buf: &mut EditorBuffer, metrics: &LayoutMetrics, pango_ctx: &pango::Context) {
        Self::move_horiz_visually(buf, metrics, pango_ctx, 1);
        Self::ensure_cursor_valid(buf);
        buf.request_redraw();
    }

    /// Helper: after moving the cursor, start/extend selection from prev_cursor to current
    fn update_selection_after_move(buf: &mut EditorBuffer, prev_row: usize, prev_col: usize) {
        let new_cursor = buf.cursor;
        if (prev_row, prev_col) != (new_cursor.row, new_cursor.col) {
            match &mut buf.selection {
                Some(sel) => {
                    sel.end_row = new_cursor.row;
                    sel.end_col = new_cursor.col;
                    sel.clamp_to_buffer(&buf.lines);
                    if sel.start_row == sel.end_row && sel.start_col == sel.end_col {
                        buf.selection = None;
                    }
                }
                None => {
                    let mut sel = Selection::new(prev_row, prev_col);
                    sel.set(prev_row, prev_col, new_cursor.row, new_cursor.col);
                    buf.selection = Some(sel);
                }
            }
        }
    }

    /// Select left (visual order, bidi-aware)
    pub fn select_left(buf: &mut EditorBuffer, metrics: &LayoutMetrics, pango_ctx: &pango::Context) {
        let prev = buf.cursor;
        Self::move_horiz_visually(buf, metrics, pango_ctx, -1);
        Self::ensure_cursor_valid(buf);
        Self::update_selection_after_move(buf, prev.row, prev.col);
        buf.request_redraw();
    }

    /// Select right (visual order, bidi-aware)
    pub fn select_right(buf: &mut EditorBuffer, metrics: &LayoutMetrics, pango_ctx: &pango::Context) {
        let prev = buf.cursor;
        Self::move_horiz_visually(buf, metrics, pango_ctx, 1);
        Self::ensure_cursor_valid(buf);
        Self::update_selection_after_move(buf, prev.row, prev.col);
        buf.request_redraw();
    }

    /// Select up: keep desired X and extend selection
    pub fn select_up(buf: &mut EditorBuffer, metrics: &LayoutMetrics, pango_ctx: &pango::Context) {
        let prev = buf.cursor;
        Self::move_cursor_up(buf, metrics, pango_ctx);
        Self::update_selection_after_move(buf, prev.row, prev.col);
        buf.request_redraw();
    }

    /// Select down: keep desired X and extend selection
    pub fn select_down(buf: &mut EditorBuffer, metrics: &LayoutMetrics, pango_ctx: &pango::Context) {
        let prev = buf.cursor;
        Self::move_cursor_down(buf, metrics, pango_ctx);
        Self::update_selection_after_move(buf, prev.row, prev.col);
        buf.request_redraw();
    }

    /// Move cursor up: keep desired visual X using index_to_pos and xy_to_index
    pub fn move_cursor_up(buf: &mut EditorBuffer, metrics: &LayoutMetrics, pango_ctx: &pango::Context) {
        if buf.lines.is_empty() { buf.cursor.row = 0; buf.cursor.col = 0; buf.request_redraw(); return; }
        if buf.cursor.row == 0 { buf.request_redraw(); return; }
        // Establish desired_x from current caret if absent
        let cur_layout = Self::configured_layout(buf, metrics, pango_ctx, buf.cursor.row);
    let (byte_index, _trailing) = Self::current_pango_index(buf, &cur_layout);
        let caret_rect = cur_layout.index_to_pos(byte_index);
        let current_x = metrics.text_left_offset + caret_rect.x() as f64 / pango::SCALE as f64;
        let desired_x = buf.desired_x.unwrap_or(current_x);
        // Move to previous row
        let new_row = buf.cursor.row - 1;
        let new_layout = Self::configured_layout(buf, metrics, pango_ctx, new_row);
        const PANGO_SCALE_F: f64 = pango::SCALE as f64;
        let x_pango = ((desired_x - metrics.text_left_offset) * PANGO_SCALE_F) as i32;
        let (success, new_index, new_trailing) = new_layout.xy_to_index(x_pango, 0);
        let index = if success { new_index } else { 0 };
        let line = buf.lines.get(new_row).cloned().unwrap_or_default();
        // Map byte index to column
        let mut col = 0usize; let mut set=false;
        for (i, (bi, _)) in line.char_indices().enumerate() {
            if bi as i32 == index { col = i; set = true; break; }
            if (bi as i32) > index { col = i; set = true; break; }
            col = i;
        }
        if !set { col = line.chars().count(); }
        if new_trailing > 0 { col = (col + 1).min(line.chars().count()); }
        buf.cursor.row = new_row;
        buf.cursor.col = col;
        buf.desired_x = Some(desired_x);
        Self::ensure_cursor_valid(buf);
        buf.request_redraw();
    }

    /// Move cursor down: keep desired visual X using index_to_pos and xy_to_index
    pub fn move_cursor_down(buf: &mut EditorBuffer, metrics: &LayoutMetrics, pango_ctx: &pango::Context) {
        if buf.lines.is_empty() { buf.cursor.row = 0; buf.cursor.col = 0; buf.request_redraw(); return; }
        if buf.cursor.row + 1 >= buf.lines.len() { buf.request_redraw(); return; }
        // Establish desired_x from current caret if absent
        let cur_layout = Self::configured_layout(buf, metrics, pango_ctx, buf.cursor.row);
        let (byte_index, _trailing) = Self::current_pango_index(buf, &cur_layout);
        let caret_rect = cur_layout.index_to_pos(byte_index);
        let current_x = metrics.text_left_offset + caret_rect.x() as f64 / pango::SCALE as f64;
        let desired_x = buf.desired_x.unwrap_or(current_x);
        // Move to next row
        let new_row = buf.cursor.row + 1;
        let new_layout = Self::configured_layout(buf, metrics, pango_ctx, new_row);
        const PANGO_SCALE_F: f64 = pango::SCALE as f64;
        let x_pango = ((desired_x - metrics.text_left_offset) * PANGO_SCALE_F) as i32;
        let (success, new_index, new_trailing) = new_layout.xy_to_index(x_pango, 0);
        let index = if success { new_index } else { 0 };
        let line = buf.lines.get(new_row).cloned().unwrap_or_default();
        // Map byte index to column
        let mut col = 0usize; let mut set=false;
        for (i, (bi, _)) in line.char_indices().enumerate() {
            if bi as i32 == index { col = i; set = true; break; }
            if (bi as i32) > index { col = i; set = true; break; }
            col = i;
        }
        if !set { col = line.chars().count(); }
        if new_trailing > 0 { col = (col + 1).min(line.chars().count()); }
        buf.cursor.row = new_row;
        buf.cursor.col = col;
        buf.desired_x = Some(desired_x);
        Self::ensure_cursor_valid(buf);
        buf.request_redraw();
    }

    /// Insert a character at the cursor
    pub fn insert_char(buf: &mut EditorBuffer, c: char) {
        if buf.lines.is_empty() {
            buf.lines.push(String::new());
            buf.cursor.row = 0;
            buf.cursor.col = 0;
        }
        let row = buf.cursor.row;
        let col = buf.cursor.col;
        if row < buf.lines.len() {
            let line = &mut buf.lines[row];
            let byte_idx = line.char_indices().nth(col).map(|(i, _)| i).unwrap_or(line.len());
            line.insert(byte_idx, c);
            buf.cursor.col += 1;
        }
        Self::ensure_cursor_valid(buf);
        buf.request_redraw();
    }

    /// Insert newline at cursor
    pub fn insert_newline(buf: &mut EditorBuffer) {
        if buf.lines.is_empty() {
            buf.lines.push(String::new());
            buf.cursor.row = 0;
            buf.cursor.col = 0;
        }
        let row = buf.cursor.row;
        let col = buf.cursor.col;
        if row < buf.lines.len() {
            let rest = buf.lines[row].split_off(col);
            buf.lines.insert(row + 1, rest);
            buf.cursor.row += 1;
            buf.cursor.col = 0;
        }
        Self::ensure_cursor_valid(buf);
        buf.request_redraw();
    }

    /// Handle a key event
    pub fn handle_key_event(buf: &mut EditorBuffer, keyval: Key, metrics: &LayoutMetrics, pango_ctx: &pango::Context) {
        match keyval {
            Key::Left => Self::move_cursor_left(buf, metrics, pango_ctx),
            Key::Right => Self::move_cursor_right(buf, metrics, pango_ctx),
            Key::Up => Self::move_cursor_up(buf, metrics, pango_ctx),
            Key::Down => Self::move_cursor_down(buf, metrics, pango_ctx),
            Key::BackSpace => {
                buf.backspace();
                buf.request_redraw();
            },
            Key::Delete => {
                buf.delete();
                buf.request_redraw();
            },
            Key::Return => Self::insert_newline(buf),
            _ => {
                if let Some(c) = keyval.to_unicode() {
                    Self::insert_char(buf, c);
                }
            }
        }
    }
}
