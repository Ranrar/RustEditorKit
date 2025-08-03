/// Enum of all editor actions that can be triggered by keybindings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditorAction {
    // Navigation
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorUp,
    MoveCursorDown,
    MoveCursorStartOfLine,
    MoveCursorEndOfLine,
    MoveCursorHome,        // Alias for start of line
    MoveCursorEnd,         // Alias for end of line
    MoveCursorPageUp,
    MoveCursorPageDown,
    // Selection
    SelectLeft,
    SelectRight,
    SelectUp,
    SelectDown,
    SelectAll,
    // Editing
    CopySelection,
    CutSelection,
    PasteClipboard,
    DeleteLeft,
    DeleteRight,
    Backspace,             // Delete character before cursor
    Delete,                // Delete character at cursor
    InsertText,            // Insert text at cursor
    InsertNewline,         // Insert newline
    Undo,
    Redo,
    // Indentation and Tabulation
    Indent,
    Unindent,
    ConvertTabsToSpaces,
    ToggleSoftTabs,
    // Escape and Cancel
    Escape,
    ClearSelection,
    ExitInsertMode,
    CloseOverlay,
    // File Operations
    NewFile,
    OpenFile,
    SaveFile,
    SaveAs,
    // Search & Replace
    Find,
    FindNext,
    Replace,
    // Layout and View
    ToggleA4Mode,          // Toggle A4 page mode
    // Multi-cursor
    AddCursor,             // Add cursor at position
}

/// Represents a key combination (key + modifiers)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyCombo {
    pub key: &'static str, // e.g. "Left", "Ctrl+C"
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

impl KeyCombo {
    /// Converts a GTK keyval and modifier state to a KeyCombo for keymap matching
    pub fn from_gtk_event(keyval: u32, state: gtk4::gdk::ModifierType) -> Self {
        // Recognize fallback keyvals for common keys
        let key = match keyval {
            0xff08 => "Backspace",
            0xff09 => "Tab",
            0xfe20 => "Tab", // ISO_Left_Tab (Shift+Tab) - treat as Tab, shift will be handled by modifier
            0xff0d => "Return",
            0xff1b => "Escape",
            0xff51 => "Left",
            0xff52 => "Up",
            0xff53 => "Right",
            0xff54 => "Down",
            0xffff => "Delete",
            0xff50 => "Home",
            0xff57 => "End",
            0xff55 => "PageUp",
            0xff56 => "PageDown",
            _ => {
                let key_cstr = unsafe { gdk4_sys::gdk_keyval_name(keyval) };
                if !key_cstr.is_null() {
                    unsafe { std::ffi::CStr::from_ptr(key_cstr).to_str().unwrap_or("") }
                } else {
                    ""
                }
            }
        };
        let ctrl = state.contains(gtk4::gdk::ModifierType::CONTROL_MASK);
        let shift = state.contains(gtk4::gdk::ModifierType::SHIFT_MASK);
        let alt = state.contains(gtk4::gdk::ModifierType::ALT_MASK);
        Self { key, ctrl, shift, alt }
    }
    pub const fn new(key: &'static str, ctrl: bool, shift: bool, alt: bool) -> Self {
        Self { key, ctrl, shift, alt }
    }
}
