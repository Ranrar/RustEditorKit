# EditorWidget - GTK4 Integration Guide

`EditorWidget` is RustEditorKit's main GTK4 component, providing a complete text editor widget ready for integration into GTK4 applications. It encapsulates all editor functionality with configuration via RON files.

## Quick Start

### 1. Basic Integration

```rust
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use rusteditorkit::widget::editor::EditorWidget;

fn main() {
    let app = Application::builder()
        .application_id("com.example.editor")View
        .build();

    app.connect_activate(|app| {
        // Create editor widget
        let editor = EditorWidget::new();
        editor.connect_signals();

        // Create window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Text Editor")
            .default_width(800)
            .default_height(600)
            .child(editor.widget())
            .build();

        window.show();
        editor.widget().grab_focus();
    });

    app.run();
}
```

### 2. Configuration Setup

Create a `config.ron` file in your project directory:

```ron
(
    // Font settings
    font: "Fira Mono",
    font_size: 14.0,
    line_height: 22.0,
    
    // Editor colors
    fg_color: "#222222",
    bg_color: "#f8f8ff", 
    cursor_color: "#0055aa",
    
    // Selection colors
    selection_bg_color: "#b3d7ff",
    selection_opacity: 0.3,
    
    // Gutter settings
    gutter_width: 60.0,
    gutter_color: "#e0e0e0",
    line_number_color: "#888888",
    selected_line_number_color: "#0055aa",
    show_line_numbers: true,
    
    // Visual features
    highlight_current_line: true,
    active_line_bg_color: "#f0f8ff",
    show_active_line_bg: true,
    
    // Syntax highlighting
    syntax_highlighting: true,
    
    // Margins
    margin_left: 8.0,
    margin_right: 8.0,
    margin_top: 4.0,
    margin_bottom: 4.0,
    
    // Advanced features
    show_whitespace_guides: false,
    whitespace_guide_color: "#e0e0e0",
    diagnostics_highlighting: true,
    error_color: "#ff3333",
    warning_color: "#ffaa00",
    
    // Search highlighting
    search_match_color: "#ffff99",
    
    // Markdown syntax coloring (if enabled)
    markdown_syntax_coloring: true,
    markdown_header_color: "#0055aa",
    markdown_bold_color: "#222222",
    markdown_italic_color: "#444444",
    markdown_code_color: "#e0e0e0",
    markdown_link_color: "#0055aa",
    markdown_quote_color: "#888888",
    markdown_list_color: "#0055aa",
    
    // Character spacing
    character_spacing: 0.0,
)
```

## Configuration Reference

### Required Configuration

The `EditorWidget` **requires** a valid `config.ron` file. All fields must be present:

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `font` | String | Font family name | `"Fira Mono"` |
| `font_size` | f64 | Font size in points | `14.0` |
| `line_height` | f64 | Line height in pixels | `22.0` |
| `fg_color` | String | Text color (hex) | `"#222222"` |
| `bg_color` | String | Background color (hex) | `"#f8f8ff"` |
| `cursor_color` | String | Cursor color (hex) | `"#0055aa"` |

### Optional Visual Features

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `selection_bg_color` | String | Selection background | `"#b3d7ff"` |
| `selection_opacity` | f64 | Selection transparency | `0.3` |
| `highlight_current_line` | bool | Highlight current line | `true` |
| `active_line_bg_color` | String | Current line background | `"#f0f8ff"` |
| `show_active_line_bg` | bool | Show current line highlight | `true` |

### Gutter Configuration

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `gutter_width` | f64 | Gutter width in pixels | `60.0` |
| `gutter_color` | String | Gutter background color | `"#e0e0e0"` |
| `line_number_color` | String | Line number color | `"#888888"` |
| `selected_line_number_color` | String | Current line number color | `"#0055aa"` |
| `show_line_numbers` | bool | Show line numbers | `true` |

## Advanced Usage

### Custom Configuration Loading

```rust
use rusteditorkit::widget::editor::EditorWidget;
use rusteditorkit::api::config_loader::load_config;

// Load custom config file
let config = load_config("custom_config.ron").expect("Failed to load config");
let editor = EditorWidget::with_config(config);
editor.connect_signals();
```

### Accessing the Buffer

```rust
let editor = EditorWidget::new();

// Access the underlying EditorBuffer
editor.with_buffer(|buffer| {
    buffer.insert_text("Hello, world!");
    buffer.move_cursor_down(1);
    buffer.select_all();
});

// Or get a reference for multiple operations
let buffer = editor.buffer();
buffer.borrow_mut().insert_text("More text");
```

### Event Handling

The widget automatically handles:

- **Keyboard input**: All standard text editing operations
- **Mouse events**: Click positioning, selection, double/triple-click
- **Clipboard**: Copy (Ctrl+C), Cut (Ctrl+X), Paste (Ctrl+V)
- **Navigation**: Arrow keys, Home/End, Page Up/Down
- **Selection**: Shift+arrows, Ctrl+A (select all)
- **Undo/Redo**: Ctrl+Z, Ctrl+Y

### Custom Key Bindings

