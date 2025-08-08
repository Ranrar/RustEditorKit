//! Pango-based text layout and rendering
use gtk4::cairo::Context;
use gtk4::pango;
use crate::corelogic::EditorBuffer;
use crate::render::layout::LayoutMetrics;
use crate::corelogic::gutter::parse_color;

/// Draws the text content layer
pub fn render_text_layer(rkit: &EditorBuffer, ctx: &Context, layout: &LayoutMetrics) {
    let font_cfg = &rkit.config.font;
    let char_spacing = font_cfg.font_character_spacing();
    let font_color = font_cfg.font_color();
    let (r, g, b, a) = parse_color(font_color);
    let paragraph_spacing = rkit.font_paragraph_spacing().max(0.0); // Clamp to zero
    let mut line_y_offsets = rkit.line_y_offsets(
        layout.line_height,
        paragraph_spacing,
        layout.top_offset,
    );
    // Apply vertical scroll offset (in pixels based on line_height spacing assumption)
    let scroll_px = (rkit.scroll_offset as f64) * layout.line_height;
    for y in &mut line_y_offsets { *y -= scroll_px; }
    // Precompute tabs for consistent tab stop alignment
    let tabs = layout.build_tab_array(&rkit.config);
    for (i, line) in rkit.lines.iter().enumerate() {
        let pango_layout = pangocairo::functions::create_layout(ctx);
        pango_layout.set_text(line);
        pango_layout.set_font_description(Some(&layout.text_metrics.font_desc));
        pango_layout.set_spacing(char_spacing as i32);
        pango_layout.set_height((layout.line_height * pango::SCALE as f64) as i32);
        pango_layout.set_tabs(Some(&tabs));
        let context = pango_layout.context();
        context.set_round_glyph_positions(true);
        let y_baseline = line_y_offsets[i] + layout.text_metrics.baseline_offset;
        ctx.set_source_rgba(r, g, b, a);
        ctx.move_to(layout.text_left_offset, y_baseline);
        pangocairo::functions::show_layout(ctx, &pango_layout);
    }
    // Draw cursor at correct y_offset for the current line
    let cursor_row = rkit.cursor.row;
    if cursor_row < line_y_offsets.len() {
        let line = &rkit.lines[cursor_row];
        let pango_layout = pangocairo::functions::create_layout(ctx);
        pango_layout.set_text(line);
        pango_layout.set_font_description(Some(&layout.text_metrics.font_desc));
        pango_layout.set_spacing(char_spacing as i32);
        pango_layout.set_height((layout.line_height * pango::SCALE as f64) as i32);
        pango_layout.set_tabs(Some(&tabs));
        let context = pango_layout.context();
        context.set_round_glyph_positions(true);
        crate::render::cursor::render_cursor_layer(rkit, ctx, &pango_layout, layout, line_y_offsets[cursor_row]);
    }
}
