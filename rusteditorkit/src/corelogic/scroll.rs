//! Scroll system for internal content scrolling in DrawingArea
//! 
//! This module provides configurable scroll offset management and scroll policies
//! for content inside a gtk::DrawingArea without using GTK ScrolledWindow.

use serde::Deserialize;

/// Scroll policy options for each axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum ScrollPolicy {
    /// Always show scrollbar/indicator and enable scrolling
    Always,
    /// Show scrollbar/indicator only when content overflows, enable scrolling
    Automatic,
    /// Never show scrollbar/indicator, disable scrolling
    Never,
}

impl Default for ScrollPolicy {
    fn default() -> Self {
        ScrollPolicy::Automatic
    }
}

/// Configuration for scroll behavior loaded from config files
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ScrollConfig {
    /// Enable horizontal scrolling
    pub horizontal_scroll_enabled: bool,
    /// Enable vertical scrolling  
    pub vertical_scroll_enabled: bool,
    /// Horizontal scroll policy
    pub horizontal_scroll_policy: ScrollPolicy,
    /// Vertical scroll policy
    pub vertical_scroll_policy: ScrollPolicy,
    /// Scroll sensitivity multiplier
    pub scroll_sensitivity: f64,
    /// Whether to smooth scroll animations
    pub smooth_scrolling: bool,
    /// Scroll step size for discrete scrolling (lines)
    pub scroll_step_size: f64,
}

impl Default for ScrollConfig {
    fn default() -> Self {
        Self {
            horizontal_scroll_enabled: true,
            vertical_scroll_enabled: true,
            horizontal_scroll_policy: ScrollPolicy::Automatic,
            vertical_scroll_policy: ScrollPolicy::Automatic,
            scroll_sensitivity: 1.0,
            smooth_scrolling: true,
            scroll_step_size: 3.0,
        }
    }
}

/// Internal scroll state tracking offsets and bounds
#[derive(Debug, Clone)]
pub struct ScrollState {
    /// Current horizontal scroll offset (0.0 to max_horizontal)
    pub horizontal_offset: f64,
    /// Current vertical scroll offset (0.0 to max_vertical)
    pub vertical_offset: f64,
    /// Maximum horizontal scroll (content_width - viewport_width)
    pub max_horizontal: f64,
    /// Maximum vertical scroll (content_height - viewport_height)
    pub max_vertical: f64,
    /// Viewport width
    pub viewport_width: f64,
    /// Viewport height
    pub viewport_height: f64,
    /// Content width
    pub content_width: f64,
    /// Content height
    pub content_height: f64,
}

impl Default for ScrollState {
    fn default() -> Self {
        Self {
            horizontal_offset: 0.0,
            vertical_offset: 0.0,
            max_horizontal: 0.0,
            max_vertical: 0.0,
            viewport_width: 0.0,
            viewport_height: 0.0,
            content_width: 0.0,
            content_height: 0.0,
        }
    }
}

impl ScrollState {
    /// Create a new scroll state with given dimensions
    pub fn new(viewport_width: f64, viewport_height: f64, content_width: f64, content_height: f64) -> Self {
        let mut state = Self {
            horizontal_offset: 0.0,
            vertical_offset: 0.0,
            max_horizontal: 0.0,
            max_vertical: 0.0,
            viewport_width,
            viewport_height,
            content_width,
            content_height,
        };
        state.update_bounds();
        state
    }

    /// Update scroll bounds based on current viewport and content dimensions
    pub fn update_bounds(&mut self) {
        self.max_horizontal = (self.content_width - self.viewport_width).max(0.0);
        self.max_vertical = (self.content_height - self.viewport_height).max(0.0);
        
        // Clamp current offsets to new bounds
        self.clamp_offsets();
    }

    /// Set viewport dimensions and update bounds
    pub fn set_viewport_size(&mut self, width: f64, height: f64) {
        self.viewport_width = width;
        self.viewport_height = height;
        self.update_bounds();
    }

    /// Set content dimensions and update bounds
    pub fn set_content_size(&mut self, width: f64, height: f64) {
        self.content_width = width;
        self.content_height = height;
        self.update_bounds();
    }

