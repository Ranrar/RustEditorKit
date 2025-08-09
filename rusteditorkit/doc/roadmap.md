# RustEditorKit Roadmap

## Core Editing Functionality
 - [x] Unified sizing API for GTK widgets (trait-based, flexible modes)
- [x] Multi-cursor editing (local only)
- [x] Undo/redo stack  
- [x] Selection logic (select all, multi-selection, start/end)  
- [x] Cursor navigation (move left/right/up/down, page up/down, home/end)  
- [x] Line highlight  
- [x] Simpel configurations file when using the `editorwidget` in GTK or via `API`
- [x] Clipboard integration (copy to clipboard)  
- [x] Undo/redo
- [ ] Whitespace guide color setter/toggle
- [ ] Search and replace  
- [ ] IME support
- [ ] API to use in non GTK envierments
- [ ] Status bar update API

## Page Layout & Document Format
- [ ] custom Margin
- [ ] A4 page logic  
- [ ] US Letter page logic 

## Appearance & Theming
- [ ] auto switch light/dark-mode
- [ ] GPU rendering for ultra-fast updates    

## Plugin System
- [ ] Plugin system   
- [ ] Configurable grammar via RON/TOML   
- [ ] Auto-indent   
- [ ] Comment toggle   
- [ ] Syntax highlighting
- [ ] AI assistant    
- [ ] Annotation mode (locked/static text interleaved with editable regions)

## Performance & Optimization
- [ ] Performance optimization (large file support < 100 mb)