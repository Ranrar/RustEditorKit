//! Pointer rendering and visual debugging for pointer interactions

use gtk4::cairo::Context;

/// Tracks last known mouse position for debugging
#[derive(Debug, Clone, Copy)]
pub struct MouseDebugInfo {
    pub x: f64,
    pub y: f64,
    pub is_valid: bool,
}

impl Default for MouseDebugInfo {
    fn default() -> Self {
        Self {
            x: -1.0,
            y: -1.0,
            is_valid: false,
        }
    }
}

/// Draws a marker at the last mouse click/drag position for visual debugging
pub fn render_mouse_marker(ctx: &Context, debug_info: &MouseDebugInfo) {
    if !debug_info.is_valid {
        return; // Skip invalid coordinates
    }
    
    let x = debug_info.x;
    let y = debug_info.y;
    
    // Draw a small green circle at the mouse click position
    ctx.set_source_rgba(0.0, 1.0, 0.0, 0.7);
    ctx.arc(x, y, 3.0, 0.0, 2.0 * std::f64::consts::PI);
    let _ = ctx.fill();
    
    // Draw crosshairs to make position more visible
    ctx.set_source_rgba(0.0, 0.7, 0.0, 0.5);
    ctx.set_line_width(0.5);
    ctx.move_to(x - 5.0, y);
    ctx.line_to(x + 5.0, y);
    ctx.move_to(x, y - 5.0);
    ctx.line_to(x, y + 5.0);
    let _ = ctx.stroke();
    
    // Draw coordinates label
    ctx.set_source_rgba(0.0, 0.7, 0.0, 0.8);
    ctx.move_to(x + 5.0, y - 5.0);
    ctx.set_font_size(9.0);
    let _ = ctx.show_text(&format!("({:.1},{:.1})", x, y));
}

/// Prints detailed debug info for a mouse click: position, line/word, and line band analysis
pub fn debug_mouse_click(
    x: f64,
    y: f64,
    row: usize,
    col: usize,
    clicked_word: &str,
    line_metrics: &[crate::render::layout::LineMetrics],
) {
    println!("[CURSOR CLICK] Position: ({:.1}, {:.1}), Line: {}, Column: {}, Word: \"{}\"", x, y, row, col, clicked_word);
    for (i, line_metric) in line_metrics.iter().enumerate() {
        let band_top = line_metric.y_top;
        let band_bottom = band_top + line_metric.height;
        let marker = if i == row { "→" } else { " " };
        let hit = if y >= band_top && y < band_bottom { "✓" } else { " " };
        println!("[LINE DEBUG] {}Line {}: y=[{:.1}-{:.1}], height={:.1} {}", marker, i, band_top, band_bottom, line_metric.height, hit);
    }
    if row < line_metrics.len() {
        let band_top = line_metrics[row].y_top;
        let band_bottom = band_top + line_metrics[row].height;
        if y >= band_top && y < band_bottom {
            println!("[LINE DEBUG] Click falls within line {} band ✓", row);
        } else {
            println!("[LINE DEBUG] Click does NOT fall within line {} band ✗", row);
        }
    }
}
