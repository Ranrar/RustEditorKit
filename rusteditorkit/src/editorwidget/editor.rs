//! Public EditorWidget for RustEditorKit
//! Encapsulates EditorBuffer, input handling, and rendering logic.

use gtk4::prelude::*;
use gtk4::DrawingArea;
use glib::Propagation;
use pangocairo;
 // Removed unused Inhibit import
use std::cell::RefCell;
use std::rc::Rc;
use crate::core::EditorBuffer;
use crate::imcontext::EditorIMContext;

/// Main editor widget struct
pub struct EditorWidget {
    buffer: Rc<RefCell<EditorBuffer>>,
    drawing_area: DrawingArea,
    im_context: EditorIMContext,
}

impl EditorWidget {
    /// Get a reference to the buffer (for integration/testing)
    pub fn buffer(&self) -> Rc<RefCell<EditorBuffer>> {
        self.buffer.clone()
    }
    /// Create a new EditorWidget
    pub fn new() -> Self {
        let buffer = Rc::new(RefCell::new(EditorBuffer::new()));
        let drawing_area = DrawingArea::new();
        // Set redraw callback so buffer.request_redraw() triggers UI update
        {
            let da_clone = drawing_area.clone();
            buffer.borrow_mut().redraw_callback = Some(Box::new(move || {
                da_clone.queue_draw();
            }));
        }
        drawing_area.set_focusable(true);
        // Ensure the DrawingArea is large enough for text input/rendering
        drawing_area.set_content_width(400);
        drawing_area.set_content_height(300);
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

        // IMContext integration
        let buffer_clone = buffer.clone();
        let im_context = EditorIMContext::new(move |text| {
            println!("IMContext commit: {}", text);
            let mut buf = buffer_clone.borrow_mut();
            for c in text.chars() {
                let row = buf.cursor.row;
                let col = buf.cursor.col;
                if row < buf.lines.len() {
                    let line = &mut buf.lines[row];
                    let byte_idx = line.char_indices().nth(col).map(|(i, _)| i).unwrap_or(line.len());
                    line.insert(byte_idx, c);
                    buf.cursor.col += 1;
                }
            }
            buf.request_redraw();
        });

        Self { buffer, drawing_area, im_context }
    }

    /// Get the GTK4 widget for integration
    pub fn widget(&self) -> &DrawingArea {
        &self.drawing_area
    }

    /// Example: Move cursor left
    pub fn move_cursor_left(&self) {
        let mut buf = self.buffer.borrow_mut();
        if buf.cursor.col > 0 {
            buf.cursor.col -= 1;
        } else if buf.cursor.row > 0 {
            buf.cursor.row -= 1;
            buf.cursor.col = buf.lines[buf.cursor.row].len();
        }
        buf.request_redraw();
    }

    /// Example: Insert a character at the cursor
    pub fn insert_char(&self, c: char) {
        let mut buf = self.buffer.borrow_mut();
        let row = buf.cursor.row;
        let col = buf.cursor.col;
        if row < buf.lines.len() {
            let line = &mut buf.lines[row];
            let byte_idx = line.char_indices().nth(col).map(|(i, _)| i).unwrap_or(line.len());
            line.insert(byte_idx, c);
            buf.cursor.col += 1;
            buf.request_redraw();
        }
    }

    /// Move cursor right
    pub fn move_cursor_right(&self) {
        let mut buf = self.buffer.borrow_mut();
        if buf.cursor.col < buf.lines[buf.cursor.row].len() {
            buf.cursor.col += 1;
        } else if buf.cursor.row + 1 < buf.lines.len() {
            buf.cursor.row += 1;
            buf.cursor.col = 0;
        }
        buf.request_redraw();
    }

    /// Move cursor up
    pub fn move_cursor_up(&self) {
        let mut buf = self.buffer.borrow_mut();
        if buf.cursor.row > 0 {
            buf.cursor.row -= 1;
            buf.cursor.col = buf.cursor.col.min(buf.lines[buf.cursor.row].len());
        }
        buf.request_redraw();
    }

    /// Move cursor down
    pub fn move_cursor_down(&self) {
        let mut buf = self.buffer.borrow_mut();
        if buf.cursor.row + 1 < buf.lines.len() {
            buf.cursor.row += 1;
            buf.cursor.col = buf.cursor.col.min(buf.lines[buf.cursor.row].len());
        }
        buf.request_redraw();
    }

    /// Delete character before cursor
    pub fn delete_char(&self) {
        let mut buf = self.buffer.borrow_mut();
        let row = buf.cursor.row;
        let col = buf.cursor.col;
        if col > 0 {
            buf.lines[row].remove(col - 1);
            buf.cursor.col -= 1;
        } else if row > 0 {
            let prev_len = buf.lines[row - 1].len();
            let removed = buf.lines.remove(row);
            buf.lines[row - 1].push_str(&removed);
            buf.cursor.row -= 1;
            buf.cursor.col = prev_len;
        }
        buf.request_redraw();
    }

    /// Insert newline at cursor
    pub fn insert_newline(&self) {
        let mut buf = self.buffer.borrow_mut();
        let row = buf.cursor.row;
        let col = buf.cursor.col;
        if row < buf.lines.len() {
            let rest = buf.lines[row].split_off(col);
            buf.lines.insert(row + 1, rest);
            buf.cursor.row += 1;
            buf.cursor.col = 0;
            buf.request_redraw();
        }
    }