    /// Clamp scroll offsets to valid bounds
    pub fn clamp_offsets(&mut self) {
        self.horizontal_offset = self.horizontal_offset.clamp(0.0, self.max_horizontal);
        self.vertical_offset = self.vertical_offset.clamp(0.0, self.max_vertical);
    }

    /// Set horizontal scroll offset with clamping
    pub fn set_horizontal_offset(&mut self, offset: f64) {
        self.horizontal_offset = offset.clamp(0.0, self.max_horizontal);
    }

    /// Set vertical scroll offset with clamping
    pub fn set_vertical_offset(&mut self, offset: f64) {
        self.vertical_offset = offset.clamp(0.0, self.max_vertical);
    }

    /// Set both scroll offsets with clamping
    pub fn set_scroll_offset(&mut self, horizontal: f64, vertical: f64) {
        self.set_horizontal_offset(horizontal);
        self.set_vertical_offset(vertical);
    }

    /// Get current scroll offsets as (horizontal, vertical) tuple
    pub fn get_scroll_offset(&self) -> (f64, f64) {
        (self.horizontal_offset, self.vertical_offset)
    }

    /// Apply scroll delta with clamping, respecting enabled axes
    pub fn scroll_by(&mut self, dx: f64, dy: f64, config: &ScrollConfig) {
        if config.horizontal_scroll_enabled {
            let new_x = self.horizontal_offset + (dx * config.scroll_sensitivity);
            self.set_horizontal_offset(new_x);
        }
        
        if config.vertical_scroll_enabled {
            let new_y = self.vertical_offset + (dy * config.scroll_sensitivity);
            self.set_vertical_offset(new_y);
        }
    }

    /// Check if horizontal scrolling is needed (content overflows)
    pub fn needs_horizontal_scroll(&self) -> bool {
        self.content_width > self.viewport_width
    }

    /// Check if vertical scrolling is needed (content overflows)
    pub fn needs_vertical_scroll(&self) -> bool {
        self.content_height > self.viewport_height
    }

    /// Check if horizontal scrollbar should be visible based on policy
    pub fn show_horizontal_scrollbar(&self, policy: ScrollPolicy) -> bool {
        match policy {
            ScrollPolicy::Always => true,
            ScrollPolicy::Automatic => self.needs_horizontal_scroll(),
            ScrollPolicy::Never => false,
        }
    }

    /// Check if vertical scrollbar should be visible based on policy
    pub fn show_vertical_scrollbar(&self, policy: ScrollPolicy) -> bool {
        match policy {
            ScrollPolicy::Always => true,
            ScrollPolicy::Automatic => self.needs_vertical_scroll(),
            ScrollPolicy::Never => false,
        }
    }

    /// Get scroll progress as (horizontal_progress, vertical_progress) between 0.0 and 1.0
    pub fn get_scroll_progress(&self) -> (f64, f64) {
        let horizontal_progress = if self.max_horizontal > 0.0 {
            self.horizontal_offset / self.max_horizontal
        } else {
            0.0
        };
        
        let vertical_progress = if self.max_vertical > 0.0 {
            self.vertical_offset / self.max_vertical
        } else {
            0.0
        };
        
        (horizontal_progress, vertical_progress)
    }

    /// Reset scroll offsets to origin
    pub fn reset(&mut self) {
        self.horizontal_offset = 0.0;
        self.vertical_offset = 0.0;
    }

    /// Scroll to make a rectangle visible
    /// Adjusts scroll offsets to ensure the specified rectangle is within the viewport
    pub fn scroll_to_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        // Ensure the rectangle is within bounds
        let rect_right = x + width;
        let rect_bottom = y + height;
        
        // Horizontal scrolling
        if x < self.horizontal_offset {
            // Rectangle is to the left of viewport, scroll left
            self.set_horizontal_offset(x);
        } else if rect_right > self.horizontal_offset + self.viewport_width {
            // Rectangle is to the right of viewport, scroll right
            self.set_horizontal_offset(rect_right - self.viewport_width);
        }
        
