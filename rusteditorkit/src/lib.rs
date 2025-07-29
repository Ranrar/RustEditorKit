// Editor logic library entry point
// Exposes all editor modules for use as a library

pub mod core;
pub mod multicursor;
pub mod mouse_selection;
pub mod undo;
pub mod clipboard;
pub mod navigation;
pub mod selection;
pub mod fileio;
pub mod editing;
pub mod search;
pub mod bracket;
pub mod indent;
pub mod a4;
pub mod render;
pub mod editorwidget;
pub mod imcontext;

pub mod cursor;

// Example: re-export EditorBuffer for external use
pub use core::EditorBuffer;
