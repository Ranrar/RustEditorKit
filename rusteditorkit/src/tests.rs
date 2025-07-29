//! Unit tests for EditorBuffer operations

use super::core::EditorBuffer;

#[cfg(test)]
#[test]
fn test_new_buffer() {
    let buf = EditorBuffer::new();
    assert_eq!(buf.lines.len(), 3);
    assert_eq!(buf.cursor.row, 0);
    assert_eq!(buf.cursor.col, 0);
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
    buf.selection = Some(crate::selection::Selection {
        start_row: 0,
        start_col: 0,
        end_row: 0,
        end_col: 2,
    });
    let cut = buf.cut();
    assert_eq!(cut, "fn");
    buf.cursor.row = 1;
    buf.cursor.col = 0;
    buf.paste(&cut);
    assert!(buf.lines[1].starts_with("fn"));
}
