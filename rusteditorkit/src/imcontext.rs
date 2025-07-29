//! IMContextSimple wrapper for RustEditorKit
//! Handles input method composition and commit events for text input.

use gtk4::prelude::*;
use gtk4::IMContextSimple;
use std::rc::Rc;
use std::cell::RefCell;

pub struct EditorIMContext {
    pub im_context: IMContextSimple,
}

impl EditorIMContext {
    pub fn new(on_commit: impl Fn(String) + 'static) -> Self {
        let im_context = IMContextSimple::new();
        im_context.connect_commit(move |_, text| {
            on_commit(text.to_string());
        });
        Self { im_context }
    }
}
