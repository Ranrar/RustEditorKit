// Editor logic library entry point
// Exposes all editor modules for use as a library

// Core logic modules (centralized)
pub mod corelogic;

// UI and platform modules
pub mod keybinds;
pub mod crossplatform;

pub mod widget;
pub mod imcontext;
pub mod ui; // UI scheduler helpers (GLib main loop)

// Legacy modules (will be deprecated)
pub mod core; // Legacy core, will be removed
pub mod multicursor; // Will be merged into corelogic
pub mod bracket; // Will be merged into corelogic
pub mod indent; // Will be merged into corelogic

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
