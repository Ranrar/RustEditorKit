# RustEditorKit Roadmap

## Core Editing Functionality
- [x] Multi-cursor editing (local only)
- [x] Undo/redo stack  
- [x] Selection logic (select all, multi-selection, start/end)  
- [x] Cursor navigation (move left/right/up/down, page up/down, home/end)  
- [x] Auto-indent and comment toggle  
- [x] Syntax highlighting toggle  
- [x] Line highlight toggle  
- [x] Search and replace  
- [x] Clipboard integration (copy to clipboard)  
- [x] Undo/redo selection cursor
- [x] Whitespace guide color setter/toggle
- [x] Simpel configurations file when using the `editorwidget` in GTK or via `API`
- [x] API to use in non GTK envierments

## 🤖 AI-assisted Tools
- [ ] AI assistant (manual on/off) for writing and editing suggestions

## Collaborative Editing via Yjs CRDT
- [ ] Shared document model  
- [ ] Multi-cursor editing (over network API)  
- [ ] Conflict resolution / OT or CRDT engine  
- [ ] Annotation mode support in collaborative context   
- [ ] Session host/join logic (start collaboration session)
- [ ] Real-time sync engine (low-latency updates)
- [ ] User presence (remote user cursors, names, colors)

## Page Layout & Document Format
- [x] Margin and A4 page support  
- [x] A4 page logic  
- [ ] US Letter page logic 
- [ ] Print/export (PDF/HTML/Markdown)
- [ ] HTML background template (A4/US Letter, theme-aware)

## Appearance & Theming
- [-] Customizable themes and colors
- [ ] Configurable grammar/themes via RON/TOML  
- [ ] Custom caret animations/smooth scroll  
- [ ] GPU rendering for ultra-fast updates    

## Extensions & Plugin System
- [x] Extensible modules for new features  
- [ ] Plugin system (Rust-based extensions) 

## Structured & Semantic Editing
- [ ] Annotation mode (locked/static text interleaved with editable regions)

## ✒️ Input UX & Typography
- [ ] Floating writing line (free movement, fixed line height & spacing)

## System & Input Integration
- [x] File I/O (basic load/save)  
- [x] Crossplatform font, file location
- [ ] IME support
- [ ] Accessibility support  
- [ ] Keybindings customization

## UI Components & Diagnostics
- [ ] Status bar update
- [ ] Diagnostics highlighting toggle/messages
- [ ] Completion and diagnostics modules (structure exists, not used)  

## Performance & Optimization
- [ ] Performance optimization (large file support)  

## Testing & Code Quality
- [x] Unit tests for buffer operations
- [ ] Test for each function