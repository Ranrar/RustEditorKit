// Editor logic library entry point
// Exposes all editor modules for use as a library

// Core logic modules (centralized)
pub mod corelogic;
pub mod utilits;

// UI and platform modules
pub mod keybinds;
pub use utilits::crossplatform;

pub mod widget;
pub mod imcontext;
pub use utilits::ui; // UI scheduler helpers (GLib main loop)


pub mod config {
    pub mod configuration;
    pub mod api_config_loader;
    pub mod editor_config_loader;
}

pub mod render;
pub use render::render_editor;

// Re-export the main types from the new centralized structure
pub use corelogic::{EditorBuffer, EditorCursor};
pub use corelogic::{CommandDispatcher, CommandError, CommandParams, CommandResult};
pub use config::configuration::EditorConfig;
