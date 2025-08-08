//! Modular gutter rendering logic for EditorBuffer


use serde::Deserialize;
use gtk4::cairo::Context;
use gtk4::pango;
use crate::core::EditorBuffer;

#[derive(Debug, Clone, Deserialize)]
pub struct GutterConfig {
    pub toggle: bool,
    pub ltr_width: i32,
    pub padding: i32,
    pub bg_color: String,
    pub border: GutterBorderConfig,
    pub line_numbers: GutterLineNumbersConfig,
    pub font_size: i32,
    pub font_weight: String,
    pub active_line: GutterActiveLineConfig,
    pub markers: GutterMarkersConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GutterBorderConfig {
    pub toggle: bool,
    pub color: String,
    pub width: i32,
    pub style: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GutterLineNumbersConfig {
    pub color: String,
    pub ltr_width: i32,
    pub align: String,
    pub padding: i32,
    pub hover_color: String,
    pub active_clickable: bool,
}


#[derive(Debug, Clone, Deserialize)]
pub struct GutterActiveLineConfig {
    pub line_number_color: String,
    pub highlight_toggle: bool,
    pub highlight_color: String,
    pub highlight_opacity: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GutterMarkersConfig {
    pub enabled: bool,
    pub icon_char: String,
    pub icon_path: String,
    pub icon_size: i32,
    pub color: String,
    pub hover_color: String,
    pub spacing: i32,
    pub position: String,
}

impl Default for GutterConfig {
    fn default() -> Self {
        Self {
            toggle: true,
            ltr_width: 50,
            padding: 4,
            bg_color: "#1e1e1e".to_string(),
            border: GutterBorderConfig::default(),
            line_numbers: GutterLineNumbersConfig::default(),
            font_size: 11,
            font_weight: "normal".to_string(),
            active_line: GutterActiveLineConfig::default(),
            markers: GutterMarkersConfig::default(),
        }
    }
}

impl Default for GutterBorderConfig {
    fn default() -> Self {
        Self {
            toggle: true,
            color: "#444".to_string(),
            width: 1,
            style: "solid".to_string(),
        }
    }
}

impl Default for GutterLineNumbersConfig {
    fn default() -> Self {
        Self {
            color: "#aaa".to_string(),
            ltr_width: 35,
            align: "right".to_string(),
            padding: 6,
            hover_color: "#fff".to_string(),
            active_clickable: false,
        }
    }
}


impl Default for GutterActiveLineConfig {
    fn default() -> Self {
        Self {
            line_number_color: "#fff".to_string(),
            highlight_toggle: true,
            highlight_color: "#333".to_string(),
            highlight_opacity: 0.6,
        }
    }
}

impl Default for GutterMarkersConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            icon_char: "●".to_string(),
            icon_path: "res/icons/breakpoint.svg".to_string(),
            icon_size: 12,
            color: "#e06c75".to_string(),
            hover_color: "#ffb3b3".to_string(),
            spacing: 4,
            position: "left".to_string(),
        }
    }
}

/// Helper: parse color string to RGBA (reuse from render.rs or move to a utils module)
pub fn parse_color(color: &str) -> (f64, f64, f64, f64) {
    if let Some(stripped) = color.strip_prefix('#') {
        match stripped.len() {
            6 => {
                let r = u8::from_str_radix(&stripped[0..2], 16).unwrap_or(0) as f64 / 255.0;
                let g = u8::from_str_radix(&stripped[2..4], 16).unwrap_or(0) as f64 / 255.0;
                let b = u8::from_str_radix(&stripped[4..6], 16).unwrap_or(0) as f64 / 255.0;
                (r, g, b, 1.0)
            }
            8 => {
                let r = u8::from_str_radix(&stripped[0..2], 16).unwrap_or(0) as f64 / 255.0;
                let g = u8::from_str_radix(&stripped[2..4], 16).unwrap_or(0) as f64 / 255.0;
                let b = u8::from_str_radix(&stripped[4..6], 16).unwrap_or(0) as f64 / 255.0;
                let a = u8::from_str_radix(&stripped[6..8], 16).unwrap_or(255) as f64 / 255.0;
                (r, g, b, a)
            }
            _ => (0.0, 0.0, 0.0, 1.0)
        }
    } else if color.starts_with("rgba") {
        let nums: Vec<f64> = color[5..color.len()-1].split(',').filter_map(|s| s.trim().parse().ok()).collect();
        if nums.len() == 4 {
            (nums[0]/255.0, nums[1]/255.0, nums[2]/255.0, nums[3])
        } else {
            (0.0, 0.0, 0.0, 1.0)
        }
    } else {
        (0.0, 0.0, 0.0, 1.0)
    }
}

