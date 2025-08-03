//! Input handling for the EditorWidget
//! Handles keyboard input, cursor movement, and text insertion

use gtk4::gdk::Key;
use crate::corelogic::EditorBuffer;

/// Input handling utilities for the editor
pub struct InputHandler;

impl InputHandler {
    /// Ensure cursor is always valid after buffer changes
    pub fn ensure_cursor_valid(buf: &mut EditorBuffer) {
        if buf.lines.is_empty() {
            buf.cursor.row = 0;
            buf.cursor.col = 0;
        } else {
            if buf.cursor.row >= buf.lines.len() {
                buf.cursor.row = buf.lines.len() - 1;
            }
            buf.cursor.col = buf.cursor.col.min(buf.lines[buf.cursor.row].len());
        }
    }

    /// Move cursor left
    pub fn move_cursor_left(buf: &mut EditorBuffer) {
        if buf.lines.is_empty() {
            buf.cursor.row = 0;
            buf.cursor.col = 0;
        } else if buf.cursor.col > 0 {
            buf.cursor.col -= 1;
        } else if buf.cursor.row > 0 {
            buf.cursor.row -= 1;
            buf.cursor.col = buf.lines[buf.cursor.row].len();
        }
        Self::ensure_cursor_valid(buf);
        buf.request_redraw();
    }

    /// Move cursor right
    pub fn move_cursor_right(buf: &mut EditorBuffer) {
        if buf.lines.is_empty() {
            buf.cursor.row = 0;
            buf.cursor.col = 0;
        } else if buf.cursor.col < buf.lines[buf.cursor.row].len() {
            buf.cursor.col += 1;
        } else if buf.cursor.row + 1 < buf.lines.len() {
            buf.cursor.row += 1;
            buf.cursor.col = 0;
        }
        Self::ensure_cursor_valid(buf);
        buf.request_redraw();
    }

    /// Move cursor up
    pub fn move_cursor_up(buf: &mut EditorBuffer) {
        if buf.lines.is_empty() {
            buf.cursor.row = 0;
            buf.cursor.col = 0;
        } else if buf.cursor.row > 0 {
            buf.cursor.row -= 1;
            buf.cursor.col = buf.cursor.col.min(buf.lines[buf.cursor.row].len());
        }
        Self::ensure_cursor_valid(buf);
        buf.request_redraw();
    }

    /// Move cursor down
    pub fn move_cursor_down(buf: &mut EditorBuffer) {
        if buf.lines.is_empty() {
            buf.cursor.row = 0;
            buf.cursor.col = 0;
        } else if buf.cursor.row + 1 < buf.lines.len() {
            buf.cursor.row += 1;
            buf.cursor.col = buf.cursor.col.min(buf.lines[buf.cursor.row].len());
        }
        Self::ensure_cursor_valid(buf);
        buf.request_redraw();
    }

    /// Insert a character at the cursor
    pub fn insert_char(buf: &mut EditorBuffer, c: char) {
        if buf.lines.is_empty() {
            buf.lines.push(String::new());
            buf.cursor.row = 0;
            buf.cursor.col = 0;
        }
        let row = buf.cursor.row;
        let col = buf.cursor.col;
        if row < buf.lines.len() {
            let line = &mut buf.lines[row];
            let byte_idx = line.char_indices().nth(col).map(|(i, _)| i).unwrap_or(line.len());
            line.insert(byte_idx, c);
            buf.cursor.col += 1;
        }
        Self::ensure_cursor_valid(buf);
        buf.request_redraw();
    }

    /// Insert newline at cursor
    pub fn insert_newline(buf: &mut EditorBuffer) {
        if buf.lines.is_empty() {
            buf.lines.push(String::new());
            buf.cursor.row = 0;
            buf.cursor.col = 0;
        }
        let row = buf.cursor.row;
        let col = buf.cursor.col;
        if row < buf.lines.len() {
            let rest = buf.lines[row].split_off(col);
            buf.lines.insert(row + 1, rest);
            buf.cursor.row += 1;
            buf.cursor.col = 0;
        }
        Self::ensure_cursor_valid(buf);
        buf.request_redraw();
    }

    /// Handle a key event
    pub fn handle_key_event(buf: &mut EditorBuffer, keyval: Key) {
        match keyval {
            Key::Left => Self::move_cursor_left(buf),
            Key::Right => Self::move_cursor_right(buf),
            Key::Up => Self::move_cursor_up(buf),
            Key::Down => Self::move_cursor_down(buf),
            Key::BackSpace => {
                buf.backspace();
                buf.request_redraw();
            },
            Key::Delete => {
                buf.delete();
                buf.request_redraw();
            },
            Key::Return => Self::insert_newline(buf),
            _ => {
                if let Some(c) = keyval.to_unicode() {
                    Self::insert_char(buf, c);
                }
            }
        }
    }
}
