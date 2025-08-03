//! Core Logic Module for RustEditorKit
//! 
//! This module contains the centralized editor logic organized by functionality.
//! All EditorBuffer implementations are consolidated here for better maintainability.

pub mod buffer;
pub mod editing;
pub mod font;
pub mod cursor;
pub mod gutter;
pub mod undo;
pub mod clipboard;
pub mod search;
pub mod fileio;
pub mod selection;
// pub mod layout;  // Temporarily disabled - needs config updates
pub mod dispatcher;

// Re-export the main types for convenience
pub use buffer::{EditorBuffer, EditorCursor};
pub use cursor::*;
pub use selection::Selection;
pub use undo::*;
pub use search::*;
pub use fileio::*;
// pub use layout::*;  // Temporarily disabled
pub use dispatcher::*;
