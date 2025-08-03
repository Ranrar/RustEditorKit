//! Configuration management for the EditorWidget
//! Handles loading and applying config files

use std::cell::RefCell;
use std::rc::Rc;
use crate::corelogic::EditorBuffer;

/// Configuration management utilities for the editor
pub struct ConfigManager;

impl ConfigManager {
    /// Load and apply config from a RON file to the editor buffer, handling errors gracefully
    pub fn load_config_from_file(buffer: &Rc<RefCell<EditorBuffer>>, path: &str) {
        match crate::config::editor_config_loader::load_widget_config(path) {
            Ok(config) => {
                {
                    let mut buf = buffer.borrow_mut();
                    buf.apply_config(config);
                    if buf.debug_mode {
                        println!("[DEBUG] Config loaded successfully from '{}'.", path);
                    }
                    // Remove the first line if there is more than one line
                    if buf.lines.len() > 1 {
                        buf.lines.remove(0);
                    }
                }
                buffer.borrow().request_redraw();
            }
            Err(e) => {
                {
                    let mut buf = buffer.borrow_mut();
                    buf.lines.clear();
                    buf.lines.push(e.clone());
                    if buf.debug_mode {
                        println!("[DEBUG] Config load failed: {}", e);
                    }
                }
                buffer.borrow().request_redraw();
            }
        }
    }
}
