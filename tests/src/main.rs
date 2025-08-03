#[cfg(test)]
mod keybind_tests {
    use rusteditorkit::keybinds::{EditorAction, KeyCombo};
    use rusteditorkit::keybinds::linux::linux_keymap;
    use rusteditorkit::keybinds::mac::mac_keymap;
    use rusteditorkit::keybinds::win::win_keymap;

    fn test_keymap(name: &str, keymap: std::collections::HashMap<EditorAction, KeyCombo>) {
        println!("\n--- Testing keymap: {} ---", name);
        for (action, combo) in keymap.iter() {
            // Emulate a key event: convert KeyCombo to keyval/modifiers
            // For simplicity, we use the key string and modifiers directly
            println!("[DEBUG] key name for {:?}: {}", action, combo.key);
            let keyval = gdk_keyval_from_name(combo.key);
            let mut state = gtk4::gdk::ModifierType::empty();
            if combo.ctrl { state |= gtk4::gdk::ModifierType::CONTROL_MASK; }
            if combo.shift { state |= gtk4::gdk::ModifierType::SHIFT_MASK; }
            if combo.alt { state |= gtk4::gdk::ModifierType::ALT_MASK; }
            let emu_combo = KeyCombo::from_gtk_event(keyval, state);
            if keyval == 0xffffff {
                println!("[SKIP] Unmappable key for {:?}: {:?}", action, combo);
                continue;
            }
            assert_eq!(&emu_combo, combo, "KeyCombo mismatch for {:?} in {}", action, name);
            println!("✓ {:?}: {:?} => {:?}", action, combo, emu_combo);
        }
    }

    // Helper: get keyval from key name using gdk4_sys
    fn gdk_keyval_from_name(name: &str) -> u32 {
        // Fallback for common keys if GDK lookup fails
        let name_clean: String = name.chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_ascii_lowercase();
        println!("[FALLBACK DEBUG] cleaned key name: {}", name_clean);
        let fallback = match name_clean.as_str() {
            "backspace" => 0xff08,
            "tab" => 0xff09,
            "return" | "enter" => 0xff0d,
            "escape" => 0xff1b,
            "left" => 0xff51,
            "up" => 0xff52,
            "right" => 0xff53,
            "down" => 0xff54,
            "delete" => 0xffff,
            "home" => 0xff50,
            "end" => 0xff57,
            "pageup" | "prior" => 0xff55,
            "pagedown" | "next" => 0xff56,
            // Also match with underscore for completeness
            "page_down" => 0xff56,
            "page_up" => 0xff55,
            _ => 0xffffff,
        };
        unsafe {
            let cstr = std::ffi::CString::new(name).unwrap();
            let val = gdk4_sys::gdk_keyval_from_name(cstr.as_ptr());
            if val == 0 {
                if fallback != 0xffffff {
                    println!("[FALLBACK USED] {} -> {:#x}", name, fallback);
                    fallback
                } else {
                    println!("[FALLBACK UNKNOWN] {} -> {:#x}", name, fallback);
                    0xffffff
                }
            } else {
                val
            }
        }
    }

    #[test]
    fn test_all_platform_keymaps() {
        test_keymap("Linux", linux_keymap());
        test_keymap("macOS", mac_keymap());
        test_keymap("Windows", win_keymap());
    }
}
use rusteditorkit::corelogic::EditorBuffer;

fn main() {
    let mut buf = EditorBuffer::new();
    
    // Override with the same 9 lines from demo
    buf.lines = vec![
        "fn main() {".to_string(),
        "    println!(\"Hello, world!\");".to_string(),
        "}".to_string(),
        "".to_string(),
        "// This is a demo of RustEditorKit".to_string(),
        "let x = 42;".to_string(),
        "for i in 0..x {".to_string(),
        "    println!(\"Line {}\", i);".to_string(),
        "}".to_string(),
    ];
    
    println!("=== Buffer Analysis ===");
    println!("Total lines: {}", buf.lines.len());
    println!("Lines with indices:");
    for (i, line) in buf.lines.iter().enumerate() {
        println!("  Index {}: '{}' (Gutter displays: {})", i, line, i + 1);
    }
    
    println!("\n=== Initial Cursor Position ===");
    println!("Cursor at row={}, col={}", buf.cursor.row, buf.cursor.col);
    println!("This displays as gutter line: {}", buf.cursor.row + 1);
    
    println!("\n=== Testing Cursor Movement ===");
    let mut moves = 0;
    while buf.cursor.row + 1 < buf.lines.len() {
        println!("Before move: cursor at row {}", buf.cursor.row);
        buf.move_down();
        moves += 1;
        println!("After move {}: cursor at row {} (gutter line {})", moves, buf.cursor.row, buf.cursor.row + 1);
    }
    
    println!("\n=== Final State ===");
    println!("Final cursor position: row={}, col={}", buf.cursor.row, buf.cursor.col);
    println!("Content at final position: '{}'", buf.lines[buf.cursor.row]);
    println!("This is gutter line: {}", buf.cursor.row + 1);
    println!("Maximum possible row index: {} (total {} lines)", buf.lines.len() - 1, buf.lines.len());
    
    // Try one more move (should fail)
    println!("\n=== Testing Beyond Limit ===");
    let old_row = buf.cursor.row;
    buf.move_down();
    if buf.cursor.row == old_row {
        println!("✓ Cannot move beyond line {} (gutter line {})", buf.cursor.row, buf.cursor.row + 1);
    } else {
        println!("✗ Unexpectedly moved to line {}", buf.cursor.row);
    }
}
