/// Signal connections for the EditorWidget
/// Handles connecting various signals and event handlers

impl EditorWidget {
}

use gtk4::prelude::*;
use gtk4::glib::translate::IntoGlib;
use gtk4::gdk::ModifierType;
use crate::widget::focus::FocusManager;
use crate::widget::editor::EditorWidget;
use crate::widget::input::InputHandler;

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
    let cached_metrics_rc = self.cached_metrics.clone();
    let cached_pango_ctx_rc = self.cached_pango_ctx.clone();
    let key_controller = gtk4::EventControllerKey::new();
    key_controller.connect_key_pressed(move |_controller, keyval, _keycode, state| {
            // First: prioritize arrow keys (with Shift for selection) using visual, bidi-aware movement
            use gtk4::gdk::Key;
            if matches!(keyval, Key::Left | Key::Right | Key::Up | Key::Down) {
                if let (Some(metrics), Some(pango_ctx)) = (
                    cached_metrics_rc.borrow().clone(),
                    cached_pango_ctx_rc.borrow().clone(),
                ) {
                    let shift = state.contains(ModifierType::SHIFT_MASK);
                    let mut buf = buffer_clone.borrow_mut();
                    match (keyval, shift) {
                        (Key::Left, false) => InputHandler::move_cursor_left(&mut buf, &metrics.layout, &pango_ctx),
                        (Key::Right, false) => InputHandler::move_cursor_right(&mut buf, &metrics.layout, &pango_ctx),
                        (Key::Up, false) => InputHandler::move_cursor_up(&mut buf, &metrics.layout, &pango_ctx),
                        (Key::Down, false) => InputHandler::move_cursor_down(&mut buf, &metrics.layout, &pango_ctx),
                        (Key::Left, true) => InputHandler::select_left(&mut buf, &metrics.layout, &pango_ctx),
                        (Key::Right, true) => InputHandler::select_right(&mut buf, &metrics.layout, &pango_ctx),
                        (Key::Up, true) => InputHandler::select_up(&mut buf, &metrics.layout, &pango_ctx),
                        (Key::Down, true) => InputHandler::select_down(&mut buf, &metrics.layout, &pango_ctx),
                        _ => {}
                    }
                    return glib::Propagation::Stop;
                }
            }

            // Next: Convert to KeyCombo and try keymap-based actions for everything else
            let keyval_u32: u32 = keyval.into_glib();
            let combo = crate::keybinds::KeyCombo::from_gtk_event(keyval_u32, state);
            println!("[KEYBIND DEBUG] Key event: {:?}", combo);
            if let Some((&action, _)) = keymap_clone.iter().find(|(_, kc)| **kc == combo) {
                println!("[KEYBIND DEBUG] Dispatched action: {:?}", action);
                if action == crate::keybinds::EditorAction::PasteClipboard {
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
                    let mut buf = buffer_clone.borrow_mut();
                    buf.handle_editor_action(action);
                    return glib::Propagation::Stop;
                }
            }

            // Fallback: handle regular character input for typing
            if let Some(text_char) = keyval.to_unicode() {
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
        } else if matches!(keyval, gtk4::gdk::Key::Left | gtk4::gdk::Key::Right | gtk4::gdk::Key::Up | gtk4::gdk::Key::Down) {
            let metrics_opt = self.cached_metrics.borrow().clone();
            let pango_ctx_opt = self.cached_pango_ctx.borrow().clone();
            if let (Some(metrics), Some(pango_ctx)) = (metrics_opt, pango_ctx_opt) {
                match keyval {
                    gtk4::gdk::Key::Left => InputHandler::move_cursor_left(&mut buf, &metrics.layout, &pango_ctx),
                    gtk4::gdk::Key::Right => InputHandler::move_cursor_right(&mut buf, &metrics.layout, &pango_ctx),
                    gtk4::gdk::Key::Up => InputHandler::move_cursor_up(&mut buf, &metrics.layout, &pango_ctx),
                    gtk4::gdk::Key::Down => InputHandler::move_cursor_down(&mut buf, &metrics.layout, &pango_ctx),
                    _ => {}
                }
            }
        } else if let Some(text_char) = keyval.to_unicode() {
            // Fallback for character input
            if text_char.is_ascii_graphic() || text_char.is_whitespace() {
                buf.handle_text_input(&text_char.to_string());
            }
        }
    }
}
