//! Integration tests for the scroll system
//! 
//! These tests verify that the complete scroll system works correctly
//! including configuration, event handling, and API integration.

#[cfg(test)]
mod scroll_integration_tests {
    use crate::corelogic::scroll::{ScrollConfig, ScrollPolicy};
    use crate::widget::{EditorWidget, ScrollableWidget};
    use std::sync::Once;

    static INIT_GTK: Once = Once::new();

    fn init_gtk() {
        INIT_GTK.call_once(|| {
            gtk4::init().expect("Failed to initialize GTK");
        });
    }
    
    /// Create a test EditorWidget with custom scroll configuration
    fn create_test_widget_with_scroll_config(scroll_config: ScrollConfig) -> EditorWidget {
        init_gtk();
        
        // Create widget first
        let widget = EditorWidget::new();
        
        // Update the buffer's configuration
        {
            let mut buf = widget.buffer.borrow_mut();
            buf.config.scroll = scroll_config;
        }
        
        widget
    }
    
    #[test]
    fn test_scroll_api_basic_functionality() {
        let widget = create_test_widget_with_scroll_config(ScrollConfig::default());
        
        // Set up viewport and content sizes first so scroll offsets aren't clamped to 0
        widget.update_viewport_size(800.0, 600.0);
        widget.update_content_size(1600.0, 1200.0);
        
        // Test initial scroll position
        assert_eq!(widget.get_scroll_offset(), (0.0, 0.0));
        
        // Test setting scroll offsets
        widget.set_scroll_offset(10.0, 20.0);
        assert_eq!(widget.get_scroll_offset(), (10.0, 20.0));
        
        // Test individual offset setters
        widget.set_horizontal_scroll_offset(50.0);
        widget.set_vertical_scroll_offset(60.0);
        assert_eq!(widget.get_horizontal_scroll_offset(), 50.0);
        assert_eq!(widget.get_vertical_scroll_offset(), 60.0);
        
        // Test reset
        widget.reset_scroll();
        assert_eq!(widget.get_scroll_offset(), (0.0, 0.0));
    }
    
    #[test]
    fn test_scroll_by_functionality() {
        let widget = create_test_widget_with_scroll_config(ScrollConfig::default());
        
        // Set up viewport and content sizes first
        widget.update_viewport_size(800.0, 600.0);
        widget.update_content_size(1600.0, 1200.0);
        
        // Start at origin
        assert_eq!(widget.get_scroll_offset(), (0.0, 0.0));
        
        // Scroll by positive amounts
        widget.scroll_by(10.0, 15.0);
        assert_eq!(widget.get_scroll_offset(), (10.0, 15.0));
        
        // Scroll by negative amounts (should be clamped to 0)
        widget.scroll_by(-20.0, -25.0);
        assert_eq!(widget.get_scroll_offset(), (0.0, 0.0));
    }
    
    #[test]
    fn test_scroll_configuration_respects_enabled_flags() {
        // Test with horizontal scrolling disabled
        let mut config = ScrollConfig::default();
        config.horizontal_scroll_enabled = false;
        config.vertical_scroll_enabled = true;
        
        let widget = create_test_widget_with_scroll_config(config);
        
        // Set up viewport and content sizes
        widget.update_viewport_size(800.0, 600.0);
        widget.update_content_size(1600.0, 1200.0);
        
        // Should only allow vertical scrolling
        widget.scroll_by(10.0, 15.0);
        let (h, v) = widget.get_scroll_offset();
        assert_eq!(h, 0.0); // Horizontal should not change
        assert_eq!(v, 15.0); // Vertical should change
    }
    
    #[test]
    fn test_scroll_policy_configuration() {
        let mut config = ScrollConfig::default();
        config.horizontal_scroll_policy = ScrollPolicy::Always;
        config.vertical_scroll_policy = ScrollPolicy::Never;
        
        let widget = create_test_widget_with_scroll_config(config);
        let scroll_config = widget.get_scroll_config();
        
        assert_eq!(scroll_config.horizontal_scroll_policy, ScrollPolicy::Always);
        assert_eq!(scroll_config.vertical_scroll_policy, ScrollPolicy::Never);
    }
    
    #[test]
    fn test_scroll_sensitivity_configuration() {
        let mut config = ScrollConfig::default();
        config.scroll_sensitivity = 2.0;
        config.scroll_step_size = 5.0;
        
        let widget = create_test_widget_with_scroll_config(config);
        let scroll_config = widget.get_scroll_config();
        
        assert_eq!(scroll_config.scroll_sensitivity, 2.0);
        assert_eq!(scroll_config.scroll_step_size, 5.0);
    }
    
    #[test]
    fn test_viewport_and_content_size_updates() {
        let widget = create_test_widget_with_scroll_config(ScrollConfig::default());
        
        // Set initial viewport and content sizes
        widget.update_viewport_size(800.0, 600.0);
        widget.update_content_size(1600.0, 1200.0);
        
        // Test maximum scroll values
        let (max_h, max_v) = widget.get_max_scroll();
        assert_eq!(max_h, 800.0); // 1600 - 800
        assert_eq!(max_v, 600.0); // 1200 - 600
        
        // Test scroll needs detection
        assert!(widget.needs_horizontal_scroll());
        assert!(widget.needs_vertical_scroll());
    }
    
    #[test]
    fn test_scroll_visibility_with_policies() {
        let widget = create_test_widget_with_scroll_config(ScrollConfig::default());
        
        // Test with different policies
        assert!(widget.show_horizontal_scrollbar(ScrollPolicy::Always));
        assert!(widget.show_vertical_scrollbar(ScrollPolicy::Always));
        
        assert!(!widget.show_horizontal_scrollbar(ScrollPolicy::Never));
        assert!(!widget.show_vertical_scrollbar(ScrollPolicy::Never));
        
        // For automatic, need to set up content overflow
        widget.update_viewport_size(800.0, 600.0);
        widget.update_content_size(1600.0, 1200.0);
        
        assert!(widget.show_horizontal_scrollbar(ScrollPolicy::Automatic));
        assert!(widget.show_vertical_scrollbar(ScrollPolicy::Automatic));
    }
    
    #[test]
    fn test_scroll_to_position() {
        let widget = create_test_widget_with_scroll_config(ScrollConfig::default());
        
        // Set up viewport and content
        widget.update_viewport_size(800.0, 600.0);
        widget.update_content_size(1600.0, 1200.0);
        
        // Scroll to specific position
        widget.scroll_to_position(100.0, 150.0);
        assert_eq!(widget.get_scroll_offset(), (100.0, 150.0));
        
        // Test clamping to max values
        widget.scroll_to_position(1000.0, 800.0);
        assert_eq!(widget.get_scroll_offset(), (800.0, 600.0)); // Clamped to max
    }
}
