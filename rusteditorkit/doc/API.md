# RustEditorKit API Reference

RustEditorKit provides a comprehensive API for building text editors with GTK4. The API is designed for both standalone use and GTK4 widget integration.

## Core Types

### `EditorBuffer`
The central data structure containing text content, cursor position, selection state, undo history, and configuration.

```rust
pub struct EditorBuffer {
    pub lines: Vec<String>,
    pub cursor: EditorCursor,
    pub selection: Option<Selection>,
    pub config: EditorConfig,
    // ... internal fields
}
```


### `ConfigurableSize` Trait & `SizeMode` Enum

Flexible trait-based API for widget sizing and focus management. Use this to configure GTK4 widgets with various sizing strategies.

```rust
use rusteditorkit::widget::ConfigurableSize;
use rusteditorkit::corelogic::SizeMode;
let drawing_area = gtk::DrawingArea::new();
// Minimum size, expandable
drawing_area.configure_size(SizeMode::Minimum(400, 300));
// Switch to aspect ratio mode at runtime
drawing_area.switch_mode(SizeMode::AspectRatio(16.0/9.0));
```

#### SizeMode Variants
- `Fixed(w, h)`: Exact width/height, no expansion
- `Minimum(w, h)`: Minimum width/height, expandable
- `Dynamic`: No constraints, fully expandable
- `Maximum(w, h)`: Max width/height, clamps on size change
- `MinMax { min_w, min_h, max_w, max_h }`: Min/max bounds
- `AspectRatio(ratio)`: Maintains width/height ratio
- `AutoFitToParent`: Matches parent size
- `PercentOfParent(wp, hp)`: Percent of parent size
- `ContainerAware(mode)`: Adapts to container type

### `Selection`
Represents text selection with start and end coordinates, supporting both single-line and multi-line selections.

```rust
pub struct Selection {
    pub start_row: usize,
    pub start_col: usize, 
    pub end_row: usize,
    pub end_col: usize,
}
```

### `EditorConfig`
Configuration structure loaded from RON files, controlling appearance, behavior, and theming.

## API Categories

### Text Editing

| Function | Description | Example |
|----------|-------------|---------|
| `insert_text(text: &str)` | Insert text at cursor | `buffer.insert_text("Hello")` |
| `insert_newline()` | Insert line break | `buffer.insert_newline()` |
| `backspace()` | Delete character before cursor | `buffer.backspace()` |
| `delete()` | Delete character at cursor | `buffer.delete()` |
| `delete_line()` | Delete current line | `buffer.delete_line()` |
| `duplicate_line()` | Duplicate current line | `buffer.duplicate_line()` |

### Navigation

| Function | Description | Example |
|----------|-------------|---------|
| `move_left()` | Move cursor left | `buffer.move_left()` |
| `move_right()` | Move cursor right | `buffer.move_right()` |
| `move_up()` | Move cursor up | `buffer.move_up()` |
| `move_down()` | Move cursor down | `buffer.move_down()` |
| `move_home()` | Move to line start | `buffer.move_home()` |
| `move_end()` | Move to line end | `buffer.move_end()` |
| `move_page_up(lines)` | Move up by page | `buffer.move_page_up(20)` |
| `move_page_down(lines)` | Move down by page | `buffer.move_page_down(20)` |

### Selection

| Function | Description | Example |
|----------|-------------|---------|
| `select_all()` | Select all text | `buffer.select_all()` |
| `select_left()` | Extend selection left | `buffer.select_left()` |
| `select_right()` | Extend selection right | `buffer.select_right()` |
| `select_up()` | Extend selection up | `buffer.select_up()` |
| `select_down()` | Extend selection down | `buffer.select_down()` |
| `clear_selection()` | Clear current selection | `buffer.clear_selection()` |
| `get_selected_text()` | Get selected text | `let text = buffer.get_selected_text()` |
| `delete_selection()` | Delete selected text | `buffer.delete_selection()` |

### Clipboard Operations

| Function | Description | Example |
|----------|-------------|---------|
| `copy_to_clipboard()` | Copy selection to clipboard | `buffer.copy_to_clipboard()` |
| `cut_to_clipboard()` | Cut selection to clipboard | `buffer.cut_to_clipboard()` |
| `paste_text(text: &str)` | Paste text at cursor | `buffer.paste_text("text")` |
| `copy()` | Get text to copy | `let text = buffer.copy()` |

### Undo/Redo

| Function | Description | Example |
|----------|-------------|---------|
| `undo()` | Undo last action | `buffer.undo()` |
| `redo()` | Redo last undone action | `buffer.redo()` |
| `push_undo()` | Save current state | `buffer.push_undo()` |

### Mouse Interaction