```rust
let editor = EditorWidget::new();

// Add custom key handling
editor.widget().connect_key_pressed(move |widget, keyval, keycode, state| {
    // Custom key handling logic
    match keyval {
        gdk::Key::F1 => {
            // Handle F1 key
            glib::Propagation::Stop
        }
        _ => glib::Propagation::Proceed
    }
});
```

## Integration Patterns

### Menu Integration

```rust
use gtk4::{gio, Menu, MenuBar};

fn create_menu_bar(editor: &EditorWidget) -> MenuBar {
    let menu = Menu::new();
    
    // File menu
    let file_menu = Menu::new();
    file_menu.append(Some("New"), Some("app.new"));
    file_menu.append(Some("Open"), Some("app.open"));
    file_menu.append(Some("Save"), Some("app.save"));
    
    // Edit menu  
    let edit_menu = Menu::new();
    edit_menu.append(Some("Copy"), Some("app.copy"));
    edit_menu.append(Some("Cut"), Some("app.cut"));
    edit_menu.append(Some("Paste"), Some("app.paste"));
    
    menu.append_submenu(Some("File"), &file_menu);
    menu.append_submenu(Some("Edit"), &edit_menu);
    
    MenuBar::from_model(Some(&menu))
}
```

### Status Bar Integration

```rust
use gtk4::{Box, Label, Statusbar};

fn create_status_bar(editor: &EditorWidget) -> Box {
    let status_box = Box::new(gtk4::Orientation::Horizontal, 0);
    let cursor_label = Label::new(Some("Line 1, Col 1"));
    
    // Update cursor position
    editor.connect_cursor_moved(move |row, col| {
        cursor_label.set_text(&format!("Line {}, Col {}", row + 1, col + 1));
    });
    
    status_box.append(&cursor_label);
    status_box
}
```

### Multiple Editor Tabs

```rust
use gtk4::{Notebook, ScrolledWindow};

fn create_tabbed_editor() -> Notebook {
    let notebook = Notebook::new();
    
    // Add first tab
    let editor1 = EditorWidget::new();
    editor1.connect_signals();
    
    let scroll1 = ScrolledWindow::builder()
        .child(editor1.widget())
        .build();
        
    notebook.append_page(&scroll1, Some(&Label::new(Some("Document 1"))));
    
    // Add more tabs as needed...
    
    notebook
}
```

## Theming and Customization

### Dark Theme Example

```ron
(
    font: "JetBrains Mono",
    font_size: 13.0,
    line_height: 20.0,
    
    // Dark theme colors
    fg_color: "#e0e0e0",
    bg_color: "#2b2b2b",
    cursor_color: "#ffffff",
    
    selection_bg_color: "#404040",
    selection_opacity: 0.4,
    
    gutter_color: "#333333", 
    line_number_color: "#666666",
    selected_line_number_color: "#ffffff",
    
    active_line_bg_color: "#3a3a3a",
    show_active_line_bg: true,
    
    // Syntax colors for dark theme
    syntax_highlighting: true,
    
    // Error/warning colors
    error_color: "#ff6b6b",
    warning_color: "#ffd93d",
    
    // ... other settings
)
```

### High Contrast Theme

```ron
(
    font: "Liberation Mono",
    font_size: 16.0,
    line_height: 24.0,
    
    // High contrast colors
    fg_color: "#000000",
    bg_color: "#ffffff", 
    cursor_color: "#ff0000",
    
    selection_bg_color: "#0000ff",
    selection_opacity: 0.2,
    
    gutter_color: "#f0f0f0",
    line_number_color: "#000000",
    
    // Accessibility features
    highlight_current_line: true,
    active_line_bg_color: "#ffffcc",
    
    // ... other settings
)
```

## Troubleshooting

### Common Issues

**Config file not found:**
```
Error: Failed to load config from 'config.ron': No such file or directory
```
*Solution:* Ensure `config.ron` exists in your project directory or specify the correct path.

**Missing config fields:**
```
Error: RON deserialization failed: missing field `font_size`
```
*Solution:* Add all required fields to your `config.ron` file.

**Invalid color format:**
```
Error: Invalid color format: should be #RRGGBB or #RRGGBBAA
```
*Solution:* Use valid hex color codes like `"#ff0000"` or `"#ff0000ff"`.

### Debugging

Enable debug output for troubleshooting:

```rust
// Enable keybind debugging
std::env::set_var("RUSTEDITORKIT_DEBUG_KEYBINDS", "1");

// Enable rendering debugging  
std::env::set_var("RUSTEDITORKIT_DEBUG_RENDER", "1");

let editor = EditorWidget::new();
```

## Performance Considerations

- **Large files**: The editor loads entire files into memory. For very large files (>100MB)
- **Rendering**: Text rendering is optimized with dirty region updates. Avoid unnecessary redraws.
- **Unicode**: All text operations are Unicode-safe but may be slower than byte-based operations.

## See Also

- **[API Reference](./API.md)** - Complete API documentation
- **[Roadmap](./roadmap.md)** - Planned features and development timeline
- **[RON Format](https://github.com/ron-rs/ron)** - Configuration file format documentation
