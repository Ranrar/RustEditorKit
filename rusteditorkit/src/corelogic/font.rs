// Modular FontConfig and font API for RustEditorKit (cross-platform)
use serde::Deserialize;

/// Modular font configuration for the editor core and widgets
#[derive(Debug, Clone, Deserialize)]
pub struct FontConfig {
    // === Basic Font Settings ===
    pub font_name: String,
    pub font_size: f64,
    pub font_color: String,

    // === Layout & Spacing ===
    pub font_line_height: f64,
    pub font_character_spacing: f64,
    pub font_word_spacing: f64,
    pub font_letter_case: String, // "Normal", "Uppercase", "Lowercase", "SmallCaps"

    // === Font Style ===
    pub font_weight: String,      // "Normal", "Bold", or numeric
    pub font_style: String,       // "Normal", "Italic", "Oblique"
    pub font_stretch: String,     // "Normal", "Condensed", etc.

    // === Rendering Options ===
    pub font_antialias: bool,
    pub font_hinting: String,     // "None", "Slight", "Medium", "Full"
    pub font_ligatures: bool,
    pub font_subpixel_order: String,

    // === Advanced Typography ===
    pub font_features: Vec<String>,
    pub font_variants: Vec<String>,
    pub font_language: String,

    // === Fallbacks ===
    pub font_fallbacks: Vec<String>,

    // === Shadow / Glow ===
    pub font_shadow_toggle: bool,
    pub font_shadow_color: String,
    pub font_shadow_offset_x: f64,
    pub font_shadow_offset_y: f64,
    pub font_shadow_blur_radius: f64,

    // === Experimental / Platform Specific ===
    pub font_decimal_alignment: bool,
    pub font_use_core_text: bool,

}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            font_name: "Fira Code".to_string(),
            font_size: 14.0,
            font_color: "#222222".to_string(),
            font_line_height: 22.0,
            font_character_spacing: 0.0,
            font_word_spacing: 0.0,
            font_letter_case: "Normal".to_string(),
            font_weight: "Normal".to_string(),
            font_style: "Normal".to_string(),
            font_stretch: "Normal".to_string(),
            font_antialias: true,
            font_hinting: "Medium".to_string(),
            font_ligatures: true,
            font_subpixel_order: "RGB".to_string(),
            font_features: vec!["kern".to_string(), "liga".to_string(), "clig".to_string()],
            font_variants: vec!["normal".to_string()],
            font_language: "en".to_string(),
            font_fallbacks: vec![
                "JetBrains Mono".to_string(),
                "DejaVu Sans Mono".to_string(),
                "Courier New".to_string(),
                "Monospace".to_string(),
            ],
            font_shadow_toggle: false,
            font_shadow_color: "#888888".to_string(),
            font_shadow_offset_x: 1.0,
            font_shadow_offset_y: 1.0,
            font_shadow_blur_radius: 2.0,
            font_decimal_alignment: false,
            font_use_core_text: false,
        }
    }
}

impl FontConfig {
    /// Modular API: Getters and setters for all font properties
    pub fn set_font_name(&mut self, name: &str) { self.font_name = name.to_string(); }
    pub fn font_name(&self) -> &str { &self.font_name }
    pub fn set_font_size(&mut self, size: f64) { self.font_size = size; }
    pub fn font_size(&self) -> f64 { self.font_size }
    pub fn set_font_color(&mut self, color: &str) { self.font_color = color.to_string(); }
    pub fn font_color(&self) -> &str { &self.font_color }
    pub fn set_font_line_height(&mut self, h: f64) { self.font_line_height = h; }
    pub fn font_line_height(&self) -> f64 { self.font_line_height }
    pub fn set_font_character_spacing(&mut self, s: f64) { self.font_character_spacing = s; }
    pub fn font_character_spacing(&self) -> f64 { self.font_character_spacing }
    pub fn set_font_word_spacing(&mut self, s: f64) { self.font_word_spacing = s; }
    pub fn font_word_spacing(&self) -> f64 { self.font_word_spacing }
    pub fn set_font_letter_case(&mut self, c: &str) { self.font_letter_case = c.to_string(); }
    pub fn font_letter_case(&self) -> &str { &self.font_letter_case }
    pub fn set_font_weight(&mut self, w: &str) { self.font_weight = w.to_string(); }
    pub fn font_weight(&self) -> &str { &self.font_weight }
    pub fn set_font_style(&mut self, s: &str) { self.font_style = s.to_string(); }
    pub fn font_style(&self) -> &str { &self.font_style }
    pub fn set_font_stretch(&mut self, s: &str) { self.font_stretch = s.to_string(); }
    pub fn font_stretch(&self) -> &str { &self.font_stretch }
    pub fn set_font_antialias(&mut self, v: bool) { self.font_antialias = v; }
    pub fn font_antialias(&self) -> bool { self.font_antialias }
    pub fn set_font_hinting(&mut self, h: &str) { self.font_hinting = h.to_string(); }
    pub fn font_hinting(&self) -> &str { &self.font_hinting }
    pub fn set_font_ligatures(&mut self, v: bool) { self.font_ligatures = v; }
    pub fn font_ligatures(&self) -> bool { self.font_ligatures }
    pub fn set_font_subpixel_order(&mut self, s: &str) { self.font_subpixel_order = s.to_string(); }
    pub fn font_subpixel_order(&self) -> &str { &self.font_subpixel_order }
    pub fn set_font_features(&mut self, f: Vec<String>) { self.font_features = f; }
    pub fn font_features(&self) -> &Vec<String> { &self.font_features }
    pub fn set_font_variants(&mut self, v: Vec<String>) { self.font_variants = v; }
    pub fn font_variants(&self) -> &Vec<String> { &self.font_variants }
    pub fn set_font_language(&mut self, l: &str) { self.font_language = l.to_string(); }
    pub fn font_language(&self) -> &str { &self.font_language }
    pub fn set_font_fallbacks(&mut self, f: Vec<String>) { self.font_fallbacks = f; }
    pub fn font_fallbacks(&self) -> &Vec<String> { &self.font_fallbacks }
    pub fn set_font_shadow_toggle(&mut self, v: bool) { self.font_shadow_toggle = v; }
    pub fn font_shadow_toggle(&self) -> bool { self.font_shadow_toggle }
    pub fn set_font_shadow_color(&mut self, c: &str) { self.font_shadow_color = c.to_string(); }
    pub fn font_shadow_color(&self) -> &str { &self.font_shadow_color }
    pub fn set_font_shadow_offset_x(&mut self, x: f64) { self.font_shadow_offset_x = x; }
    pub fn font_shadow_offset_x(&self) -> f64 { self.font_shadow_offset_x }
    pub fn set_font_shadow_offset_y(&mut self, y: f64) { self.font_shadow_offset_y = y; }
    pub fn font_shadow_offset_y(&self) -> f64 { self.font_shadow_offset_y }
    pub fn set_font_shadow_blur_radius(&mut self, r: f64) { self.font_shadow_blur_radius = r; }
    pub fn font_shadow_blur_radius(&self) -> f64 { self.font_shadow_blur_radius }
    pub fn set_font_decimal_alignment(&mut self, v: bool) { self.font_decimal_alignment = v; }
    pub fn font_decimal_alignment(&self) -> bool { self.font_decimal_alignment }
    pub fn set_font_use_core_text(&mut self, v: bool) { self.font_use_core_text = v; }
    pub fn font_use_core_text(&self) -> bool { self.font_use_core_text }
}
