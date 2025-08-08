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
    let left_margin = layout.text_left_offset;
    
    // Use line_metrics for variable-height lines
    // Find row by comparing against inclusive bands [y_top, y_top + height) for each line
    let mut row = 0;
    let mut found = false;
    
    println!("[POINTER DEBUG] Mouse position: ({:.1}, {:.1})", x, y);
    
    // If we have any line metrics available, use them for accurate hit-testing
    if !layout.line_metrics.is_empty() {
        let max_lines = layout.line_metrics.len().min(buffer.lines.len());
        
        for i in 0..max_lines {
            let line_metric = &layout.line_metrics[i];
            let band_top = line_metric.y_top;
            
            // Calculate the band's bottom position
            // If this is the last line, use the line height
            // Otherwise, use the next line's y_top as the boundary
            let band_bottom = if i + 1 < max_lines {
                layout.line_metrics[i+1].y_top
            } else {
                band_top + line_metric.height + buffer.config.font.font_paragraph_spacing()
            };
            
            println!("[POINTER DEBUG] Line {}: y=[{:.1}-{:.1}], height={:.1}", 
                     i, band_top, band_bottom, line_metric.height);
            
            // Check if y is within this band
            if y >= band_top && y < band_bottom {
                row = i;
                found = true;
                println!("[POINTER DEBUG] Found match at row {}", i);
                break;
            }
        }
        
        // If we still haven't found a match but have valid line metrics
        if !found && !layout.line_metrics.is_empty() {
            // If we're above the first line, use first line
            if y < layout.line_metrics[0].y_top {
                row = 0;
                found = true;
                println!("[POINTER DEBUG] Above first line, using row 0");
            }
            // If we're below the last line, use last line
            else if y >= layout.line_metrics.last().unwrap().y_top + layout.line_metrics.last().unwrap().height {
                row = (layout.line_metrics.len() - 1).min(buffer.lines.len() - 1);
                found = true;
                println!("[POINTER DEBUG] Below last line, using row {}", row);
            }
        }
    }
    
    if !found {
        // Check if click is below the last visible line
        if !layout.line_metrics.is_empty() {
            let max_lines = layout.line_metrics.len().min(buffer.lines.len());
            
            // If we have metrics and the click is below the last visible line
            if max_lines > 0 && y > layout.line_metrics[max_lines - 1].y_top + layout.line_metrics[max_lines - 1].height {
                // Calculate which line this should be based on global coordinates
                let last_visible_line = max_lines - 1;
                let last_bottom = layout.line_metrics[last_visible_line].y_top + layout.line_metrics[last_visible_line].height;
                let distance_below_last = y - last_bottom;
                
                // Use the average line height of visible lines to estimate which line was clicked
                let mut avg_line_height = layout.line_height;
                if max_lines > 1 {
                    let total_height = layout.line_metrics[max_lines - 1].y_top + layout.line_metrics[max_lines - 1].height - 
                                      layout.line_metrics[0].y_top;
                    avg_line_height = total_height / max_lines as f64;
                }
                
                // Estimate which line beyond the visible area was clicked
                let estimated_lines_below = (distance_below_last / avg_line_height).floor() as usize;
                let target_line = (last_visible_line + 1 + estimated_lines_below).min(buffer.lines.len() - 1);
                
                println!("[POINTER DEBUG] Click below visible area. Estimated line: {}", target_line);
                
                // We can't directly update buffer.scroll_offset here since buffer is an immutable reference
                // We'll return the line index and let the caller handle scrolling
                
                row = target_line;
                found = true;
            }
        }
        
        // If still not found, choose nearest visible row center as before
        if !found {
            let mut nearest_row = 0;
            let mut min_dist = f64::MAX;
            
            if !layout.line_metrics.is_empty() {
                // Use the available line metrics, but don't go beyond buffer.lines.len()
                let max_lines = layout.line_metrics.len().min(buffer.lines.len());
                for i in 0..max_lines {
                    let line_metric = &layout.line_metrics[i];
                    let center = line_metric.y_top + (line_metric.height / 2.0);
                    let dist = (y - center).abs();
                    if dist < min_dist {
                        min_dist = dist;
                        nearest_row = i;
                    }
                }
            } else {
                // Fallback to constant line height if line_metrics isn't valid
                let line_height = layout.line_height;
                for i in 0..buffer.lines.len() {
                    let top = layout.top_offset + (i as f64 * line_height);
                    let center = top + (line_height / 2.0);
                    let dist = (y - center).abs();
                    if dist < min_dist {
                        min_dist = dist;
                        nearest_row = i;
                    }
                }
            }
            row = nearest_row;
            println!("[POINTER DEBUG] Using nearest row {}", row);
        }
    }
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
    
    // Handle auto-scrolling when clicking beyond visible area
    let last_visible_line = if !layout.line_metrics.is_empty() {
        layout.line_metrics.len() - 1
    } else {
        0
    };
    
    // If the click resulted in a row beyond the last visible line, adjust scroll offset
    if row > last_visible_line && row < buffer.lines.len() {
        let lines_to_scroll = row - last_visible_line;
        buffer.scroll_offset = buffer.scroll_offset.saturating_add(lines_to_scroll);
        buffer.scroll_offset = buffer.scroll_offset.min(buffer.lines.len().saturating_sub(1));
        println!("[SCROLL DEBUG] Auto-scrolled {} lines down to show clicked line", lines_to_scroll);
    }
    
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
    buffer.ensure_cursor_visible();
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
