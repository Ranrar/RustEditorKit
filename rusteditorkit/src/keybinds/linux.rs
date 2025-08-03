use super::editor_action::{EditorAction, KeyCombo};
use std::collections::HashMap;

pub fn linux_keymap() -> HashMap<EditorAction, KeyCombo> {
    use EditorAction::*;
    let mut map = HashMap::new();
    // === Navigation ===
    map.insert(MoveCursorLeft, KeyCombo::new("Left", false, false, false));
    map.insert(MoveCursorRight, KeyCombo::new("Right", false, false, false));
    map.insert(MoveCursorUp, KeyCombo::new("Up", false, false, false));
    map.insert(MoveCursorDown, KeyCombo::new("Down", false, false, false));
    map.insert(MoveCursorStartOfLine, KeyCombo::new("Home", false, false, false));
    map.insert(MoveCursorEndOfLine, KeyCombo::new("End", false, false, false));
    map.insert(MoveCursorPageUp, KeyCombo::new("PageUp", false, false, false));
    map.insert(MoveCursorPageDown, KeyCombo::new("PageDown", false, false, false));
    // === Selection ===
    map.insert(SelectLeft, KeyCombo::new("Left", false, true, false));
    map.insert(SelectRight, KeyCombo::new("Right", false, true, false));
    map.insert(SelectUp, KeyCombo::new("Up", false, true, false));
    map.insert(SelectDown, KeyCombo::new("Down", false, true, false));
    map.insert(SelectAll, KeyCombo::new("a", true, false, false));
    // === Editing ===
    map.insert(CopySelection, KeyCombo::new("c", true, false, false));
    map.insert(CutSelection, KeyCombo::new("x", true, false, false));
    map.insert(PasteClipboard, KeyCombo::new("v", true, false, false));
    map.insert(DeleteLeft, KeyCombo::new("Backspace", false, false, false));
    map.insert(DeleteRight, KeyCombo::new("Delete", false, false, false));
    map.insert(Undo, KeyCombo::new("z", true, false, false));
    map.insert(Redo, KeyCombo::new("y", true, false, false));
    // === Indentation and Tabulation ===
    map.insert(Indent, KeyCombo::new("Tab", false, false, false));
    map.insert(Unindent, KeyCombo::new("Tab", false, true, false));
    map.insert(ConvertTabsToSpaces, KeyCombo::new("8", true, true, false));
    map.insert(ToggleSoftTabs, KeyCombo::new("t", true, true, false));
    // === Line Operations ===
    map.insert(InsertNewline, KeyCombo::new("Return", false, false, false));
    // === Escape and Cancel ===
    map.insert(ClearSelection, KeyCombo::new("Escape", false, false, false));
    // === File Operations ===
    map.insert(NewFile, KeyCombo::new("n", true, false, false));
    map.insert(OpenFile, KeyCombo::new("o", true, false, false));
    map.insert(SaveFile, KeyCombo::new("s", true, false, false));
    map.insert(SaveAs, KeyCombo::new("s", true, true, false));
    // === Search & Replace ===
    map.insert(Find, KeyCombo::new("f", true, false, false));
    map.insert(FindNext, KeyCombo::new("F3", false, false, false));
    map.insert(Replace, KeyCombo::new("h", true, false, false));
    map
}
