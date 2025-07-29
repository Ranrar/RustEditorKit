//! EditorWidget rendering logic
//! Draws text, gutter, and cursor for EditorBuffer

use gtk4::cairo::Context;
use crate::core::EditorBuffer;
use gtk4::pango;
use pangocairo;

/// Render the editor buffer, gutter, and cursor
pub fn render_editor(buf: &EditorBuffer, ctx: &Context, width: i32, height: i32) {
    // Fill background
    ctx.set_source_rgb(0.95, 0.95, 0.98);
    ctx.paint().unwrap_or(());

    // Gutter settings
    let gutter_width = 60.0;
    ctx.set_source_rgb(0.90, 0.90, 0.93); // gutter bg
    ctx.rectangle(0.0, 0.0, gutter_width, height as f64);
    ctx.fill().unwrap_or(());

    // Draw line numbers and text
    let pango_ctx = pangocairo::functions::create_context(ctx);
    for (i, line) in buf.lines.iter().enumerate() {
        let y = 30.0 + i as f64 * 22.0;
        let line_num = format!("{:>3}", i + 1);
        let layout = pango::Layout::new(&pango_ctx);
        layout.set_text(&line_num);
        layout.set_font_description(Some(&pango::FontDescription::from_string("Fira Mono 12")));
        ctx.set_source_rgb(0.5, 0.5, 0.7); // line number color
        ctx.move_to(8.0, y);
        pangocairo::functions::show_layout(ctx, &layout);

        // Draw text line
        let layout = pango::Layout::new(&pango_ctx);
        layout.set_text(line);
        layout.set_font_description(Some(&pango::FontDescription::from_string("Fira Mono 12")));
        ctx.set_source_rgb(0.15, 0.15, 0.18); // text color
        ctx.move_to(gutter_width + 8.0, y);
        pangocairo::functions::show_layout(ctx, &layout);

        // Draw cursor if on this line
        if i == buf.cursor.row {
            let pos = layout.index_to_pos(buf.cursor.col as i32);
            let cursor_x = gutter_width + 8.0 + pos.x() as f64 / pango::SCALE as f64;
            ctx.set_source_rgb(0.2, 0.6, 0.9); // cursor color
            ctx.rectangle(cursor_x, y, 2.0, 18.0);
            ctx.fill().unwrap_or(());
        }
    }
}
