//! Highlight rendering logic for the editor
//! This module draws the active line highlight using unified line height and config

use crate::corelogic::EditorBuffer;
use crate::render::layout::LayoutMetrics;
use cairo::Context;

/// Draws the active line highlight if enabled in config
///
/// # Arguments
/// * `buf` - EditorBuffer reference
/// * `ctx` - Cairo context
/// * `layout` - LayoutMetrics for positioning
/// * `width` - Total editor width
pub fn render_highlight_layer(buf: &EditorBuffer, ctx: &Context, layout: &LayoutMetrics, width: i32) {
    let gutter_config = buf.config.gutter();
    let line_height = layout.line_height;
    let row = buf.cursor.row.min(buf.lines.len().saturating_sub(1));
    // Use unified y-offsets for perfect alignment
    let y_offsets = buf.line_y_offsets(line_height, buf.config.font.font_paragraph_spacing(), layout.top_offset);
    let y_line = y_offsets.get(row).copied().unwrap_or(layout.top_offset);
    let y_baseline = y_line + layout.text_metrics.baseline_offset;
    if gutter_config.active_line.highlight_toggle {
        let highlight_color = &gutter_config.active_line.highlight_color;
        let (hr, hg, hb, _) = crate::corelogic::gutter::parse_color(highlight_color);
        ctx.set_source_rgba(hr, hg, hb, gutter_config.active_line.highlight_opacity);
        ctx.rectangle(0.0, y_baseline, width as f64, line_height);
        ctx.fill().unwrap();
    }
}
