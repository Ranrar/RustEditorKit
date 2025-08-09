//! Scrollable widget trait for the scroll system
//! 
//! This module provides the API trait for accessing scroll functionality.

use crate::corelogic::scroll::{ScrollConfig, ScrollPolicy};

/// Trait for widgets that support internal scrolling
pub trait ScrollableWidget {
    /// Set the horizontal scroll offset
    fn set_horizontal_scroll_offset(&self, offset: f64);
    
    /// Set the vertical scroll offset 
    fn set_vertical_scroll_offset(&self, offset: f64);
    
    /// Set both scroll offsets at once
    fn set_scroll_offset(&self, horizontal: f64, vertical: f64);
    
    /// Get the current horizontal scroll offset
    fn get_horizontal_scroll_offset(&self) -> f64;
    
    /// Get the current vertical scroll offset
    fn get_vertical_scroll_offset(&self) -> f64;
    
    /// Get both scroll offsets as (horizontal, vertical) tuple
    fn get_scroll_offset(&self) -> (f64, f64);
    
    /// Get the maximum horizontal scroll value
    fn get_max_horizontal_scroll(&self) -> f64;
    
    /// Get the maximum vertical scroll value
    fn get_max_vertical_scroll(&self) -> f64;
    
    /// Get maximum scroll values as (horizontal, vertical) tuple
    fn get_max_scroll(&self) -> (f64, f64);
    
    /// Scroll by relative amounts
    fn scroll_by(&self, horizontal_delta: f64, vertical_delta: f64);
    
    /// Check if horizontal scrolling is needed (content overflows)
    fn needs_horizontal_scroll(&self) -> bool;
    
    /// Check if vertical scrolling is needed (content overflows)
    fn needs_vertical_scroll(&self) -> bool;
    
    /// Check if horizontal scrollbar should be visible based on policy
    fn show_horizontal_scrollbar(&self, policy: ScrollPolicy) -> bool;
    
    /// Check if vertical scrollbar should be visible based on policy
    fn show_vertical_scrollbar(&self, policy: ScrollPolicy) -> bool;
    
    /// Update viewport dimensions (called automatically during resize)
    fn update_viewport_size(&self, width: f64, height: f64);
    
    /// Update content dimensions (called automatically when content changes)
    fn update_content_size(&self, width: f64, height: f64);
    
    /// Reset scroll offsets to (0, 0)
    fn reset_scroll(&self);
    
    /// Scroll to make a specific position visible
    fn scroll_to_position(&self, x: f64, y: f64);
    
    /// Get current scroll configuration
    fn get_scroll_config(&self) -> ScrollConfig;
}