    /// Handle a key event (for integration)
    pub fn handle_key_event(&self, keyval: gtk4::gdk::Key) {
        match keyval {
            gtk4::gdk::Key::Left => self.move_cursor_left(),
            gtk4::gdk::Key::Right => self.move_cursor_right(),
            gtk4::gdk::Key::Up => self.move_cursor_up(),
            gtk4::gdk::Key::Down => self.move_cursor_down(),
            gtk4::gdk::Key::BackSpace => self.delete_char(),
            gtk4::gdk::Key::Return => self.insert_newline(),
            _ => {
                if let Some(c) = keyval.to_unicode() {
                    self.insert_char(c);
                }
            }
        }
    }

    /// Connect draw and input signals (call in new)
    pub fn connect_signals(&self) {
        let buffer = self.buffer.clone();
        let im_context_draw = self.im_context.im_context.clone();
        self.drawing_area.set_draw_func(move |_area, ctx, width, height| {
            let buf = buffer.borrow();
            crate::editorwidget::render::render_editor(&buf, ctx, width, height);
            // Calculate cursor pixel position (fixed metrics: 16px/char, 20px/line)
            let char_width = 16;
            let line_height = 20;
            let cursor_x = (buf.cursor.col as i32) * char_width;
            let cursor_y = (buf.cursor.row as i32) * line_height;
            let rect = gtk4::gdk::Rectangle::new(cursor_x, cursor_y, char_width, line_height);
            im_context_draw.set_cursor_location(&rect);

            // Focus indicator: draw border if focused
            if _area.has_focus() {
                println!("DrawingArea is focused");
                ctx.set_source_rgba(0.2, 0.6, 1.0, 1.0); // blue border
                ctx.set_line_width(3.0);
                ctx.rectangle(1.5, 1.5, width as f64 - 3.0, height as f64 - 3.0);
                ctx.stroke().unwrap_or(());
            }

            // Debug overlay: show cursor position and current line
            ctx.set_source_rgba(0.0, 0.0, 0.0, 0.7); // semi-transparent black
            ctx.rectangle(0.0, 0.0, 320.0, 40.0);
            ctx.fill().unwrap_or(());
            ctx.set_source_rgba(1.0, 1.0, 1.0, 1.0); // white text
            let debug_text = format!("Cursor: row={}, col={} | Line: {}",
                buf.cursor.row,
                buf.cursor.col,
                buf.lines.get(buf.cursor.row).unwrap_or(&"".to_string())
            );
            let pango_ctx = _area.create_pango_context();
            let layout = gtk4::pango::Layout::new(&pango_ctx);
            layout.set_text(&debug_text);
            layout.set_font_description(Some(&gtk4::pango::FontDescription::from_string("Fira Mono 12")));
            ctx.move_to(8.0, 8.0);
            pangocairo::functions::show_layout(ctx, &layout);
        });

        // Notify IMContext of focus changes
        let focus_controller = gtk4::EventControllerFocus::new();
        let im_context_enter = self.im_context.im_context.clone();
        focus_controller.connect_enter(move |_| {
            println!("DrawingArea focus ENTER");
            im_context_enter.focus_in();
        });
        let im_context_leave = self.im_context.im_context.clone();
        focus_controller.connect_leave(move |_| {
            println!("DrawingArea focus LEAVE");
            im_context_leave.focus_out();
        });
        self.drawing_area.add_controller(focus_controller);

        let key_controller = gtk4::EventControllerKey::new();
        let im_context_key = self.im_context.im_context.clone();
        let buffer = self.buffer.clone();
        key_controller.connect_key_pressed(move |_controller, keyval, _keycode, _state| {
            println!("Key pressed: {:?}", keyval);
            if let Some(event) = _controller.current_event() {
                if im_context_key.filter_keypress(&event) {
                    println!("IMContext filtered keypress");
                    return Propagation::Stop;
                }
            }
            // Handle navigation/editing keys
            let mut buf = buffer.borrow_mut();
            match keyval {
                gtk4::gdk::Key::Left => { buf.move_left(); buf.request_redraw(); },
                gtk4::gdk::Key::Right => { buf.move_right(); buf.request_redraw(); },
                gtk4::gdk::Key::Up => { buf.move_up(); buf.request_redraw(); },
                gtk4::gdk::Key::Down => { buf.move_down(); buf.request_redraw(); },
                gtk4::gdk::Key::BackSpace => { buf.backspace(); buf.request_redraw(); },
                gtk4::gdk::Key::Delete => { buf.delete(); buf.request_redraw(); },
                gtk4::gdk::Key::Return => {
                    let row = buf.cursor.row;
                    let col = buf.cursor.col;
                    if row < buf.lines.len() {
                        let rest = buf.lines[row].split_off(col);
                        buf.lines.insert(row + 1, rest);
                        buf.cursor.row += 1;
                        buf.cursor.col = 0;
                        buf.request_redraw();
                    }
                },
                _ => {
                    if let Some(c) = keyval.to_unicode() {
                        let row = buf.cursor.row;
                        let col = buf.cursor.col;
                        if row < buf.lines.len() {
                            let line = &mut buf.lines[row];
                            let byte_idx = line.char_indices().nth(col).map(|(i, _)| i).unwrap_or(line.len());
                            line.insert(byte_idx, c);
                            buf.cursor.col += 1;
                            buf.request_redraw();
                        }
                    }
                }
            }
            Propagation::Stop
        });
        self.drawing_area.add_controller(key_controller);
    }
}
