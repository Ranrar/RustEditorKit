//! Pango-based text layout and rendering
use gtk4::cairo::Context;
use crate::corelogic::EditorBuffer;
use crate::render::layout::LayoutMetrics;
use crate::corelogic::gutter::parse_color;

/// Draws the text content layer
pub fn render_text_layer(rkit: &EditorBuffer, ctx: &Context, layout: &mut LayoutMetrics) {
    let font_cfg = &rkit.config.font;
    let char_spacing = font_cfg.font_character_spacing();
    let font_color = font_cfg.font_color();
    let (r, g, b, a) = parse_color(font_color);
    let paragraph_spacing = rkit.font_paragraph_spacing().max(0.0); // Clamp to zero
    
    // Measure each line's height with its own PangoLayout
    // This enables support for variable-height lines (emoji, CJK, etc.)
    let mut current_y = layout.top_offset;
    
    // Resize line_metrics if needed
    if layout.line_metrics.len() != rkit.lines.len() {
        layout.line_metrics.resize(rkit.lines.len(), crate::render::layout::LineMetrics {
            y_top: layout.top_offset,
            height: layout.line_height,
        });
    }
    
    // Calculate line heights and positions
    for (i, line) in rkit.lines.iter().enumerate() {
        let pango_layout = pangocairo::functions::create_layout(ctx);
        pango_layout.set_text(line);
        pango_layout.set_font_description(Some(&layout.text_metrics.font_desc));
        pango_layout.set_spacing(char_spacing as i32);
        
        // Measure line height (may vary with emoji, CJK characters, etc.)
        let line_height = pango_layout.extents().1.height() as f64 / gtk4::pango::SCALE as f64;
        let actual_height = line_height.max(layout.line_height); // Never go smaller than default line height
        
        layout.line_metrics[i] = crate::render::layout::LineMetrics {
            y_top: current_y,
            height: actual_height,
        };
        
        current_y += actual_height + paragraph_spacing;
    }
    
    // Precompute tabs for consistent tab stop alignment
    let tabs = layout.build_tab_array(&rkit.config);
    for (i, line) in rkit.lines.iter().enumerate() {
        let pango_layout = pangocairo::functions::create_layout(ctx);
        pango_layout.set_text(line);
        pango_layout.set_font_description(Some(&layout.text_metrics.font_desc));
        pango_layout.set_spacing(char_spacing as i32);
        pango_layout.set_tabs(Some(&tabs));
        let context = pango_layout.context();
        context.set_round_glyph_positions(true);
        
        // Use line_metrics for positioning
        let line_metric = &layout.line_metrics[i];
        let y_baseline = line_metric.y_top + layout.text_metrics.baseline_offset;
        
        ctx.set_source_rgba(r, g, b, a);
        ctx.move_to(layout.text_left_offset, y_baseline);
        pangocairo::functions::show_layout(ctx, &pango_layout);
    }
    
    // Visual debug: Draw horizontal lines at each line boundary
    let widget_width = match ctx.clip_extents() {
        Ok((_, _, width, _)) => width,
        Err(_) => 800.0, // Fallback width if we can't get clip extents
    };
    for (i, line_info) in layout.line_metrics.iter().enumerate() {
        let y = line_info.y_top;

        // Draw a semi-transparent red horizontal line at each line top
        ctx.set_source_rgba(1.0, 0.0, 0.0, 0.3);
        ctx.set_line_width(1.0);
        ctx.move_to(0.0, y);
        ctx.line_to(widget_width, y);
        let _ = ctx.stroke();

        // Draw line number next to the line for easier debugging
        ctx.set_source_rgba(0.0, 0.5, 1.0, 0.8);
        ctx.move_to(5.0, y + 10.0);
        ctx.set_font_size(9.0);
        let _ = ctx.show_text(&format!("L{}", i));
        let _ = ctx.stroke();
    }
    
    // Draw visual marker at last mouse position
    let mouse_debug_info = crate::render::pointer::MouseDebugInfo {
        x: rkit.last_mouse_x,
        y: rkit.last_mouse_y,
        is_valid: true,
    };
    crate::render::pointer::render_mouse_marker(ctx, &mouse_debug_info);
    
    // Draw cursor at correct y_offset for the current line
    let cursor_row = rkit.cursor.row;
    if cursor_row < layout.line_metrics.len() {
        let line = &rkit.lines[cursor_row];
        let pango_layout = pangocairo::functions::create_layout(ctx);
        pango_layout.set_text(line);
        pango_layout.set_font_description(Some(&layout.text_metrics.font_desc));
        pango_layout.set_spacing(char_spacing as i32);
        pango_layout.set_tabs(Some(&tabs));
        let context = pango_layout.context();
        context.set_round_glyph_positions(true);
        
        // Use line_metrics for positioning the cursor
        let y_line = layout.line_metrics[cursor_row].y_top;
        
        crate::render::cursor::render_cursor_layer(rkit, ctx, &pango_layout, layout, y_line);
    }
}
