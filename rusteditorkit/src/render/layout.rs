//! Handles geometry: line heights, spacing, padding, alignment
use gtk4::cairo::Context;
use gtk4::pango;
use crate::corelogic::EditorBuffer;
// unified_line_height is only used in per-line calculation below

/// Stores metrics for a specific line including its position and height
#[derive(Debug, Clone)]
pub struct LineMetrics {
    /// Y-coordinate of the top of the line
    pub y_top: f64,
    /// Height of this particular line
    pub height: f64,
}

#[derive(Debug, Clone)]
pub struct FontMetrics {
    pub font_desc: pango::FontDescription,
    pub baseline: f64,
    pub height: f64,
    pub baseline_offset: f64,
    pub average_char_width: f64,
}

#[derive(Debug, Clone)]
pub struct LayoutMetrics {
    pub line_height: f64,
    pub text_metrics: FontMetrics,
    pub gutter_metrics: FontMetrics,
    pub text_left_offset: f64,
    pub top_offset: f64,
    /// Per-line metrics for variable-height lines
    pub line_metrics: Vec<LineMetrics>,
}

impl FontMetrics {
    pub fn calculate(ctx: &Context, font_desc: &pango::FontDescription) -> Self {
        let layout = pangocairo::functions::create_layout(ctx);
        layout.set_font_description(Some(font_desc));
        layout.set_text("Hg");
        let baseline = layout.baseline() as f64 / pango::SCALE as f64;
        let height = layout.extents().1.height() as f64 / pango::SCALE as f64;
        
        // Calculate average character width using a representative sample
        layout.set_text("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
        let char_sample_width = layout.extents().1.width() as f64 / pango::SCALE as f64;
        let average_char_width = char_sample_width / 62.0; // 26 + 26 + 10 chars
        
        Self {
            font_desc: font_desc.clone(),
            baseline,
            height,
            baseline_offset: 0.0,
            average_char_width,
        }
    }
}

impl LayoutMetrics {
    pub fn calculate(rkit: &EditorBuffer, ctx: &Context) -> Self {
        let font_cfg = &rkit.config.font;
        let font_string = format!("{} {}", font_cfg.font_name(), font_cfg.font_size());
        let text_font_desc = pango::FontDescription::from_string(&font_string);
        // Gutter uses the same font as text for alignment
        let gutter_font_desc = text_font_desc.clone();
        let mut text_metrics = FontMetrics::calculate(ctx, &text_font_desc);
        let mut gutter_metrics = FontMetrics::calculate(ctx, &gutter_font_desc);
        // Use unified line height calculation for consistency
        let paragraph_spacing = font_cfg.font_paragraph_spacing();
        // Use a default glyph_height for initial line_height (for metrics, not per-line)
        let line_height = crate::corelogic::line_height::unified_line_height(
            text_metrics.height,
            gutter_metrics.height,
            text_metrics.height, // fallback: use text_metrics.height for initial metrics
            paragraph_spacing,
        );
        text_metrics.baseline_offset = (line_height - text_metrics.height) / 2.0 + text_metrics.baseline;
        gutter_metrics.baseline_offset = (line_height - gutter_metrics.height) / 2.0 + gutter_metrics.baseline;
        let text_left_offset = if rkit.config.gutter.toggle {
            rkit.config.gutter.ltr_width as f64 + rkit.config.margin_left
        } else {
            rkit.config.margin_left
        };
        let top_offset = rkit.config.margin_top;
        
        // Initialize line_metrics with per-line tallest glyph measurement
        let mut line_metrics = Vec::with_capacity(rkit.lines.len());
        let mut y = top_offset;
        for line in &rkit.lines {
            // Measure tallest glyph in this line using Pango
            let layout = pangocairo::functions::create_layout(ctx);
            layout.set_font_description(Some(&text_font_desc));
            layout.set_text(line);
            let glyph_height = layout.extents().1.height() as f64 / pango::SCALE as f64;
            let per_line_height = crate::corelogic::line_height::unified_line_height(
                text_metrics.height,
                gutter_metrics.height,
                glyph_height,
                paragraph_spacing,
            );
            line_metrics.push(LineMetrics {
                y_top: y,
                height: per_line_height,
            });
            y += per_line_height;
        }
        
        Self {
            line_height,
            text_metrics,
            gutter_metrics,
            text_left_offset,
            top_offset,
            line_metrics,
        }
    }

    /// Build a TabArray using the configured tab width in spaces and the average glyph width.
    /// Aligns tab stops every N spaces starting at x=0 in pixels.
    pub fn build_tab_array(&self, cfg: &crate::config::configuration::EditorConfig) -> pango::TabArray {
        let spaces = cfg.tab_width_spaces.max(1);
        let tab_px = self.text_metrics.average_char_width * spaces as f64;
        // Create, then set a sequence of tab stops up to a reasonable count.
        // Pango treats beyond-last tabs by repeating last interval if using aligned char, but for Left tabs we add many.
        let count = 64; // plenty for typical lines; Pango handles excess gracefully.
        // positions_in_pixels=false -> locations are in Pango units (multiply by SCALE)
        let mut tabs = pango::TabArray::new(count, false);
        for i in 0..count {
            let pos = ((i as f64 + 1.0) * tab_px) * pango::SCALE as f64;
            tabs.set_tab(i as i32, pango::TabAlign::Left, pos as i32);
        }
        tabs
    }
}
