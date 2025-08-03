/// Returns the default config file path for RustEditorKit.
///
/// # Example
/// ```rust
/// let path = rusteditorkit::config::api_config_loader::default_config_path();
/// println!("Default config path: {}", path);
/// ```
pub fn default_config_path() -> &'static str {
    "src/config/config.ron"
}

/// Returns config file path, optionally overridden by the `RUSTEDITORKIT_CONFIG` environment variable.
///
/// # Example
/// ```rust
/// // Set env var to override config location
/// std::env::set_var("RUSTEDITORKIT_CONFIG", "/tmp/myconfig.ron");
/// let path = rusteditorkit::config::api_config_loader::config_path_from_env();
/// println!("Config path: {}", path);
/// ```
pub fn config_path_from_env() -> String {
    std::env::var("RUSTEDITORKIT_CONFIG").unwrap_or_else(|_| default_config_path().to_string())
}
/// API-only config loader for RustEditorKit
/// Example usage for non-GTK integrations

use std::fs::File;
use std::io::BufReader;
use ron::de::from_reader;
use crate::config::configuration::EditorConfig;

/// Load EditorConfig from a RON file, with robust error handling
pub fn load_config(path: &str) -> Result<EditorConfig, String> {
    let file = File::open(path).map_err(|e| {
        format!(
            "Config error: Could not open config file at '{}'.\nReason: {}\nSuggestion: Please check the file path and ensure the file exists.",
            path, e
        )
    })?;
    let reader = BufReader::new(file);
    from_reader(reader).map_err(|e| {
        format!(
            "Config error: Failed to parse RON config at '{}'.\nReason: {}\nSuggestion: Please check the config file format and documentation.",
            path, e
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_config() {
        let config = load_config("src/config/config.ron");
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.font.font_name, "Fira Mono");
        assert!(config.font.font_size > 0.0);
    }
}
