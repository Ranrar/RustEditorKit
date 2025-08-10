# RustEditorKit - AI Coding Agent Instructions

RustEditorKit is a modular, cross-platform text editor engine built with GTK4 and Rust. It's designed as a library for embedding rich text editing capabilities into Rust applications.

## Architecture Overview

### Core Module Structure
- **`rusteditorkit/src/corelogic/`** - Pure business logic (buffer, cursor, editing, undo, etc.)
- **`rusteditorkit/src/render/`** - Layered rendering system using Cairo/Pango
- **`rusteditorkit/src/widget/`** - GTK4 integration and user input handling  
- **`rusteditorkit/src/config/`** - RON-based configuration with platform fallbacks
- **`rusteditorkit/src/keybinds/`** - Platform-specific key mappings (Linux/macOS/Windows)

### Central Types
```rust
// Core data structure - all editor state lives here
EditorBuffer: Rc<RefCell<EditorBuffer>>

// Main widget - connects GTK4 to core logic
EditorWidget::new() -> connects signals, renders via drawing_area

// Configuration - loaded from .ron files
EditorConfig: font, cursor, gutter, scroll, selection settings
```

## Key Patterns & Conventions

### 1. Layered Rendering Architecture
Rendering uses a **layered approach** in `render/mod.rs`:
```rust
// Order matters - each layer builds on previous
background::render_background_layer()
text::render_text_layer()        // Must be first - calculates metrics
gutter::render_gutter_layer()
selection::render_selection_layer()
cursor::render_cursor_layer()
```

### 2. State Management Pattern
- **Core logic**: Pure functions in `corelogic/` modules
- **State storage**: Centralized in `EditorBuffer` 
- **UI updates**: Buffer calls `redraw_callback` to trigger GTK redraws
- **Configuration**: Runtime config changes update buffer state immediately

### 3. Cross-Platform Keybindings
```rust
// Platform detection at compile time
#[cfg(target_os = "linux")]
let keymap = crate::keybinds::linux::linux_keymap();

// Unified action system
EditorAction enum -> KeyCombo mapping -> buffer methods
```

## Development Workflows

### Build & Test Commands
```bash
# Run demo application
cargo run --bin demo

# Run all tests (includes keybind validation)
cargo test --workspace

# Run specific test module
cargo test --bin unit_tests

# Fix failing doctests first:
cargo test --doc --package rusteditorkit

# Cross-platform compatibility check (requires setup)
# Note: Cross-compilation requires mingw-w64 for Windows targets
cargo check  # Native platform only
```

### Configuration Testing
```bash
# Test with custom config
cp rusteditorkit/src/config/config.ron demo/src/config.ron
cargo run --bin demo
```

## Critical Integration Points

### Widget Configuration System
Uses trait-based sizing API instead of direct GTK calls:
```rust
use rusteditorkit::corelogic::{ConfigurableSize, SizeMode};

drawing_area.configure_size(SizeMode::Minimum(400, 300));
drawing_area.switch_mode(SizeMode::AspectRatio(16.0/9.0));
```

### Signal Connection Pattern
Widget setup follows this order:
```rust
let editor = EditorWidget::new();
editor.connect_signals();          // Input handling
editor.update_cursor_config();     // Cursor blink setup
window.set_child(Some(editor.widget()));
```

### Configuration Loading
RON files control all behavior - see `rusteditorkit/src/config/config.ron`:
```ron
// Example: cursor configuration
cursor: (
    cursor_blink: true,
    cursor_blink_rate: 1000,
    cursor_color: "#000000",
    cursor_type: "line",  // "line" | "block" | "underline"
)
```

## Project-Specific Conventions

### Error Handling
- Use `Result<T, CommandError>` for operations that can fail
- Avoid panics in production code - graceful degradation preferred
- `CommandDispatcher` provides centralized error handling for editor actions

### Memory Management  
- `Rc<RefCell<>>` only for shared mutable state (EditorBuffer)
- Prefer immutable data structures where possible
- Clone configs liberally - they're small and cacheable

### Testing Strategy
- **Unit tests**: Logic-only tests in `corelogic/` modules (no GTK required)
- **Integration tests**: Full widget tests in `unit_tests/` workspace
- **Platform tests**: Keybind validation across Linux/macOS/Windows

### Documentation Standards
- Public APIs require `///` doc comments with examples
- Internal functions use `//` comments explaining "why" not "what"
- See `rusteditorkit/doc/API.md` for complete API reference

## Common Gotchas

1. **Rendering order matters** - text layer must render first to calculate line metrics
2. **Config changes** require calling `update_cursor_config()` to restart blink timers
3. **Platform keybinds** use different modifier keys (Cmd vs Ctrl) - test on target platform
4. **RON syntax** is strict about trailing commas and parentheses in config files
5. **Buffer mutations** should trigger `request_redraw()` to update UI
6. **GTK4 thread safety** - All UI operations must happen on the main thread
7. **Doctests with GTK** require `no_run` annotation to avoid initialization issues

## Current State & Known Issues

**Alpha Status** - Core functionality working, API may change
- ✅ Text rendering, cursor movement, basic editing
- ✅ Undo/redo, search, clipboard integration  
- ✅ RON configuration system with live reloading
- ⚠️ Mouse selection visually not working as expected
- ⚠️ Arrow key navigation issues with multi-byte UTF-8
- ❌ Scrollbar not implemented yet
- ❌ Some config options marked "not implemented" in config.ron

## Quick Reference Files
- **API Examples**: `demo/src/main.rs` - Complete widget integration
- **Config Schema**: `rusteditorkit/src/config/configuration.rs` 
- **Render Pipeline**: `rusteditorkit/src/render/mod.rs`
- **Action Dispatch**: `rusteditorkit/src/corelogic/dispatcher.rs`
