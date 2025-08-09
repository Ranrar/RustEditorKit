
//! Unified sizing API for GTK widgets
//!
//! # Overview
//! This module provides a trait-based API for flexible widget sizing in GTK4 Rust applications.
//! It centralizes all sizing and focus logic, supporting multiple strategies:
//! - Fixed, minimum, dynamic, maximum, min/max, aspect ratio, auto-fit, percent-of-parent, container-aware
//!
//! # Example Usage
//! ```rust
//! use rusteditorkit::widget::ConfigurableSize;
//! use rusteditorkit::corelogic::SizeMode;
//! let drawing_area = gtk::DrawingArea::new();
//! drawing_area.configure_size(SizeMode::Minimum(400, 300));
//! drawing_area.switch_mode(SizeMode::AspectRatio(16.0/9.0));
//! ```
//!
//! # SizeMode Variants
//! - `Fixed(w, h)`: Exact width/height, no expansion
//! - `Minimum(w, h)`: Minimum width/height, expandable
//! - `Dynamic`: No constraints, fully expandable
//! - `Maximum(w, h)`: Max width/height, clamps on size change
//! - `MinMax { min_w, min_h, max_w, max_h }`: Min/max bounds
//! - `AspectRatio(ratio)`: Maintains width/height ratio
//! - `AutoFitToParent`: Matches parent size
//! - `PercentOfParent(wp, hp)`: Percent of parent size
//! - `ContainerAware(mode)`: Adapts to container type

#[non_exhaustive]
pub enum SizeMode {
    Fixed(u32, u32),
    Minimum(u32, u32),
    Dynamic,
    Maximum(u32, u32),
    MinMax { min_w: u32, min_h: u32, max_w: u32, max_h: u32 },
    AspectRatio(f64),
    AutoFitToParent,
    PercentOfParent(f64, f64),
    ContainerAware(ContainerMode),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerMode {
    Box,
    Grid,
    Overlay,
    Other,
}

pub trait ConfigurableSize {
    /// Configure the widget size according to the provided `SizeMode`.
    fn configure_size(&self, mode: SizeMode);
    /// Switch to a different size mode at runtime.
    fn switch_mode(&self, new_mode: SizeMode);
}

// Documentation and examples will be added after implementation.
