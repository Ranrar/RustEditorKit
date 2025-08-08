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
    let row = buf.cursor.row.min(buf.lines.len().saturating_sub(1));
    
    // Skip if the row is beyond our known metrics
    if row >= layout.line_metrics.len() {
        return;
    }
    
    // Use line_metrics for variable-height lines
    let line_metric = &layout.line_metrics[row];
    let y_line = line_metric.y_top;
    let y_baseline = y_line + layout.text_metrics.baseline_offset;
    let line_height = line_metric.height;
    
    if gutter_config.active_line.highlight_toggle {
        let highlight_color = &gutter_config.active_line.highlight_color;
        let (hr, hg, hb, _) = crate::corelogic::gutter::parse_color(highlight_color);
        ctx.set_source_rgba(hr, hg, hb, gutter_config.active_line.highlight_opacity);
        ctx.rectangle(0.0, y_baseline, width as f64, line_height);
        ctx.fill().unwrap();
    }
}