        // Vertical scrolling
        if y < self.vertical_offset {
            // Rectangle is above viewport, scroll up
            self.set_vertical_offset(y);
        } else if rect_bottom > self.vertical_offset + self.viewport_height {
            // Rectangle is below viewport, scroll down
            self.set_vertical_offset(rect_bottom - self.viewport_height);
        }
    }

    /// Scroll to make a point visible with optional margin
    pub fn scroll_to_point(&mut self, x: f64, y: f64, margin: f64) {
        self.scroll_to_rect(x - margin, y - margin, margin * 2.0, margin * 2.0);
    }

    /// Get the visible content rectangle (viewport in content coordinates)
    pub fn get_visible_rect(&self) -> (f64, f64, f64, f64) {
        (
            self.horizontal_offset,
            self.vertical_offset,
            self.viewport_width,
            self.viewport_height,
        )
    }

    /// Check if a point is visible in the current viewport
    pub fn is_point_visible(&self, x: f64, y: f64) -> bool {
        x >= self.horizontal_offset
            && x <= self.horizontal_offset + self.viewport_width
            && y >= self.vertical_offset
            && y <= self.vertical_offset + self.viewport_height
    }

    /// Check if a rectangle is fully visible in the current viewport
    pub fn is_rect_visible(&self, x: f64, y: f64, width: f64, height: f64) -> bool {
        let rect_right = x + width;
        let rect_bottom = y + height;
        
        x >= self.horizontal_offset
            && rect_right <= self.horizontal_offset + self.viewport_width
            && y >= self.vertical_offset
            && rect_bottom <= self.vertical_offset + self.viewport_height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_state_creation() {
        let state = ScrollState::new(800.0, 600.0, 1600.0, 1200.0);
        assert_eq!(state.viewport_width, 800.0);
        assert_eq!(state.viewport_height, 600.0);
        assert_eq!(state.content_width, 1600.0);
        assert_eq!(state.content_height, 1200.0);
        assert_eq!(state.max_horizontal, 800.0);
        assert_eq!(state.max_vertical, 600.0);
    }

    #[test]
    fn test_scroll_offset_clamping() {
        let mut state = ScrollState::new(800.0, 600.0, 1600.0, 1200.0);
        
        // Test setting offsets within bounds
        state.set_scroll_offset(400.0, 300.0);
        assert_eq!(state.get_scroll_offset(), (400.0, 300.0));
        
        // Test clamping beyond bounds
        state.set_scroll_offset(1000.0, 800.0);
        assert_eq!(state.get_scroll_offset(), (800.0, 600.0));
        
        // Test clamping below bounds
        state.set_scroll_offset(-100.0, -50.0);
        assert_eq!(state.get_scroll_offset(), (0.0, 0.0));
    }

    #[test]
    fn test_scroll_by() {
        let mut state = ScrollState::new(800.0, 600.0, 1600.0, 1200.0);
        let config = ScrollConfig::default();
        
        state.scroll_by(100.0, 50.0, &config);
        assert_eq!(state.get_scroll_offset(), (100.0, 50.0));
        
        // Test clamping on scroll_by
        state.scroll_by(1000.0, 1000.0, &config);
        assert_eq!(state.get_scroll_offset(), (800.0, 600.0));
    }

    #[test]
    fn test_scroll_needs_detection() {
        let state = ScrollState::new(800.0, 600.0, 1600.0, 1200.0);
        assert!(state.needs_horizontal_scroll());
        assert!(state.needs_vertical_scroll());
        
        let state2 = ScrollState::new(800.0, 600.0, 400.0, 300.0);
        assert!(!state2.needs_horizontal_scroll());
        assert!(!state2.needs_vertical_scroll());
    }

    #[test]
    fn test_scroll_progress() {
        let mut state = ScrollState::new(800.0, 600.0, 1600.0, 1200.0);
        
        // At origin
        assert_eq!(state.get_scroll_progress(), (0.0, 0.0));
        
        // At middle
        state.set_scroll_offset(400.0, 300.0);
        assert_eq!(state.get_scroll_progress(), (0.5, 0.5));
        
        // At end
        state.set_scroll_offset(800.0, 600.0);
        assert_eq!(state.get_scroll_progress(), (1.0, 1.0));
    }

    #[test]
    fn test_scroll_policy_defaults() {
        let config = ScrollConfig::default();
        assert_eq!(config.horizontal_scroll_policy, ScrollPolicy::Automatic);
        assert_eq!(config.vertical_scroll_policy, ScrollPolicy::Automatic);
        assert!(config.horizontal_scroll_enabled);
        assert!(config.vertical_scroll_enabled);
    }
}
