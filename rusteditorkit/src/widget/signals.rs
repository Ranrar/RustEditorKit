//! Signal connections for the EditorWidget
//! Handles connecting various signals and event handlers

use gtk4::prelude::*;
use gtk4::glib::translate::IntoGlib;
use crate::widget::focus::FocusManager;
use crate::widget::editor::EditorWidget;

impl EditorWidget {
    /// Connect all signals for the editor widget
    pub fn connect_signals(&self) {
        // Setup focus controllers
        FocusManager::setup_focus_controllers(&self.drawing_area);
        
        // Connect draw signal using modular render system
        self.connect_draw_signal();
        
        // Connect mouse event handlers
        self.connect_mouse_signals();
        
        // Connect key event handler using unified keybind system
        let buffer_clone = self.buffer().clone();
        let keymap_clone = self.keymap.clone();
        let key_controller = gtk4::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_controller, keyval, _keycode, state| {
            // Convert GTK key event to KeyCombo for mapping
            let keyval_u32: u32 = keyval.into_glib();
            let combo = crate::keybinds::KeyCombo::from_gtk_event(keyval_u32, state);
            
            // Debug output for key events
            println!("[KEYBIND DEBUG] Key event: {:?}", combo);
            
            // Find matching action in keymap
            if let Some((&action, _)) = keymap_clone.iter().find(|(_, kc)| **kc == combo) {
                println!("[KEYBIND DEBUG] Dispatched action: {:?}", action);
                
                // Special handling for clipboard operations that require async access
                if action == crate::keybinds::EditorAction::PasteClipboard {
                    // Handle paste operation with proper async clipboard access
                    let buffer_for_paste = buffer_clone.clone();
                    if let Some(display) = gtk4::gdk::Display::default() {
                        let clipboard = display.clipboard();
                        clipboard.read_text_async(
                            gtk4::gio::Cancellable::NONE,
                            move |result| {
                                match result {
                                    Ok(Some(text)) => {
                                        println!("[DEBUG] Clipboard paste: {}", text);
                                        let mut buf = buffer_for_paste.borrow_mut();
                                        buf.paste_text(&text);
                                        buf.request_redraw();
                                    },
                                    Ok(None) => println!("[DEBUG] Clipboard is empty"),
                                    Err(e) => eprintln!("[ERROR] Clipboard error: {}", e),
                                }
                            }
                        );
                    }
                    return glib::Propagation::Stop;
                } else {
                    // Handle other keybind actions via dispatcher
                    let mut buf = buffer_clone.borrow_mut();
                    buf.handle_editor_action(action);
                    return glib::Propagation::Stop;
                }
            }
            
            // Fallback: handle regular character input for typing
            if let Some(text_char) = keyval.to_unicode() {
                // Only handle printable characters and basic whitespace
                if text_char.is_ascii_graphic() || text_char == ' ' || text_char == '\t' {
                    let mut buf = buffer_clone.borrow_mut();
                    buf.handle_text_input(&text_char.to_string());
                    return glib::Propagation::Stop;
                }
            }
            
            glib::Propagation::Proceed
        });
        self.drawing_area.add_controller(key_controller);
    }

    /// Connect mouse event handlers for selection support
    fn connect_mouse_signals(&self) {
        // Primary mouse button controller (for clicking and dragging)
        let buffer_primary = self.buffer().clone();
        let mouse_primary = gtk4::GestureClick::new();
        mouse_primary.set_button(1); // Left mouse button
        
        // Handle single clicks
        let buffer_click = buffer_primary.clone();
        mouse_primary.connect_pressed(move |gesture, _n_press, x, y| {
            let state = gesture.current_event_state();
            let shift_held = state.contains(gtk4::gdk::ModifierType::SHIFT_MASK);
            
            println!("[MOUSE DEBUG] Click at ({:.1}, {:.1}), shift: {}", x, y, shift_held);
            
            let mut buf = buffer_click.borrow_mut();
            // Use approximate metrics - in a real implementation, get these from layout
            let line_height = 20.0; // Approximate
            let char_width = 10.0;   // Approximate  
            let left_margin = 50.0;  // Approximate gutter width
            let top_margin = 5.0;    // Approximate top padding
            
            buf.handle_mouse_click(x, y, shift_held, line_height, char_width, left_margin, top_margin);
            buf.request_redraw();
        });

        // Handle double and triple clicks
        let buffer_multi = buffer_primary.clone();
        mouse_primary.connect_released(move |_gesture, n_press, x, y| {
            let mut buf = buffer_multi.borrow_mut();
            // Use approximate metrics
            let line_height = 20.0;
            let char_width = 10.0;
            let left_margin = 50.0;
            let top_margin = 5.0;
            
            match n_press {
                2 => {
                    println!("[MOUSE DEBUG] Double-click at ({:.1}, {:.1})", x, y);
                    buf.handle_double_click(x, y, line_height, char_width, left_margin, top_margin);
                },
                3 => {
                    println!("[MOUSE DEBUG] Triple-click at ({:.1}, {:.1})", x, y);
                    buf.handle_triple_click(x, y, line_height, char_width, left_margin, top_margin);
                },
                _ => {
                    // Single click - already handled in pressed
                }
            }
            buf.request_redraw();
        });

        self.drawing_area.add_controller(mouse_primary);

        // Drag controller for selection
        let buffer_drag = self.buffer().clone();
        let drag_controller = gtk4::GestureDrag::new();
        
        let buffer_drag_update = buffer_drag.clone();
        drag_controller.connect_drag_update(move |drag_ctrl, _x, _y| {
            // Get absolute position
            if let Some((_start_x, _start_y)) = drag_ctrl.start_point() {
                if let Some((dx, dy)) = drag_ctrl.offset() {
                    let current_x = _start_x + dx;
                    let current_y = _start_y + dy;
                    
                    println!("[MOUSE DEBUG] Drag to ({:.1}, {:.1})", current_x, current_y);
                    
                    let mut buf = buffer_drag_update.borrow_mut();
                    // Use approximate metrics
                    let line_height = 20.0;
                    let char_width = 10.0;
                    let left_margin = 50.0;
                    let top_margin = 5.0;
                    
                    buf.handle_mouse_drag(current_x, current_y, line_height, char_width, left_margin, top_margin);
                    buf.request_redraw();
                }
            }
        });

        let buffer_drag_end = buffer_drag.clone();
        drag_controller.connect_drag_end(move |_, _x, _y| {
            println!("[MOUSE DEBUG] Drag ended");
            let mut buf = buffer_drag_end.borrow_mut();
            buf.handle_mouse_release();
        });

        self.drawing_area.add_controller(drag_controller);
    }

    /// Load and apply config from a RON file
    pub fn load_config_from_file(&self, path: &str) {
        crate::widget::config::ConfigManager::load_config_from_file(&self.buffer(), path);
    }

    /// Move cursor left
    pub fn move_cursor_left(&self) {
        let buffer = self.buffer();
        let mut buf = buffer.borrow_mut();
        buf.handle_editor_action(crate::keybinds::EditorAction::MoveCursorLeft);
    }

    /// Move cursor right
    pub fn move_cursor_right(&self) {
        let buffer = self.buffer();
        let mut buf = buffer.borrow_mut();
        buf.handle_editor_action(crate::keybinds::EditorAction::MoveCursorRight);
    }

    /// Move cursor up
    pub fn move_cursor_up(&self) {
        let buffer = self.buffer();
        let mut buf = buffer.borrow_mut();
        buf.handle_editor_action(crate::keybinds::EditorAction::MoveCursorUp);
    }

    /// Move cursor down
    pub fn move_cursor_down(&self) {
        let buffer = self.buffer();
        let mut buf = buffer.borrow_mut();
        buf.handle_editor_action(crate::keybinds::EditorAction::MoveCursorDown);
    }

    /// Insert a character at the cursor
    pub fn insert_char(&self, c: char) {
        let buffer = self.buffer();
        let mut buf = buffer.borrow_mut();
        buf.handle_text_input(&c.to_string());
    }

    /// Insert newline at cursor
    pub fn insert_newline(&self) {
        let buffer = self.buffer();
        let mut buf = buffer.borrow_mut();
        buf.handle_editor_action(crate::keybinds::EditorAction::InsertNewline);
    }

    /// Handle a key event (for integration)
    pub fn handle_key_event(&self, keyval: gtk4::gdk::Key) {
        let buffer = self.buffer();
        let mut buf = buffer.borrow_mut();
        
        // Convert key to action and handle via dispatcher
        let keyval_u32: u32 = keyval.into_glib();
        let combo = crate::keybinds::KeyCombo::from_gtk_event(keyval_u32, gtk4::gdk::ModifierType::empty());
        
        // Try to find matching action in keymap
        if let Some((&action, _)) = self.keymap.iter().find(|(_, kc)| **kc == combo) {
            buf.handle_editor_action(action);
        } else if let Some(text_char) = keyval.to_unicode() {
            // Fallback for character input
            if text_char.is_ascii_graphic() || text_char.is_whitespace() {
                buf.handle_text_input(&text_char.to_string());
            }
        }
    }
}
