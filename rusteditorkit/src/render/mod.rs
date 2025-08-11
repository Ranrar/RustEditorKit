//! Renderer public interface and submodule re-exports
use gtk4::cairo::Context;
use crate::corelogic::buffer::EditorBuffer;

/// Main rendering entry point with layered architecture
pub fn render_editor(rkit: &EditorBuffer, ctx: &Context, width: i32, height: i32) {
    let mut layout = LayoutMetrics::calculate(rkit, ctx);
    background::render_background_layer(rkit, ctx, width, height);
    // Text layer must be rendered before other layers as it calculates the line metrics
    text::render_text_layer(rkit, ctx, &mut layout);
    gutter::render_gutter_layer(rkit, ctx, &layout, height);
    highlight::render_highlight_layer(rkit, ctx, &layout, width);
    selection::render_selection_layer(rkit, ctx, &layout, width);
}

pub mod background;
pub mod gutter;
pub mod text;
pub mod cursor;
pub mod layout;
pub mod theme;
pub mod cache;
pub mod invalidate;
pub mod highlight;
pub mod selection;

// Publicly re-export main types and entry points
pub use background::render_background_layer;
pub use gutter::render_gutter_layer;
pub use text::render_text_layer;
pub use cursor::render_cursor_layer;
pub use layout::{LayoutMetrics, FontMetrics};
pub use selection::render_selection_layer;