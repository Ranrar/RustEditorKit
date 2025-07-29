//! Unit tests for EditorBuffer operations

use super::core::EditorBuffer;

#[cfg(test)]
#[test]
fn test_new_buffer() {
    let buf = EditorBuffer::new();
    assert_eq!(buf.lines.len(), 3);
    assert_eq!(buf.cursor_row, 0);
    assert_eq!(buf.cursor_col, 0);
}

#[cfg(test)]
#[test]
fn test_insert_and_undo() {
    let mut buf = EditorBuffer::new();
    buf.lines[0].push_str(" // comment");
    buf.push_undo();
    buf.lines[0].push_str(" more");
    buf.undo();
    assert_eq!(buf.lines[0], "fn main() { // comment");
}

#[cfg(test)]
#[test]
fn test_cut_copy_paste() {
    let mut buf = EditorBuffer::new();
    buf.selection_start = Some((0, 0));
    buf.selection_end = Some((0, 2));
    let cut = buf.cut();
    assert_eq!(cut, "fn");
    buf.cursor_row = 1;
    buf.cursor_col = 0;
    buf.paste(&cut);
    assert!(buf.lines[1].starts_with("fn"));
}
