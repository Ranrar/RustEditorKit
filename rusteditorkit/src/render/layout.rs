//! Handles geometry: line heights, spacing, padding, alignment
use gtk4::cairo::Context;
use gtk4::pango;
use crate::corelogic::EditorBuffer;

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
        // Always use the maximum of text or gutter heights for line_height
        let line_height = text_metrics.height.max(gutter_metrics.height);
        text_metrics.baseline_offset = (line_height - text_metrics.height) / 2.0 + text_metrics.baseline;
        gutter_metrics.baseline_offset = (line_height - gutter_metrics.height) / 2.0 + gutter_metrics.baseline;
        let text_left_offset = if rkit.config.gutter.toggle {
            rkit.config.gutter.ltr_width as f64 + rkit.config.margin_left
        } else {
            rkit.config.margin_left
        };
        let top_offset = rkit.config.margin_top;
        Self {
            line_height,
            text_metrics,
            gutter_metrics,
            text_left_offset,
            top_offset,
        }
    }
}
