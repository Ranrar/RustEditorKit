//! Renders the caret, handles blinking and movement
use gtk4::cairo::Context;
use gtk4::pango;
use crate::corelogic::EditorBuffer;
use crate::render::layout::LayoutMetrics;
use crate::corelogic::gutter::parse_color;

/// Draws the cursor with exact alignment to text baseline
pub fn render_cursor_layer(
    rkit: &EditorBuffer,
    ctx: &Context,
    text_layout: &pango::Layout,
    layout: &LayoutMetrics,
    y_line: f64
) {
    let cursor_cfg = &rkit.config.cursor;
    let cursor_state = &rkit.cursor_state;
    if !cursor_state.is_cursor_visible() {
        return;
    }
    let (r, g, b, a) = parse_color(&cursor_cfg.cursor_color);
    ctx.set_source_rgba(r, g, b, a);
    let col = rkit.cursor.col.min(rkit.lines[rkit.cursor.row].chars().count());
    // Unicode fallback: Pango handles multi-byte, so just document
    let cursor_rect = text_layout.index_to_pos(col as i32);
    let cursor_x = layout.text_left_offset + (cursor_rect.x() as f64) / (pango::SCALE as f64);
    let y_baseline = y_line + layout.text_metrics.baseline_offset;
    let cursor_y = y_baseline + cursor_cfg.cursor_padding_y;
    let text_height = layout.text_metrics.height;
    // let line_height = layout.line_height;
    // println!("text_height = {}, gutter_height = {}, line_height = {}", text_height, layout.gutter_metrics.height, line_height);
    match cursor_cfg.cursor_type.as_str() {
        "bar" => {
            ctx.rectangle(
                cursor_x - cursor_cfg.cursor_padding_x, 
                cursor_y, 
                cursor_cfg.cursor_thickness, 
                text_height
            );
        },
        "block" => {
            if cursor_cfg.cursor_roundness > 0.0 {
                let _ = ctx.save();
                ctx.arc(
                    cursor_x + (rkit.config.font.font_size() / 2.0),
                    cursor_y + (text_height / 2.0),
                    rkit.config.font.font_size() / 2.0,
                    0.0,
                    std::f64::consts::PI * 2.0
                );
                let _ = ctx.restore();
            } else {
                ctx.rectangle(
                    cursor_x - cursor_cfg.cursor_padding_x,
                    cursor_y,
                    rkit.config.font.font_size() + 2.0 * cursor_cfg.cursor_padding_x,
                    text_height
                );
            }
        },
        "underline" => {
            let underline_y = cursor_y + text_height - cursor_cfg.cursor_thickness;
            ctx.rectangle(
                cursor_x - cursor_cfg.cursor_padding_x,
                underline_y,
                rkit.config.font.font_size() + 2.0 * cursor_cfg.cursor_padding_x,
                cursor_cfg.cursor_thickness
            );
        },
        _ => {
            ctx.rectangle(
                cursor_x - cursor_cfg.cursor_padding_x,
                cursor_y,
                cursor_cfg.cursor_thickness,
                text_height
            );
        }
    }
    ctx.fill().unwrap_or(());
}
