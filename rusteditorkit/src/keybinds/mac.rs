use super::editor_action::{EditorAction, KeyCombo};
use std::collections::HashMap;

pub fn mac_keymap() -> HashMap<EditorAction, KeyCombo> {
    use EditorAction::*;
    let mut map = HashMap::new();
    // On macOS, Command is usually used instead of Ctrl
    // For simplicity, we use ctrl=true to represent Command
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
    map.insert(SelectAll, KeyCombo::new("A", true, false, false));
    // === Editing ===
    map.insert(CopySelection, KeyCombo::new("C", true, false, false));
    map.insert(CutSelection, KeyCombo::new("X", true, false, false));
    map.insert(PasteClipboard, KeyCombo::new("V", true, false, false));
    map.insert(DeleteLeft, KeyCombo::new("Backspace", false, false, false));
    map.insert(DeleteRight, KeyCombo::new("Delete", false, false, false));
    map.insert(Undo, KeyCombo::new("Z", true, false, false));
    map.insert(Redo, KeyCombo::new("Y", true, false, false));
    // === Indentation and Tabulation ===
    map.insert(Indent, KeyCombo::new("Tab", false, false, false));
    map.insert(Unindent, KeyCombo::new("Tab", false, true, false));
    map.insert(ConvertTabsToSpaces, KeyCombo::new("8", true, true, false));
    map.insert(ToggleSoftTabs, KeyCombo::new("T", true, true, false));
    // === Escape and Cancel ===
    map.insert(Escape, KeyCombo::new("Escape", false, false, false));
    map.insert(ClearSelection, KeyCombo::new("Escape", false, false, false));
    map.insert(ExitInsertMode, KeyCombo::new("Escape", false, false, false));
    map.insert(CloseOverlay, KeyCombo::new("Escape", false, false, false));
    // === File Operations ===
    map.insert(NewFile, KeyCombo::new("N", true, false, false));
    map.insert(OpenFile, KeyCombo::new("O", true, false, false));
    map.insert(SaveFile, KeyCombo::new("S", true, false, false));
    map.insert(SaveAs, KeyCombo::new("S", true, true, false));
    // === Search & Replace ===
    map.insert(Find, KeyCombo::new("F", true, false, false));
    map.insert(FindNext, KeyCombo::new("F3", false, false, false));
    map.insert(Replace, KeyCombo::new("H", true, false, false));
    map
}
