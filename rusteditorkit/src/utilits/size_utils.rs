
//! Helper utilities for widget sizing
//!
//! # Functions
//! - `clamp_dim(val, min, max)`: Clamp a dimension between min and max
//! - `detect_container(widget)`: Detects parent container type (Box, Grid, Overlay, Other)
//! - `percent_of_parent(widget, w_percent, h_percent)`: Calculates percent dimensions of parent
//! - `aspect_ratio_dims(width, aspect)`: Calculates height for a given width and aspect ratio

use gtk4::prelude::*;
use crate::corelogic::size_config::ContainerMode;

pub fn clamp_dim(val: i32, min: i32, max: i32) -> i32 {
    val.max(min).min(max)
}

pub fn detect_container(widget: &impl WidgetExt) -> ContainerMode {
    if let Some(parent) = widget.parent() {
        let type_name = parent.type_().name();
        match type_name {
            "GtkBox" => ContainerMode::Box,
            "GtkGrid" => ContainerMode::Grid,
            "GtkOverlay" => ContainerMode::Overlay,
            _ => ContainerMode::Other,
        }
    } else {
        ContainerMode::Other
    }
}

pub fn percent_of_parent(widget: &impl WidgetExt, w_percent: f64, h_percent: f64) -> (i32, i32) {
    if let Some(parent) = widget.parent() {
        let alloc = parent.allocation();
        let w = (alloc.width() as f64 * w_percent).round() as i32;
        let h = (alloc.height() as f64 * h_percent).round() as i32;
        (w, h)
    } else {
        (0, 0)
    }
}

pub fn aspect_ratio_dims(width: i32, aspect: f64) -> (i32, i32) {
    let height = (width as f64 / aspect).round() as i32;
    (width, height)
}
