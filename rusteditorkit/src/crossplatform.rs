// --- Keybinding API ---

mod keybinds_mods {
    pub use crate::keybinds::editor_action::{EditorAction, KeyCombo};
    #[cfg(target_os = "linux")] pub use crate::keybinds::linux::linux_keymap;
    #[cfg(target_os = "windows")] pub use crate::keybinds::win::win_keymap;
    #[cfg(target_os = "macos")] pub use crate::keybinds::mac::mac_keymap;
}
use keybinds_mods::*;

use std::collections::HashMap;

pub mod keybinds {
    use super::*;
    /// Returns the keybinding map for the current platform
    pub fn keymap() -> HashMap<EditorAction, KeyCombo> {
        #[cfg(target_os = "linux")] { linux_keymap() }
        #[cfg(target_os = "windows")] { win_keymap() }
        #[cfg(target_os = "macos")] { mac_keymap() }
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))] { HashMap::new() }
    }

    /// Get the key combo for a given action, if any
    pub fn get_keybinding(action: EditorAction) -> Option<KeyCombo> {
        keymap().get(&action).cloned()
    }
}
/// Open a file and return its contents as Vec<String> (lines)
pub fn open_file(path: &str) -> Result<Vec<String>, String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match reader.lines().collect::<Result<_, _>>() {
                Ok(lines) => Ok(lines),
                Err(e) => Err(format!("Failed to read file: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to open file: {}", e)),
    }
}

/// Save lines to a file
pub fn save_file(path: &str, lines: &[String]) -> Result<(), String> {
    use std::fs::File;
    use std::io::{Write};
    match File::create(path) {
        Ok(mut file) => {
            for (i, line) in lines.iter().enumerate() {
                if i > 0 {
                    if let Err(e) = writeln!(file) {
                        return Err(format!("Failed to write newline: {}", e));
                    }
                }
                if let Err(e) = write!(file, "{}", line) {
                    return Err(format!("Failed to write line: {}", e));
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("Failed to create file: {}", e)),
    }
}
// Cross-platform font and file operations for RustEditorKit
// Supports Linux, Windows, and macOS

#[derive(Debug, Clone)]
pub struct FontHandle {
    pub name: String,
    pub path: Option<String>,
}

/// List available system fonts (cross-platform)
pub fn list_fonts() -> Vec<String> {
    #[cfg(target_os = "linux")]
    {
        // Use fc-list (fontconfig) for Linux
        use std::process::Command;
        let output = Command::new("fc-list").arg(":family").output();
        if let Ok(out) = output {
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect()
        } else {
            vec![]
        }
    }
    #[cfg(target_os = "windows")]
    {
        // Use Windows font directory
        let font_dir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string()) + "\\Fonts";
        match std::fs::read_dir(&font_dir) {
            Ok(entries) => entries.filter_map(|e| {
                e.ok().and_then(|entry| entry.file_name().into_string().ok())
            }).collect(),
            Err(_) => vec![],
        }
    }
    #[cfg(target_os = "macos")]
    {
        // Use macOS font directories
        let dirs = ["/System/Library/Fonts", "/Library/Fonts", &format!("{}/Library/Fonts", std::env::var("HOME").unwrap_or_default())];
        let mut fonts = vec![];
        for dir in dirs.iter() {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    if let Ok(name) = entry.file_name().into_string() {
                        fonts.push(name);
                    }
                }
            }
        }
        fonts
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        vec![]
    }
}

/// Find a font by name (returns FontHandle if found)
pub fn find_font(name: &str) -> Option<FontHandle> {
    let fonts = list_fonts();
    for font in fonts {
        if font.to_lowercase().contains(&name.to_lowercase()) {
            return Some(FontHandle { name: font, path: None });
        }
    }
    None
}

/// Open a font file using font-kit for cross-platform support
pub fn open_font(path: &str) -> Result<FontHandle, String> {
    use font_kit::sources::fs::FsSource;
    use font_kit::handle::Handle;
    use std::path::Path;
    let fs_source = FsSource::new();
    let font_path = Path::new(path);
    if !font_path.exists() {
        return Err(format!("Font file not found: {}", path));
    }
    match fs_source.select_best_match(&[], &font_kit::properties::Properties::new()) {
        Ok(handle) => {
            match handle {
                Handle::Path { path: font_file, .. } => {
                    if font_file == font_path {
                        Ok(FontHandle { name: font_file.display().to_string(), path: Some(font_file.display().to_string()) })
                    } else {
                        Err(format!("Font file mismatch: {}", path))
                    }
                },
                _ => Err(format!("Font handle not a file: {}", path)),
            }
        },
        Err(e) => Err(format!("Failed to open font: {}", e)),
    }
}

/// Close a font handle (noop, included for API consistency)
/// Font resources are managed by Rust and font-kit; no explicit close needed.
pub fn close_font(_handle: FontHandle) -> Result<(), String> {
    Ok(())
}

// Example usage:
// let fonts = list_fonts();
// let handle = find_font("Fira Mono");
// let opened = open_font("/path/to/font.ttf");
// close_font(handle.unwrap());
