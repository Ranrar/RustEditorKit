//! Draws the editor background and page canvas (A4, US Letter, etc.)
use gtk4::cairo::Context;
use crate::corelogic::EditorBuffer;
use crate::corelogic::gutter::parse_color;

pub fn render_background_layer(rkit: &EditorBuffer, ctx: &Context, width: i32, height: i32) {
    let bg_color = rkit.config.editor_bg_color();
    let (r, g, b, a) = parse_color(bg_color);
    ctx.set_source_rgba(r, g, b, a);
    ctx.rectangle(0.0, 0.0, width as f64, height as f64);
    ctx.fill().unwrap_or(());
    // TODO: Add A4/US Letter page boundary rendering
}
