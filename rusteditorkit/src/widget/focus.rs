//! Focus management for the EditorWidget
//! Handles focus events and mouse interactions

use gtk4::prelude::*;
use gtk4::DrawingArea;

/// Focus management utilities for the editor
pub struct FocusManager;

impl FocusManager {
    /// Setup focus controllers for the drawing area
    pub fn setup_focus_controllers(drawing_area: &DrawingArea) {
        // Grab focus on pointer events
        let focus_controller = gtk4::EventControllerFocus::new();
        let da_clone = drawing_area.clone();
        focus_controller.connect_enter(move |_| {
            da_clone.grab_focus();
        });
        drawing_area.add_controller(focus_controller);

        let motion_controller = gtk4::EventControllerMotion::new();
        let da_clone2 = drawing_area.clone();
        motion_controller.connect_motion(move |_, _, _| {
            da_clone2.grab_focus();
        });
        drawing_area.add_controller(motion_controller);
    }
}
