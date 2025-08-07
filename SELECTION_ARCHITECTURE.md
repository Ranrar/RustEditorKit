# RustEditorKit Selection Architecture Documentation

## Overview

This document provides a comprehensive analysis of the text selection implementation in RustEditorKit, a cross-platform, GTK4-based Rust editor engine. The selection system supports robust multi-line text selection with proper Unicode handling, keyboard and mouse interaction, and configurable rendering.

## Table of Contents

1. [Core Data Structures](#core-data-structures)
2. [Selection Model](#selection-model)
3. [Selection Operations](#selection-operations)
4. [Mouse Interaction](#mouse-interaction)
5. [Keyboard Interaction](#keyboard-interaction)
6. [Rendering System](#rendering-system)
7. [Configuration](#configuration)
8. [Multi-Selection Support](#multi-selection-support)
9. [Unicode and Character Safety](#unicode-and-character-safety)
10. [Integration Flow](#integration-flow)
11. [Testing Patterns](#testing-patterns)
12. [Best Practices](#best-practices)

---

## Core Data Structures

### Selection Struct

Located in `src/corelogic/selection.rs`, the `Selection` struct is the foundation of the selection system:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
}
```

**Key Properties:**
- **Position-based**: Uses (row, col) coordinates rather than absolute byte/character indices
- **Direction-aware**: Start and end positions can be in any order (forward or backward selection)
- **Bounds-safe**: Includes clamping methods to ensure positions remain valid

### EditorBuffer Integration

The main `EditorBuffer` struct integrates selection state:

```rust
pub struct EditorBuffer {
    // Core text data
    pub lines: Vec<String>,
    pub cursor: EditorCursor,
    
    // Selection state
    pub selection: Option<Selection>,
    pub multi_selections: Vec<(Option<(usize, usize)>, Option<(usize, usize)>)>,
    pub multi_cursors: Vec<(usize, usize)>,
    
    // Mouse interaction state
    pub mouse_state: MouseState,
    
    // ... other fields
}
```

### Mouse State Management

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseState {
    Idle,
    Selecting { start_row: usize, start_col: usize },
    ExtendingSelection,
}
```

---

## Selection Model

### Selection Lifecycle

1. **Creation**: Selection starts when user begins drag operation or uses Shift+arrow keys
2. **Extension**: Selection grows/shrinks as user continues mouse drag or keyboard selection
3. **Normalization**: Selection coordinates are normalized for operations (start ≤ end)
4. **Validation**: Selection bounds are clamped to valid buffer positions
5. **Termination**: Selection ends when user clicks, presses Escape, or performs editing operation

### Core Methods

#### `Selection::new(row: usize, col: usize)`
Creates a new selection starting and ending at the same position (zero-width selection).

#### `Selection::set(start_row, start_col, end_row, end_col)`
Sets selection coordinates explicitly.

#### `Selection::normalized() -> ((usize, usize), (usize, usize))`
Returns selection coordinates in guaranteed start ≤ end order:

```rust
pub fn normalized(&self) -> ((usize, usize), (usize, usize)) {
    if (self.start_row, self.start_col) <= (self.end_row, self.end_col) {
        ((self.start_row, self.start_col), (self.end_row, self.end_col))
    } else {
        ((self.end_row, self.end_col), (self.start_row, self.start_col))
    }
}
```

#### `Selection::clamp_to_buffer(lines: &Vec<String>)`
Ensures selection coordinates remain within valid buffer bounds:

```rust
pub fn clamp_to_buffer(&mut self, lines: &Vec<String>) {
    let last_row = lines.len().saturating_sub(1);
    self.start_row = self.start_row.min(last_row);
    self.end_row = self.end_row.min(last_row);
    self.start_col = self.start_col.min(lines.get(self.start_row).map(|l| l.len()).unwrap_or(0));
    self.end_col = self.end_col.min(lines.get(self.end_row).map(|l| l.len()).unwrap_or(0));
    
    // Handle empty buffer
    if lines.is_empty() {
        self.start_row = 0;
        self.start_col = 0;
        self.end_row = 0;
        self.end_col = 0;
    }
}
```

#### `Selection::is_active() -> bool`
Determines if selection covers any text:

```rust
pub fn is_active(&self) -> bool {
    self.start_row != self.end_row || self.start_col != self.end_col
}
```

---

## Selection Operations

### Text Retrieval

The system provides multiple ways to access selected text:

#### `EditorBuffer::get_selected_text() -> Option<String>`
Returns the currently selected text with proper Unicode handling:

```rust
pub fn get_selected_text(&self) -> Option<String> {
    if let Some(sel) = &self.selection {
        let ((start_row, start_col), (end_row, end_col)) = sel.normalized();
        
        if start_row == end_row {
            // Single line selection - use character-based slicing
            let line = &self.lines[start_row];
            let chars: Vec<char> = line.chars().collect();
            let selected_chars = chars.get(start_col..end_col).unwrap_or(&[]);
            Some(selected_chars.iter().collect::<String>())
        } else {
            // Multi-line selection
            let mut result = String::new();
            
            // First line
            let first_line = &self.lines[start_row];
            let first_chars: Vec<char> = first_line.chars().collect();
            let first_selected = first_chars.get(start_col..).unwrap_or(&[]);
            result.push_str(&first_selected.iter().collect::<String>());
            result.push('\n');
            
            // Intermediate lines
            for row in start_row + 1..end_row {
                result.push_str(&self.lines[row]);
                result.push('\n');
            }
            
            // Last line
            let last_line = &self.lines[end_row];
            let last_chars: Vec<char> = last_line.chars().collect();
            let last_selected = last_chars.get(..end_col).unwrap_or(&last_chars);
            result.push_str(&last_selected.iter().collect::<String>());
            
            Some(result)
        }
    } else {
        None
    }
}
```

#### Copy Operation

The `copy()` method handles clipboard operations:

```rust
pub fn copy(&self) -> String {
    if let Some(sel) = &self.selection {
        let ((start_row, start_col), (end_row, end_col)) = sel.normalized();
        
        if start_row == end_row && start_row < self.lines.len() && end_col > start_col {
            // Single line selection - use character-based slicing
            let line = &self.lines[start_row];
            let chars: Vec<char> = line.chars().collect();
            let selected_chars = chars.get(start_col..end_col).unwrap_or(&[]);
            return selected_chars.iter().collect::<String>();
        } else if start_row < self.lines.len() && end_row < self.lines.len() {
            // Multi-line selection logic...
        }
    }
    
    // No selection - return current line
    self.lines.get(self.cursor.row).cloned().unwrap_or_default()
}
```

### Text Deletion

#### `EditorBuffer::delete_selection() -> bool`
Removes selected text and returns whether any text was deleted:

```rust
pub fn delete_selection(&mut self) -> bool {
    if let Some(sel) = self.selection.clone() {
        self.push_undo(); // Save state for undo
        
        let ((start_row, start_col), (end_row, end_col)) = sel.normalized();
        
        if start_row == end_row {
            // Single line deletion - use character-based operations
            let line = &mut self.lines[start_row];
            let chars: Vec<char> = line.chars().collect();
            
            let before: String = chars.get(..start_col).unwrap_or(&[]).iter().collect();
            let after: String = chars.get(end_col..).unwrap_or(&[]).iter().collect();
            *line = format!("{}{}", before, after);
            
            self.cursor.row = start_row;
            self.cursor.col = start_col;
        } else {
            // Multi-line deletion - join first and last lines
            let end_line = &self.lines[end_row];
            let end_chars: Vec<char> = end_line.chars().collect();
            let end_part: String = end_chars.get(end_col..).unwrap_or(&[]).iter().collect();
            
            let start_line = &mut self.lines[start_row];
            let start_chars: Vec<char> = start_line.chars().collect();
            let before_part: String = start_chars.get(..start_col).unwrap_or(&[]).iter().collect();
            
            *start_line = format!("{}{}", before_part, end_part);
            
            // Remove intermediate lines
            for _ in start_row + 1..=end_row {
                self.lines.remove(start_row + 1);
            }
            
            self.cursor.row = start_row;
            self.cursor.col = start_col;
        }
        
        self.selection = None;
        true
    } else {
        false
    }
}
```

---

## Mouse Interaction

### Click Handling

#### Single Click (`handle_mouse_click`)
```rust
pub fn handle_mouse_click(&mut self, x: f64, y: f64, shift_held: bool, 
                         line_height: f64, left_margin: f64, top_margin: f64, 
                         pango_ctx: &gtk4::pango::Context, 
                         font_desc: &gtk4::pango::FontDescription) {
    let (row, col) = self.screen_to_buffer_position(x, y, line_height, left_margin, 
                                                   top_margin, pango_ctx, font_desc);
    
    if shift_held && self.selection.is_some() {
        // Extend existing selection
        if let Some(sel) = &mut self.selection {
            sel.end_row = row;
            sel.end_col = col;
            sel.clamp_to_buffer(&self.lines);
        }
    } else {
        // Clear selection and set cursor position
        self.selection = None;
        self.cursor.row = row;
        self.cursor.col = col;
    }
    
    // Update mouse state
    self.mouse_state = if shift_held {
        MouseState::ExtendingSelection
    } else {
        MouseState::Selecting { start_row: row, start_col: col }
    };
}
```

#### Double Click (`handle_double_click`)
Selects the word at the click position:

```rust
pub fn handle_double_click(&mut self, x: f64, y: f64, ...) {
    let (row, col) = self.screen_to_buffer_position(x, y, ...);
    
    if row < self.lines.len() {
        let line = &self.lines[row];
        let chars: Vec<char> = line.chars().collect();
        
        if col < chars.len() {
            let mut start_col = col;
            let mut end_col = col;
            
            // Move start back to beginning of word
            while start_col > 0 && (chars[start_col - 1].is_alphanumeric() || chars[start_col - 1] == '_') {
                start_col -= 1;
            }
            
            // Move end forward to end of word
            while end_col < chars.len() && (chars[end_col].is_alphanumeric() || chars[end_col] == '_') {
                end_col += 1;
            }
            
            // Create selection for the word
            if start_col < end_col {
                let mut sel = Selection::new(row, start_col);
                sel.end_row = row;
                sel.end_col = end_col;
                self.selection = Some(sel);
                
                self.cursor.row = row;
                self.cursor.col = end_col;
            }
        }
    }
}
```

#### Triple Click (`handle_triple_click`)
Selects the entire line:

```rust
pub fn handle_triple_click(&mut self, x: f64, y: f64, ...) {
    let (row, _) = self.screen_to_buffer_position(x, y, ...);
    
    if row < self.lines.len() {
        let mut sel = Selection::new(row, 0);
        sel.end_row = row;
        sel.end_col = self.lines[row].chars().count();
        self.selection = Some(sel);
        
        self.cursor.row = row;
        self.cursor.col = self.lines[row].chars().count();
    }
}
```

### Drag Handling

#### `handle_mouse_drag`
Continuously updates selection during mouse drag:

```rust
pub fn handle_mouse_drag(&mut self, x: f64, y: f64, ...) {
    // Snap y coordinate to nearest line
    let snapped_y = (y / line_height).round() * line_height;
    let (row, col) = self.screen_to_buffer_position(x, snapped_y, ...);
    
    match self.mouse_state {
        MouseState::Selecting { start_row, start_col } => {
            // Create new selection from start to current position
            let mut sel = Selection::new(start_row, start_col);
            sel.end_row = row;
            sel.end_col = col;
            sel.clamp_to_buffer(&self.lines);
            
            // Only set selection if there's an actual area selected
            if sel.is_active() {
                self.selection = Some(sel);
            } else {
                self.selection = None;
            }
            
            // Update cursor to current position
            self.cursor.row = row;
            self.cursor.col = col;
        },
        MouseState::ExtendingSelection => {
            // Extend existing selection
            if let Some(sel) = &mut self.selection {
                sel.end_row = row;
                sel.end_col = col;
                sel.clamp_to_buffer(&self.lines);
            }
            self.cursor.row = row;
            self.cursor.col = col;
        },
        MouseState::Idle => {
            // Start new selection
            self.mouse_state = MouseState::Selecting { start_row: row, start_col: col };
        }
    }
}
```

---

## Keyboard Interaction

### Key Mapping

The system uses platform-specific key mappings defined in `src/keybinds/`:

**Linux Keybindings** (`src/keybinds/linux.rs`):
```rust
// Selection with Shift + arrow keys
map.insert(SelectLeft, KeyCombo::new("Left", false, true, false));
map.insert(SelectRight, KeyCombo::new("Right", false, true, false));
map.insert(SelectUp, KeyCombo::new("Up", false, true, false));
map.insert(SelectDown, KeyCombo::new("Down", false, true, false));

// Select all with Ctrl+A
map.insert(SelectAll, KeyCombo::new("a", true, false, false));

// Copy/Cut/Paste
map.insert(CopySelection, KeyCombo::new("c", true, false, false));
map.insert(CutSelection, KeyCombo::new("x", true, false, false));
map.insert(PasteClipboard, KeyCombo::new("v", true, false, false));

// Clear selection with Escape
map.insert(ClearSelection, KeyCombo::new("Escape", false, false, false));
```

### Directional Selection

#### `select_left()`, `select_right()`, `select_up()`, `select_down()`
These methods extend or create selection in the specified direction:

```rust
pub fn select_left(&mut self) {
    let prev_cursor = self.cursor;
    self.move_left_internal(); // Move cursor without clearing selection
    let new_cursor = self.cursor;
    
    if prev_cursor != new_cursor {
        match &mut self.selection {
            Some(sel) => {
                // Extend existing selection
                sel.end_row = new_cursor.row;
                sel.end_col = new_cursor.col;
                sel.clamp_to_buffer(&self.lines);
                
                // Clear selection if start and end are the same
                if sel.start_row == sel.end_row && sel.start_col == sel.end_col {
                    self.selection = None;
                }
            }
            None => {
                // Create new selection
                let mut sel = Selection::new(prev_cursor.row, prev_cursor.col);
                sel.set(prev_cursor.row, prev_cursor.col, new_cursor.row, new_cursor.col);
                self.selection = Some(sel);
            }
        }
    }
}
```

### Select All

#### `select_all()`
Selects the entire buffer content:

```rust
pub fn select_all(&mut self) {
    if !self.lines.is_empty() {
        let mut sel = Selection::new(0, 0);
        let end_row = self.lines.len() - 1;
        let end_col = self.lines[end_row].len();
        sel.set(0, 0, end_row, end_col);
        self.selection = Some(sel);
    }
}
```

### Clear Selection

#### `clear_selection()`
Removes the current selection:

```rust
pub fn clear_selection(&mut self) {
    self.selection = None;
}
```

---

## Rendering System

### Selection Layer Rendering

Located in `src/render/selection.rs`, this module handles visual representation:

#### `render_selection_layer()`
Main entry point for selection rendering:

```rust
pub fn render_selection_layer(buf: &EditorBuffer, ctx: &Context, 
                             layout: &LayoutMetrics, width: i32) {
    // Early return if no selection exists
    let selection = match &buf.selection {
        Some(sel) => sel,
        None => return,
    };

    if !selection.is_active() {
        return;
    }

    let selection_config = buf.config.selection();

    // Parse selection background color from config
    let bg_color = &selection_config.selection_bg_color;
    let (r, g, b, _) = parse_color(bg_color);
    let opacity = selection_config.selection_opacity;
    ctx.set_source_rgba(r, g, b, opacity);

    // Get normalized selection coordinates
    let ((start_row, start_col), (end_row, end_col)) = selection.normalized();

    // Handle single-line vs multi-line selections
    if start_row == end_row {
        render_single_line_selection_coords(ctx, start_row, start_col, end_col, layout, buf);
    } else {
        render_multi_line_selection_coords(ctx, start_row, start_col, end_row, end_col, 
                                         layout, buf, width);
    }
}
```

#### Single Line Selection Rendering

```rust
fn render_single_line_selection_coords(ctx: &Context, row: usize, start_col: usize, 
                                      end_col: usize, layout: &LayoutMetrics, 
                                      buf: &EditorBuffer) {
    if row >= buf.lines.len() {
        return;
    }

    let line = &buf.lines[row];
    let start_col = start_col.min(line.chars().count());
    let end_col = end_col.min(line.chars().count());

    // Calculate pixel positions using Pango for accurate character positioning
    let font_desc = FontDescription::from_string(&format!("{} {}", 
        buf.config.font.font_name(), buf.config.font.font_size()));
    let start_x = calculate_column_x_position(line, start_col, layout, ctx, &font_desc);
    let end_x = calculate_column_x_position(line, end_col, layout, ctx, &font_desc);

    let y_offsets = buf.line_y_offsets(layout.line_height, 
        buf.config.font.font_paragraph_spacing(), layout.top_offset);
    let y_line = y_offsets.get(row).copied().unwrap_or(layout.top_offset);
    
    let selection_width = end_x - start_x;
    if selection_width > 0.0 {
        ctx.rectangle(start_x, y_line, selection_width, layout.line_height);
        ctx.fill().unwrap();
    }
}
```

#### Multi-Line Selection Rendering

```rust
fn render_multi_line_selection_coords(ctx: &Context, start_row: usize, start_col: usize,
                                     end_row: usize, end_col: usize, layout: &LayoutMetrics,
                                     buf: &EditorBuffer, width: i32) {
    let text_left_offset = layout.text_left_offset;
    let right_edge = width as f64;
    
    for row in start_row..=end_row {
        if row >= buf.lines.len() {
            break;
        }
        
        let line = &buf.lines[row];
        let y_offsets = buf.line_y_offsets(layout.line_height, 
            buf.config.font.font_paragraph_spacing(), layout.top_offset);
        let y_line = y_offsets.get(row).copied().unwrap_or(layout.top_offset);
        
        if row == start_row {
            // First line: from start_col to end of line
            let start_col = start_col.min(line.chars().count());
            let font_desc = FontDescription::from_string(&format!("{} {}", 
                buf.config.font.font_name(), buf.config.font.font_size()));
            let start_x = calculate_column_x_position(line, start_col, layout, ctx, &font_desc);
            let width = right_edge - start_x;
            if width > 0.0 {
                ctx.rectangle(start_x, y_line, width, layout.line_height);
                ctx.fill().unwrap();
            }
        } else if row == end_row {
            // Last line: from start of line to end_col
            let end_col = end_col.min(line.chars().count());
            let font_desc = FontDescription::from_string(&format!("{} {}", 
                buf.config.font.font_name(), buf.config.font.font_size()));
            let end_x = calculate_column_x_position(line, end_col, layout, ctx, &font_desc);
            let width = end_x - text_left_offset;
            if width > 0.0 {
                ctx.rectangle(text_left_offset, y_line, width, layout.line_height);
                ctx.fill().unwrap();
            }
        } else {
            // Middle lines: select entire line
            let line_width = right_edge - text_left_offset;
            ctx.rectangle(text_left_offset, y_line, line_width, layout.line_height);
            ctx.fill().unwrap();
        }
    }
}
```

### Character Position Calculation

The system uses Pango for accurate character positioning:

```rust
fn calculate_column_x_position(line: &str, col: usize, layout: &LayoutMetrics, 
                              ctx: &Context, font_desc: &FontDescription) -> f64 {
    const PANGO_SCALE: i32 = 1024;

    if col == 0 {
        return layout.text_left_offset;
    }

    // Create a Pango layout for the line
    let pango_layout = create_layout(ctx);
    pango_layout.set_text(line);
    pango_layout.set_font_description(Some(font_desc));

    // Convert column (char index) to byte index
    let byte_index = line.char_indices().nth(col).map(|(i, _)| i).unwrap_or(line.len());
    let pos = pango_layout.index_to_pos(byte_index as i32);
    
    // pos.x() is in Pango units, convert to pixels
    layout.text_left_offset + (pos.x() as f64 / PANGO_SCALE as f64)
}
```

---

## Configuration

### Selection Configuration

Located in `src/config/configuration.rs`:

```rust
#[derive(Debug, Deserialize, Clone)]
pub struct SelectionConfig {
    pub selection_toggle: bool,           // Enable/disable selection
    pub selection_bg_color: String,       // Background color (hex)
    pub selection_opacity: f64,           // Transparency (0.0-1.0)
    pub selection_text_color: String,     // Text color (hex)
}

impl Default for SelectionConfig {
    fn default() -> Self {
        Self {
            selection_toggle: true,
            selection_bg_color: "#0050aa".to_string(),  // Blue
            selection_opacity: 0.3,                      // 30% opacity
            selection_text_color: "#ffffff".to_string(), // White
        }
    }
}
```

### Configuration Access

The `EditorConfig` provides methods to access selection configuration:

```rust
impl EditorConfig {
    pub fn selection(&self) -> &SelectionConfig { &self.selection }
    pub fn set_selection_bg_color(&mut self, color: &str) { /* ... */ }
    pub fn selection_bg_color(&self) -> &str { &self.selection.selection_bg_color }
    pub fn set_selection_opacity(&mut self, v: f64) { /* ... */ }
    pub fn selection_opacity(&self) -> f64 { self.selection.selection_opacity }
    // ... other selection configuration methods
}
```

---

## Multi-Selection Support

### Multi-Cursor System

The system supports multiple cursors and selections:

```rust
impl EditorBuffer {
    /// Add a new cursor at (row, col)
    pub fn add_cursor(&mut self, row: usize, col: usize) {
        self.multi_cursors.push((row, col));
    }

    /// Add a new selection (start, end)
    pub fn add_selection(&mut self, start: Option<(usize, usize)>, end: Option<(usize, usize)>) {
        self.multi_selections.push((start, end));
    }

    /// Clear all additional cursors and selections
    pub fn clear_cursors(&mut self) {
        self.multi_cursors.clear();
    }

    pub fn clear_selections(&mut self) {
        self.multi_selections.clear();
    }
}
```

---

## Unicode and Character Safety

### Character-Based Operations

The selection system prioritizes Unicode safety by operating on character boundaries rather than byte boundaries:

#### Character Indexing
```rust
// Convert column (char index) to byte index safely
let byte_index = line.char_indices().nth(col).map(|(i, _)| i).unwrap_or(line.len());
```

#### Character-Based Text Extraction
```rust
// Single line selection - use character-based slicing
let line = &self.lines[start_row];
let chars: Vec<char> = line.chars().collect();
let selected_chars = chars.get(start_col..end_col).unwrap_or(&[]);
return selected_chars.iter().collect::<String>();
```

#### Character-Based Text Deletion
```rust
// Character-safe deletion
let line = &mut self.lines[start_row];
let chars: Vec<char> = line.chars().collect();

let before: String = chars.get(..start_col).unwrap_or(&[]).iter().collect();
let after: String = chars.get(end_col..).unwrap_or(&[]).iter().collect();
*line = format!("{}{}", before, after);
```

### Word Boundary Detection

The system uses Unicode-aware word boundary detection:

```rust
// Word boundary detection for double-click selection
while start_col > 0 && (chars[start_col - 1].is_alphanumeric() || chars[start_col - 1] == '_') {
    start_col -= 1;
}

while end_col < chars.len() && (chars[end_col].is_alphanumeric() || chars[end_col] == '_') {
    end_col += 1;
}
```

---

## Integration Flow

### High-Level Flow Diagram

```
User Input (Mouse/Keyboard)
           ↓
     Input Handlers
  (mouse_click, key_press)
           ↓
    Selection Operations
   (create, extend, modify)
           ↓
     Buffer Updates
  (cursor, selection state)
           ↓
    Rendering Pipeline
  (selection background)
           ↓
      Display Update
```

### Event Processing Flow

1. **Input Detection**: GTK4 events are captured by the widget layer
2. **Coordinate Transformation**: Screen coordinates are converted to buffer positions
3. **Selection Logic**: Selection state is updated based on input type and modifiers
4. **Bounds Validation**: Selection coordinates are clamped to valid buffer bounds
5. **State Synchronization**: Cursor position is updated to match selection end
6. **Rendering Request**: UI redraw is triggered to reflect changes
7. **Visual Update**: Selection background is rendered using Cairo

### Buffer Position Calculation

```rust
pub fn screen_to_buffer_position(&self, x: f64, y: f64, line_height: f64, 
                                left_margin: f64, top_margin: f64,
                                pango_ctx: &gtk4::pango::Context, 
                                font_desc: &gtk4::pango::FontDescription) -> (usize, usize) {
    // Calculate row from y coordinate
    let row = ((y - top_margin) / line_height).floor().max(0.0) as usize;
    let row = row.min(self.lines.len().saturating_sub(1));
    
    if row >= self.lines.len() {
        return (0, 0);
    }
    
    // Calculate column using Pango for accurate text measurement
    let line = &self.lines[row];
    let relative_x = x - left_margin;
    
    // Use Pango to find character position at x coordinate
    let pango_layout = pangocairo::functions::create_layout(pango_ctx);
    pango_layout.set_text(line);
    pango_layout.set_font_description(Some(font_desc));
    
    let (_, col) = pango_layout.xy_to_index(
        (relative_x * pango::SCALE as f64) as i32, 
        0
    );
    
    // Convert byte index back to character index
    let col = line.char_indices().position(|(i, _)| i >= col as usize)
        .unwrap_or(line.chars().count());
    
    (row, col)
}
```

---

## Testing Patterns

### Unit Testing

While specific tests aren't present in the current codebase, the selection system should include:

#### Selection Model Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_normalization() {
        let sel = Selection {
            start_row: 2, start_col: 5,
            end_row: 1, end_col: 3,
        };
        let ((start_row, start_col), (end_row, end_col)) = sel.normalized();
        assert_eq!((start_row, start_col), (1, 3));
        assert_eq!((end_row, end_col), (2, 5));
    }

    #[test]
    fn test_selection_clamping() {
        let lines = vec![
            "short".to_string(),
            "much longer line".to_string(),
        ];
        let mut sel = Selection {
            start_row: 0, start_col: 10,  // Beyond line length
            end_row: 5, end_col: 20,      // Beyond buffer length
        };
        sel.clamp_to_buffer(&lines);
        assert_eq!(sel.start_row, 0);
        assert_eq!(sel.start_col, 5);  // Clamped to line length
        assert_eq!(sel.end_row, 1);    // Clamped to buffer length
        assert_eq!(sel.end_col, 16);   // Clamped to line length
    }

    #[test]
    fn test_unicode_selection() {
        let mut buffer = EditorBuffer::new();
        buffer.lines = vec!["Hello 🌍 World".to_string()];
        
        // Select the emoji
        let mut sel = Selection::new(0, 6);
        sel.end_row = 0;
        sel.end_col = 7;
        buffer.selection = Some(sel);
        
        let selected = buffer.get_selected_text().unwrap();
        assert_eq!(selected, "🌍");
    }
}
```

#### Integration Tests
```rust
#[test]
fn test_selection_deletion() {
    let mut buffer = EditorBuffer::new();
    buffer.lines = vec![
        "Line 1".to_string(),
        "Line 2".to_string(),
        "Line 3".to_string(),
    ];
    
    // Select from middle of line 1 to middle of line 2
    buffer.selection = Some(Selection {
        start_row: 0, start_col: 2,
        end_row: 1, end_col: 4,
    });
    
    let deleted = buffer.delete_selection();
    assert!(deleted);
    assert_eq!(buffer.lines.len(), 2);
    assert_eq!(buffer.lines[0], "Li 2");
    assert_eq!(buffer.lines[1], "Line 3");
    assert_eq!(buffer.cursor.row, 0);
    assert_eq!(buffer.cursor.col, 2);
}

#[test]
fn test_mouse_drag_selection() {
    let mut buffer = EditorBuffer::new();
    buffer.lines = vec!["Test line".to_string()];
    
    // Simulate mouse drag from position (0,1) to (0,5)
    buffer.mouse_state = MouseState::Selecting { start_row: 0, start_col: 1 };
    buffer.handle_mouse_drag(/* coordinates for position (0,5) */);
    
    assert!(buffer.selection.is_some());
    let sel = buffer.selection.unwrap();
    let ((start_row, start_col), (end_row, end_col)) = sel.normalized();
    assert_eq!((start_row, start_col), (0, 1));
    assert_eq!((end_row, end_col), (0, 5));
}
```

---

## Best Practices

### Performance Considerations

1. **Lazy Rendering**: Selection backgrounds are only rendered when selection exists and is active
2. **Efficient Bounds Checking**: Use `saturating_sub()` and `min()` for safe arithmetic
3. **Character Caching**: Consider caching character vectors for frequently accessed lines
4. **Pango Optimization**: Reuse Pango layouts when possible to avoid repeated text measurement

### Memory Safety

1. **Bounds Validation**: Always clamp selection coordinates before use
2. **Option Handling**: Use pattern matching rather than `unwrap()` for selection access
3. **Character Safety**: Convert between byte and character indices carefully
4. **Clone Minimization**: Pass selections by reference when possible

### Unicode Best Practices

1. **Character Boundaries**: Always operate on character (not byte) boundaries
2. **Grapheme Clusters**: Consider implementing grapheme cluster awareness for complex scripts
3. **Bidirectional Text**: Future enhancement should consider RTL text support
4. **Normalization**: Consider Unicode normalization for consistent text handling

### Accessibility

1. **Screen Reader Support**: Ensure selection changes are announced to assistive technologies
2. **High Contrast**: Respect system high contrast settings in selection rendering
3. **Keyboard Navigation**: Provide full keyboard accessibility for all selection operations
4. **Focus Management**: Maintain proper focus states during selection operations

### Error Handling

1. **Graceful Degradation**: Continue functioning even with invalid selection states
2. **Validation Layers**: Validate inputs at multiple levels (input, logic, rendering)
3. **Recovery Mechanisms**: Provide ways to recover from corrupted selection state
4. **Debug Information**: Include comprehensive debug logging for selection operations

---

## Conclusion

The RustEditorKit selection system demonstrates a well-architected approach to text selection with:

- **Strong Unicode Support**: Character-based operations ensure proper handling of international text
- **Cross-Platform Compatibility**: Platform-specific keybindings with shared core logic
- **Extensibility**: Multi-selection infrastructure ready for advanced features
- **Performance**: Efficient rendering with minimal overdraw
- **Safety**: Comprehensive bounds checking and validation

The modular design separates concerns effectively:
- **Core Logic**: Pure selection algorithms independent of UI framework
- **Rendering**: GTK4/Cairo-specific drawing code
- **Input Handling**: Platform-aware input processing
- **Configuration**: Flexible appearance and behavior settings

This architecture provides a solid foundation for implementing advanced text editing features while maintaining code clarity and robustness.
