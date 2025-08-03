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
    for (i, line) in rkit.lines.iter().enumerate() {
        let pango_layout = pangocairo::functions::create_layout(ctx);
        pango_layout.set_text(line);
        pango_layout.set_font_description(Some(&layout.text_metrics.font_desc));
        pango_layout.set_spacing(char_spacing as i32);
        pango_layout.set_height((layout.line_height * pango::SCALE as f64) as i32);
        let context = pango_layout.context();
        context.set_round_glyph_positions(true);
        let y_line = layout.top_offset + i as f64 * layout.line_height;
        let y_baseline = y_line + layout.text_metrics.baseline_offset;
        ctx.set_source_rgba(r, g, b, a);
        ctx.move_to(layout.text_left_offset, y_baseline);
        pangocairo::functions::show_layout(ctx, &pango_layout);
        if i == rkit.cursor.row {
            crate::render::cursor::render_cursor_layer(rkit, ctx, &pango_layout, layout, y_line);
        }
    }
}
