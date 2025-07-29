
# RustEditorKit ia a modern `Rust editor toolkit` built for GTK-RS

## Motivation

I am developing this project because I am writing a Markdown editor (Marco) and had a lot of trouble with SourceView5 and customization in GTK4. SourceView5 is not built for gtk-rs, which made integration and customization difficult. My goal is to create a sourceview in Rust that works seamlessly with GTK4, is fully customizable, and is easy to use and implement. This project aims to provide a modern, flexible code editor component for GTK4 applications, written in safe and idiomatic Rust.


## Key Features

- Multi-cursor editing
- Undo/redo stack
- Margin and A4 page support
- Search and replace
- Clipboard integration
- Customizable themes and colors
- Line numbers, gutter, and highlight support
- Extensible modules for new features
- Plugin functionality is planned for future extensibility

## EditorWidget

`EditorWidget` is the main GTK4 widget for RustEditorKit. It encapsulates all editor logic, rendering, and input handling, making it easy to embed a fully-featured code editor in your GTK4 application.

**Usage Example:**

```rust
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use rusteditorkit::editorwidget::editor::EditorWidget;

fn main() {
    let app = Application::builder()
        .application_id("com.example.rusteditorkit.demo")
        .build();

    app.connect_activate(|app| {
        let editor = EditorWidget::new();
        editor.connect_signals();

        let window = ApplicationWindow::builder()
            .application(app)
            .title("RustEditorKit Demo")
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

## Why Rust?

Rust is chosen for its:
- Memory safety (no crashes from buffer overflows)
- High performance
- Modern tooling and package management
- Strong type system and concurrency support
- Excellent integration with GTK4 via gtk-rs

## Why GTK4?

GTK4 provides:
- Modern, cross-platform UI toolkit
- Native look and feel on Linux, Windows, and macOS
- Good Rust bindings (gtk-rs)
- Advanced graphics and input support
- Active development and community

## Customizability

- Themes: Easily change colors, fonts, and styles
- Keybindings: Planned support for custom shortcuts
- Extensible: Add new modules for features like completion, diagnostics, etc.
- Plugin system: Planned for future extensibility
- API designed for subclassing and integration

## Ease of Use

- Simple API for integration into GTK4 apps
- Modular design: only use what you need
- Well-documented public API
- Example code and usage patterns provided

## Target Audience

- Rust desktop app developers
- GTK4 users needing a code editor widget
- Anyone building code or text editors in Rust
- Projects migrating from C/SourceView5 to Rust/GTK4

## Project Status

- **Alpha**: Core features implemented, many advanced features planned or stubbed
- API and design may change as feedback is received

## Contribution

- Contributions, bug reports, and feature requests are welcome!
- See issues and TODOs in the codebase
- Open a pull request or discussion for ideas

## Comparison

| Feature                | RustEditorKit (Rust) | GtkSourceView (C) |
|------------------------|---------------------|-------------------|
| Language               | Rust                | C (GObject)       |
| GTK4 Support           | Yes (native)        | Partial/legacy    |
| Undo/Redo              | Yes                 | Yes               |
| Multi-cursor           | Yes                 | No                |
| Custom Themes          | Yes                 | Yes               |
| Extensible API         | Yes (Rust traits)   | Yes (GObject)     |
| Plugin System          | Planned             | No                |
| License                | MIT                 | LGPL              |

## License

- RustEditorKit: MIT License
- GtkSourceView: LGPL License

## Quick Start

Clone the repository and run the demo:

```bash
git clone https://github.com/Ranrar/RustEditorKit.git
cd RustEditorKit
cargo run --bin demo
```

## Functions List (Detailed Status)



### Cursor & Navigation
- [x] `move_page_up`, `move_page_down`
  **Usage Example (planned):**
  ```rust
  rkit.move_page_up(lines_per_page);
  rkit.move_page_down(lines_per_page);
  ```
- [x] Line/column movement
  **Usage Example (planned):**
  ```rust
  rkit.move_cursor_to(row, col);
  rkit.move_cursor_left();
  rkit.move_cursor_right();
  rkit.move_cursor_up();
  rkit.move_cursor_down();
  ```
- [x] Multi-cursor support
  **Usage Example (planned):**
  ```rust
  rkit.add_cursor(row, col);
  rkit.remove_cursor(index);
  rkit.clear_cursors();
  ```



### Selection
- [x] `select_all`
  **Usage Example (planned):**
  ```rust
  rkit.select_all();
  ```
- [x] Multi-selection support
  **Usage Example (planned):**
  ```rust
  rkit.add_selection(start, end);
  rkit.clear_selections();
  ```
- [x] Selection start/end logic
  **Usage Example (planned):**
  ```rust
  rkit.set_selection_start(row, col);
  rkit.set_selection_end(row, col);
  ```



### Undo/Redo
- [x] Undo stack
  **Usage Example (planned):**
  ```rust
  rkit.undo();
  ```
- [x] Redo stack
  **Usage Example (planned):**
  ```rust
  rkit.redo();
  ```
- [ ] `undo_selection_cursor`, `redo_selection_cursor`
  **Usage Example (planned):**
  ```rust
  rkit.undo_selection_cursor();
  rkit.redo_selection_cursor();
  ```



### Search & Replace
- [x] `find_next`
  **Usage Example (planned):**
  ```rust
  if let Some((row, col)) = rkit.find_next("search_term", None) {
      rkit.move_cursor_to(row, col);
  }
  ```
- [x] `replace_next`
  **Usage Example (planned):**
  ```rust
  rkit.replace_next("search_term", "replacement", None);
  ```
- [ ] Search match color setter (setter exists, not used)
  **Usage Example (planned):**
  ```rust
  rkit.set_search_match_color("#ffff99");
  ```



### Clipboard
- [x] `copy_to_clipboard`
  **Usage Example (planned):**
  ```rust
  rkit.copy_to_clipboard();
  ```
- [ ] `paste_from_clipboard` (async, prints to console)
  **Usage Example (planned):**
  ```rust
  rkit.paste_from_clipboard();
  ```
- [ ] `copy` (returns first line if no selection)
  **Usage Example (planned):**
  ```rust
  let text = rkit.copy();
  ```



### Rendering & Redraw
- [x] `request_redraw`
  **Usage Example (planned):**
  ```rust
  rkit.request_redraw();
  ```
- [x] Redraw callback
  **Usage Example (planned):**
  ```rust
  rkit.redraw_callback = Some(Box::new(|| {
      // Custom redraw logic
  }));
  ```

## Rendering Model
- Uses `gtk4::DrawingArea` for full control
- Text layout via `pango::Layout`
- Redraws only dirty regions
- Cursor, selection, and highlight drawn manually


### Margin & Page
- [x] Margin setters (`update_margins`, etc.)
  **Usage Example (planned):**
  ```rust
  rkit.update_margins(EditorMarginUpdate {
      top: 2.0,
      bottom: 2.0,
      left: 2.0,
      right: 2.0,
  });
  ```
- [x] A4 page logic
  **Usage Example (planned):**
  ```rust
  rkit.set_page_type(PageType::A4);
  ```
- [ ] US Letter logic
  **Usage Example (planned):**
  ```rust
  rkit.set_page_type(PageType::USLetter);
  ```

The editor aims to support both A4 and US Letter page formats for margin and layout calculations in native metric and Imperial. You will be able to select the page type (A4 or US Letter) to match your region or printing requirements.

| Property         | A4 Paper                     | US Letter Paper                |
|------------------|------------------------------|--------------------------------|
| Standard         | ISO 216 (International)      | ANSI (North America)           |
| Dimensions (mm)  | 210 × 297 mm                 | 216 × 279 mm                   |
| Dimensions (in)  | 8.27 × 11.69 inches          | 8.5 × 11 inches                |
| Orientation      | Taller and slightly narrower | Shorter and slightly wider     |
| Common Regions   | Europe, Asia, most countries | USA, Canada, Mexico            |


### Theme & Colors
- [ ] Theme switching (`switch_theme`)
- [ ] Color setters (font, fg/bg, gutter, line number, highlight, etc.) (setters exist, not used)
- [ ] Markdown color setters (setters exist, not used)
- [ ] Diagnostics color setters (setters exist, not used)

**Usage Examples:**

#### Text Edit
```rust
// Set font family and size
rkit.set_font("Fira Mono", 14);

