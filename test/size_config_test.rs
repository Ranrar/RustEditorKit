//! Unit tests for ConfigurableSize trait and SizeMode

#[cfg(test)]
mod tests {
    use super::*;
    use gtk4::prelude::*;
    use gtk4::DrawingArea;

    #[test]
    fn test_fixed_size() {
        let da = DrawingArea::new();
        da.configure_size(SizeMode::Fixed(100, 50));
        assert_eq!(da.width_request(), 100);
        assert_eq!(da.height_request(), 50);
        assert!(da.can_focus());
    }

    #[test]
    fn test_minimum_size() {
        let da = DrawingArea::new();
        da.configure_size(SizeMode::Minimum(80, 40));
        assert_eq!(da.width_request(), 80);
        assert_eq!(da.height_request(), 40);
        assert!(da.hexpands());
        assert!(da.vexpands());
    }

    #[test]
    fn test_dynamic_size() {
        let da = DrawingArea::new();
        da.configure_size(SizeMode::Dynamic);
        assert_eq!(da.width_request(), -1);
        assert_eq!(da.height_request(), -1);
        assert!(da.hexpands());
        assert!(da.vexpands());
    }

    #[test]
    fn test_switch_mode() {
        let da = DrawingArea::new();
        da.configure_size(SizeMode::Fixed(120, 60));
        da.switch_mode(SizeMode::Minimum(60, 30));
        assert_eq!(da.width_request(), 60);
        assert_eq!(da.height_request(), 30);
    }
}
