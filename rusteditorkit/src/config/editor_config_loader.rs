//! EditorWidget config loader for RustEditorKit
//! Loads config.ron and applies settings to EditorWidget/EditorBuffer

use std::fs::File;
use std::io::BufReader;
use ron::de::from_reader;

/// Load EditorConfig from a RON file
pub fn load_widget_config(path: &str) -> Result<crate::config::configuration::EditorConfig, String> {
    let file = File::open(path).map_err(|e| {
        println!("[CONFIG DEBUG] Could not open config file: {}", e);
        format!(
            "Config error: Could not open config file at '{}'.\nReason: {}\nSuggestion: Please check the file path and ensure the file exists.",
            path, e
        )
    })?;
    let reader = BufReader::new(file);
    match from_reader::<BufReader<File>, crate::config::configuration::EditorConfig>(reader) {
        Ok(cfg) => {
            println!("[CONFIG DEBUG] RON deserialization succeeded.");
            println!("[CONFIG DEBUG] Loaded CursorConfig: {:#?}", cfg.cursor);
            Ok(cfg)
        },
        Err(e) => {
            println!("[CONFIG DEBUG] RON deserialization failed: {}", e);
            Err(format!(
                "Config error: Failed to parse RON config at '{}'.\nReason: {}\nSuggestion: Please check the config file format and documentation.",
                path, e
            ))
        }
    }
}
