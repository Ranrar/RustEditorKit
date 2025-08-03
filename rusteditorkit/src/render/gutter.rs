//! Handles line numbers, breakpoints, and fold markers
use gtk4::cairo::Context;
use crate::corelogic::EditorBuffer;
use crate::render::layout::LayoutMetrics;
use crate::corelogic::gutter::render_gutter;

/// Draws the gutter (line numbers, markers, etc.)
pub fn render_gutter_layer(rkit: &EditorBuffer, ctx: &Context, layout: &LayoutMetrics, height: i32) {
    let gutter_cfg = &rkit.config.gutter;
    if !gutter_cfg.toggle {
        return;
    }
    // Use the same font as the text area for alignment
    render_gutter(
        rkit,
        ctx,
        height,
        gutter_cfg,
        rkit.lines.len(),
        rkit.cursor.row,
        layout.line_height,
        layout.top_offset,
        layout,
    );
}
