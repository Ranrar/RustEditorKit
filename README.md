[![Contributions Welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](https://github.com/Ranrar/RustEditorKit/issues)

# RustEditorKit

A modern, customizable text editor engine for Rust applications using GTK4. Built to be a powerful alternative to GtkSourceView with full Rust integration.

## Features

- **Search & replace** with pattern matching
- **Undo/redo** with complete state management
- **Internal scrolling system** with configurable policies and smooth scrolling
- **Line numbers, gutters** and visual aids
- **Cross-platform** (Linux, Windows, macOS)
- **Modular design** - use only what you need

## Why RustEditorKit?

Perfect for:
- Building Rust desktop applications with GTK4
- Replacing GtkSourceView in existing projects
- Educational projects learning text editor architecture

## Quick Start

```bash
git clone https://github.com/Ranrar/RustEditorKit.git
cd RustEditorKit
cargo run --bin demo
```

Basic usage:
```rust
use rusteditorkit::editorwidget::editor::EditorWidget;

let editor = EditorWidget::new();
editor.connect_signals();
window.set_child(Some(editor.widget()));
```

## Documentation

- **[API Reference](rusteditorkit/doc/API.md)** - Complete function reference and usage patterns
- **[Widget Integration](rusteditorkit/doc/EditorWidget.md)** - GTK4 integration guide and configuration
- **[Scroll System](rusteditorkit/doc/ScrollSystem.md)** - Internal scrolling implementation and configuration
- **[Roadmap](rusteditorkit/doc/roadmap.md)** - Planned features and milestones

## Contributing

**We need your help!** This project is in active development and welcomes contributors of all skill levels.

**What we're looking for:**
- Bug reports and feature requests
- Code contributions (Rust experience helpful but not required)
- Documentation improvements
- Testing on different platforms
- UI/UX feedback and suggestions

**Easy ways to contribute:**
- Try the demo and report issues
- Improve documentation or examples
- Add tests for existing features
- Help with cross-platform compatibility

**Current priority areas:**
- Performance optimization
- Plugin architecture design and API

**Project Status:** Alpha - Most core features working, API may change

**Known Issues in Current Build**
- Mouse selection are not visually working as expected.
- Arrow key navigation has issues with multi-byte UTF-8 sequences.
- Scrollbar is not working (it hasn’t been implemented yet).