/// Render the gutter (background, border, line numbers, markers, etc.)
pub fn render_gutter(
    rkit: &EditorBuffer,
    ctx: &Context,
    height: i32,
    gutter_cfg: &GutterConfig,
    line_count: usize,
    active_row: usize,
    global_line_height: f64,
    top_offset: f64,
    layout: &crate::render::layout::LayoutMetrics,
) {
    if !gutter_cfg.toggle { return; }
    // Draw gutter background
    let (r, g, b, a) = parse_color(&gutter_cfg.bg_color);
    ctx.set_source_rgba(r, g, b, a);
    ctx.rectangle(0.0, 0.0, gutter_cfg.ltr_width as f64, height as f64);
    ctx.fill().unwrap_or(());

    // Draw gutter border if enabled
    if gutter_cfg.border.toggle {
        let (r, g, b, a) = parse_color(&gutter_cfg.border.color);
        ctx.set_source_rgba(r, g, b, a);
        ctx.set_line_width(gutter_cfg.border.width as f64);
        // Right border
        ctx.move_to(gutter_cfg.ltr_width as f64, 0.0);
        ctx.line_to(gutter_cfg.ltr_width as f64, height as f64);
        ctx.stroke().unwrap_or(());
    }

    // Prepare font for line numbers
    // Always use the same font as in font config.ron for gutter font
    let font_name = rkit.config.font.font_name();
    let gutter_font_size = gutter_cfg.font_size.max(8);
    let font_desc = pango::FontDescription::from_string(&format!(
        "{} {}",
        font_name,
        gutter_font_size
    ));
    let char_spacing = 0;
    // Calculate gutter line height from font metrics using the same Pango context as rendering
    // (gutter_line_height is now measured in render_editor and maxed with editor font height)

    // Use unified y-offsets for perfect alignment with text lines and apply scroll
    let mut y_offsets = rkit.line_y_offsets(global_line_height, rkit.config.font.font_paragraph_spacing(), top_offset);
    let scroll_px = (rkit.scroll_offset as f64) * global_line_height;
    for y in &mut y_offsets { *y -= scroll_px; }
    for i in 0..line_count {
        let y = y_offsets.get(i).copied().unwrap_or(top_offset);
        // ...highlight is now drawn in render/highlight.rs...
        // Line number color
        let color = if i == active_row {
            &gutter_cfg.active_line.line_number_color
        } else {
            &gutter_cfg.line_numbers.color
        };
        let (r, g, b, a) = parse_color(color);
        ctx.set_source_rgba(r, g, b, a);
        let pango_layout = pangocairo::functions::create_layout(ctx);
        pango_layout.set_text(&format!("{}", i + 1));
        pango_layout.set_font_description(Some(&font_desc));
        pango_layout.set_spacing(char_spacing);
        let context = pango_layout.context();
        context.set_round_glyph_positions(true);
        // Alignment
        let text_width = pango_layout.pixel_size().0 as f64;
        let align = gutter_cfg.line_numbers.align.as_str();
        let x = match align {
            "left" => gutter_cfg.line_numbers.padding as f64,
            "center" => (gutter_cfg.ltr_width as f64 - text_width) / 2.0,
            _ => gutter_cfg.ltr_width as f64 - text_width - gutter_cfg.line_numbers.padding as f64,
        };
        // Align gutter number to text baseline using gutter_metrics.baseline_offset from LayoutMetrics
        let y_baseline = y + layout.gutter_metrics.baseline_offset;
        ctx.move_to(x, y_baseline);
        pangocairo::functions::show_layout(ctx, &pango_layout);
        // TODO: Markers, hover, clickable, etc.
    }
}