| Function | Description | Example |
|----------|-------------|---------|
| `handle_mouse_click(x, y, shift, ...)` | Handle mouse click | `buffer.handle_mouse_click(x, y, false, ...)` |
| `handle_mouse_drag(x, y, ...)` | Handle mouse drag | `buffer.handle_mouse_drag(x, y, ...)` |
| `handle_double_click(x, y, ...)` | Handle double-click (word selection) | `buffer.handle_double_click(x, y, ...)` |
| `handle_triple_click(x, y, ...)` | Handle triple-click (line selection) | `buffer.handle_triple_click(x, y, ...)` |
| `handle_mouse_release()` | Handle mouse release | `buffer.handle_mouse_release()` |

### Search and Replace

| Function | Description | Example |
|----------|-------------|---------|
| `find_next(pattern, start)` | Find next occurrence | `buffer.find_next("text", None)` |
| `replace_next(pattern, replacement, start)` | Replace next occurrence | `buffer.replace_next("old", "new", None)` |

### Indentation

| Function | Description | Example |
|----------|-------------|---------|
| `indent()` | Indent current line/selection | `buffer.indent()` |
| `unindent()` | Unindent current line/selection | `buffer.unindent()` |

### Rendering Control

| Function | Description | Example |
|----------|-------------|---------|
| `request_redraw()` | Request screen redraw | `buffer.request_redraw()` |

## Usage Patterns

### Basic Editor Setup

```rust
use rusteditorkit::{EditorBuffer, EditorConfig};

// Create buffer with default config
let mut buffer = EditorBuffer::new();

// Add some text
buffer.insert_text("Hello, world!");
buffer.insert_newline();
buffer.insert_text("Second line");

// Navigate and select
buffer.move_home();
buffer.select_all();
buffer.copy_to_clipboard();
```

### GTK4 Widget Integration

```rust
use gtk4::prelude::*;
use rusteditorkit::widget::editor::EditorWidget;

fn create_editor_window() {
    let editor = EditorWidget::new();
    editor.connect_signals();
    
    let window = ApplicationWindow::builder()
        .title("Text Editor")
        .default_width(800)
        .default_height(600)
        .child(editor.widget())
        .build();
        
    window.show();
}
```

### Configuration Loading

```rust
use rusteditorkit::api::config_loader::{load_config, default_config_path};

// Load config from file
let config_path = default_config_path();
match load_config(&config_path) {
    Ok(config) => {
        let mut buffer = EditorBuffer::with_config(config);
        // Use configured buffer
    }
    Err(e) => eprintln!("Config error: {}", e),
}
```

### Mouse Selection Handling

```rust
// In your GTK4 event handler
let gesture = gtk4::GestureClick::new();
gesture.connect_pressed(move |_, n_press, x, y| {
    match n_press {
        1 => buffer.handle_mouse_click(x, y, false, line_height, char_width, left_margin, top_margin),
        2 => buffer.handle_double_click(x, y, line_height, char_width, left_margin, top_margin),
        3 => buffer.handle_triple_click(x, y, line_height, char_width, left_margin, top_margin),
        _ => {}
    }
});
```

## Unicode Support

RustEditorKit has comprehensive Unicode support with character-safe operations:

```rust
// Safe with Unicode text like "é ü ñ å 漢字"
buffer.insert_text("Café münü niño");
buffer.select_all();
buffer.copy_to_clipboard(); // No panics on UTF-8 boundaries
```

All string operations use character indices rather than byte indices, ensuring safe handling of multi-byte UTF-8 sequences.

## Error Handling

Most operations are designed to be safe and handle edge cases gracefully:

```rust
// Safe operations that won't panic
buffer.move_left();  // Does nothing if already at start
buffer.move_up();    // Does nothing if already at top
buffer.delete();     // Does nothing if at end of text
buffer.undo();       // Does nothing if no undo history
```

## Configuration API

### Loading Configuration

```rust
use rusteditorkit::api::config_loader::{default_config_path, config_path_from_env, load_config};

// Get default config path
let default_path = default_config_path();

// Check for environment override
let config_path = config_path_from_env();

// Load configuration
match load_config(&config_path) {
    Ok(config) => {
        println!("Loaded font: {}", config.font);
        // Use config...
    }
    Err(e) => eprintln!("Failed to load config: {}", e),
}
```

### Configuration Structure

The `EditorConfig` structure controls all aspects of editor appearance and behavior:

```rust
pub struct EditorConfig {
    // Font settings
    pub font: String,
    pub font_size: f64,
    pub line_height: f64,
    
    // Colors
    pub fg_color: String,
    pub bg_color: String,
    pub cursor_color: String,
    pub selection_bg_color: String,
    
    // Gutter
    pub gutter_width: f64,
    pub gutter_color: String,
    pub line_number_color: String,
    
    // Features
    pub syntax_highlighting: bool,
    pub show_line_numbers: bool,
    pub highlight_current_line: bool,
    
    // And many more...
}
```

See the [EditorWidget documentation](./EditorWidget.md) for RON configuration file format and examples.