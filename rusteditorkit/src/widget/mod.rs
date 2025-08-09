//! Widget module for RustEditorKit
//! 
//! This module contains all widget-related functionality, separated into logical components:
//! - editor: Main EditorWidget implementation
//! - input: Input handling and key events
//! - focus: Focus management and controllers
//! - config: Configuration loading and management
//! - signals: Signal connections and event handling

pub mod editor;
pub mod input;
pub mod focus;
pub mod config;
pub mod signals;
pub mod pointer;

// Re-export the main EditorWidget for convenience
pub use editor::EditorWidget;
mod size_config_impl;
