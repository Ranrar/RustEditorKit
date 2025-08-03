//! Text selection rendering logic for the editor
//! This module draws text selection backgrounds using unified line height and selection config

use crate::corelogic::EditorBuffer;
use crate::render::layout::LayoutMetrics;
use crate::corelogic::gutter::parse_color;
use cairo::Context;

/// Draws text selection backgrounds if a selection exists
///
/// # Arguments
/// * `buf` - EditorBuffer reference containing selection state
/// * `ctx` - Cairo context for drawing
/// * `layout` - LayoutMetrics for positioning calculations
/// * `width` - Total editor width for full-line selections
pub fn render_selection_layer(buf: &EditorBuffer, ctx: &Context, layout: &LayoutMetrics, width: i32) {
    // Early return if no selection exists
    let selection = match &buf.selection {
        Some(sel) => sel,
        None => {
            println!("[SELECTION RENDER DEBUG] No selection to render");
            return;
        }
    };

    if !selection.is_active() {
        println!("[SELECTION RENDER DEBUG] Selection exists but is not active");
        return;
    }

    println!("[SELECTION RENDER DEBUG] Rendering selection: {:?}", selection);

    let selection_config = buf.config.selection();

    // Parse selection background color from config
    let bg_color = &selection_config.selection_bg_color;
    let (r, g, b, _) = parse_color(bg_color);
    let opacity = selection_config.selection_opacity;
    
    println!("[SELECTION RENDER DEBUG] Color: r={}, g={}, b={}, opacity={}", r, g, b, opacity);
    ctx.set_source_rgba(r, g, b, opacity);

    // Get normalized selection coordinates
    let ((start_row, start_col), (end_row, end_col)) = selection.normalized();
    println!("[SELECTION RENDER DEBUG] Normalized coords: start=({}, {}), end=({}, {})", start_row, start_col, end_row, end_col);

    // Handle single-line vs multi-line selections
    if start_row == end_row {
                // Single-line selection
        println!("[SELECTION RENDER DEBUG] Single-line selection");
        render_single_line_selection_coords(ctx, start_row, start_col, end_col, layout, buf);
    } else {
        // Multi-line selection
        render_multi_line_selection_coords(
            ctx, 
            start_row,
            start_col,
            end_row,
            end_col,
            layout, 
            buf,
            width
        );
    }
}

/// Renders selection on a single line
fn render_single_line_selection_coords(
    ctx: &Context,
    row: usize,
    start_col: usize,
    end_col: usize,
    layout: &LayoutMetrics,
    buf: &EditorBuffer,
) {
    println!("[SELECTION RENDER DEBUG] Single-line render: row={}, start_col={}, end_col={}", row, start_col, end_col);
    
    if row >= buf.lines.len() {
        println!("[SELECTION RENDER DEBUG] Row {} >= buffer lines {}, returning", row, buf.lines.len());
        return;
    }

    let line = &buf.lines[row];
    let start_col = start_col.min(line.chars().count());
    let end_col = end_col.min(line.chars().count());
    
    println!("[SELECTION RENDER DEBUG] Line: '{}', clamped start_col={}, end_col={}", line, start_col, end_col);
    
    // Calculate pixel positions for start and end columns
    let start_x = calculate_column_x_position(line, start_col, layout);
    let end_x = calculate_column_x_position(line, end_col, layout);
    
    let y_line = layout.top_offset + row as f64 * layout.line_height;
    let selection_width = end_x - start_x;
    
    println!("[SELECTION RENDER DEBUG] Positions: start_x={}, end_x={}, y_line={}, width={}", start_x, end_x, y_line, selection_width);
    
    if selection_width > 0.0 {
        ctx.rectangle(start_x, y_line, selection_width, layout.line_height);
        ctx.fill().unwrap();
        println!("[SELECTION RENDER DEBUG] Rectangle drawn and filled");
    } else {
        println!("[SELECTION RENDER DEBUG] Selection width <= 0, not drawing");
    }
}

/// Renders selection spanning multiple lines
fn render_multi_line_selection_coords(
    ctx: &Context,
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
    layout: &LayoutMetrics,
    buf: &EditorBuffer,
    width: i32,
) {
    let text_left_offset = layout.text_left_offset;
    let right_edge = width as f64;
    
    for row in start_row..=end_row {
        if row >= buf.lines.len() {
            break;
        }
        
        let line = &buf.lines[row];
        let y_line = layout.top_offset + row as f64 * layout.line_height;
        
        if row == start_row {
            // First line: from start_col to end of line
            let start_col = start_col.min(line.chars().count());
            let start_x = calculate_column_x_position(line, start_col, layout);
            let width = right_edge - start_x;
            
            if width > 0.0 {
                ctx.rectangle(start_x, y_line, width, layout.line_height);
                ctx.fill().unwrap();
            }
        } else if row == end_row {
            // Last line: from start of line to end_col
            let end_col = end_col.min(line.chars().count());
            let end_x = calculate_column_x_position(line, end_col, layout);
            let width = end_x - text_left_offset;
            
            if width > 0.0 {
                ctx.rectangle(text_left_offset, y_line, width, layout.line_height);
                ctx.fill().unwrap();
            }
        } else {
            // Middle lines: select entire line
            let line_width = right_edge - text_left_offset;
            ctx.rectangle(text_left_offset, y_line, line_width, layout.line_height);
            ctx.fill().unwrap();
        }
    }
}

/// Calculates the X pixel position for a given column in a line
/// This accounts for character width variations and Unicode characters
fn calculate_column_x_position(
    _line: &str,
    col: usize,
    layout: &LayoutMetrics,
) -> f64 {
    if col == 0 {
        return layout.text_left_offset;
    }

    // For now, use a simple approximation based on average character width
    // This should be replaced with proper Pango text measurement in production
    let char_width = layout.text_metrics.average_char_width;
    layout.text_left_offset + (col as f64 * char_width)
}