// Set line height
rkit.set_line_height(1.5);

// Set line width
rkit.set_line_width(2.0);

// Set foreground and background color
rkit.set_fg_color("#222222");
rkit.set_bg_color("#fafafa");
```

#### Gutter
```rust
// Set gutter background color
rkit.set_gutter_bg_color("#f0f0f0");

// Set gutter border color
rkit.set_gutter_border_color("#cccccc");

// Set gutter font
rkit.set_gutter_font("Fira Mono", 12);

// Set gutter width
rkit.set_gutter_width(40.0);

// Set gutter and line number color
rkit.set_gutter_color("#e0e0e0");
rkit.set_line_number_color("#888888");
```

#### Other
```rust
// Set highlight color
rkit.set_highlight_color("#ffe066");


// Set diagnostics color (for syntax lint or spellcheck)
rkit.set_diagnostics_color("#ff3333");
```


### Syntax Highlighting
- [x] Syntax highlighting toggle


### Whitespace & Line Highlighting
- [ ] Whitespace guide color setter (setter exists, not used)
- [ ] Whitespace guide toggle (toggle exists, not used)
- [ ] Active/inactive line background color setters (setters exist, not used)
- [x] Line highlight toggle


### Diagnostics
- [ ] Diagnostics highlighting toggle (toggle exists, not used)
- [ ] Diagnostics messages (structure exists, not used)


### Stubs / Planned Features
- [ ] Print/export (`print_export`)
- [ ] Status bar update (`update_status_bar`)
- [ ] IME support (`ime_support`)
- [ ] Accessibility support (`accessibility_support`)
- [ ] Performance optimization (`optimize_performance`)
- [ ] Custom caret animations or smooth scroll
- [ ] GPU rendering for ultra-fast updates
- [ ] Plugin system (Rust-based extensions)
- [ ] Configurable grammar/themes via RON or TOML

## Screenshots / Demo
Coming soon...