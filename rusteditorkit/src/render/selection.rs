
/// Configuration for rendering text selection in the editor.
#[derive(Debug, Clone)]
pub struct SelectionRenderConfig {
	/// Whether selection rendering is enabled.
	pub selection_toggle: bool,
	/// Background color for selected text (hex string, e.g. "#0050aa").
	pub selection_bg_color: String,
	/// Opacity for selection background (0.0 - 1.0).
	pub selection_opacity: f32,
	/// Text color for selected text (hex string, e.g. "#ffffff").
	pub selection_text_color: String,
}


impl Default for SelectionRenderConfig {
	fn default() -> Self {
		Self {
			selection_toggle: true,
			selection_bg_color: "#0050aa".to_string(),
			selection_opacity: 0.3,
			selection_text_color: "#ffffff".to_string(),
		}
	}
}

use gtk4::cairo::Context;
use crate::corelogic::EditorBuffer;
use crate::render::layout::LayoutMetrics;
use crate::corelogic::gutter::parse_color;

/// Renders the selection layer (background and text color) for selected text.
/// Only draws if selection_toggle is true and a selection exists.
pub fn render_selection_layer(buf: &EditorBuffer, ctx: &Context, layout: &LayoutMetrics, _width: i32) {
	let sel_cfg = &buf.config.selection;
	if !sel_cfg.selection_toggle || buf.selection.is_none() {
		return;
	}
	let selection = buf.selection.as_ref().unwrap();
	let (bg_r, bg_g, bg_b, _) = parse_color(&sel_cfg.selection_bg_color);
	let opacity = sel_cfg.selection_opacity as f64;

	// Draw selection background for each selected line
	let (start_row, start_col, end_row, end_col) = selection.range();
	for row in start_row..=end_row {
		// Use line_metrics for variable-height lines
		if row >= layout.line_metrics.len() {
			continue; // Skip if the row is beyond our known metrics
		}
		
		let line_metric = &layout.line_metrics[row];
		let y_line = line_metric.y_top;
		let y_baseline = y_line + layout.text_metrics.baseline_offset;
		let line_height = line_metric.height;
		let line: &String = &buf.lines[row];
		let line_len = line.chars().count();
		let sel_start = if row == start_row { start_col } else { 0 };
		let sel_end = if row == end_row { end_col } else { line_len };
		if sel_start >= sel_end || sel_end > line_len { continue; }

		// Calculate x positions using Pango layout
		let pango_layout = pangocairo::functions::create_layout(ctx);
		pango_layout.set_text(line);
		pango_layout.set_font_description(Some(&layout.text_metrics.font_desc));
		pango_layout.set_spacing(buf.config.font.font_character_spacing() as i32);
		// Ensure same tab stops as text rendering
		let tabs = layout.build_tab_array(&buf.config);
		pango_layout.set_tabs(Some(&tabs));
		let start_x = layout.text_left_offset + pango_layout.index_to_pos(sel_start as i32).x() as f64 / gtk4::pango::SCALE as f64;
		let end_x = layout.text_left_offset + pango_layout.index_to_pos(sel_end as i32).x() as f64 / gtk4::pango::SCALE as f64;

		ctx.set_source_rgba(bg_r, bg_g, bg_b, opacity);
		ctx.rectangle(start_x, y_baseline, end_x - start_x, line_height);
		ctx.fill().unwrap();
	}
	// Text color is handled in text layer (see render_text_layer)
}
