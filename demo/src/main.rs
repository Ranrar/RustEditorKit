use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use rusteditorkit::editorwidget::editor::EditorWidget;

fn main() {
    let app = Application::builder()
        .application_id("com.example.rusteditorkit.demo")
        .build();

    app.connect_activate(|app| {
        // Create the editor widget
        let editor = EditorWidget::new();
        editor.connect_signals();

        // Enable debug mode for terminal output
        editor.buffer().borrow_mut().debug_mode = true;

        // Optionally, fill with sample text
        {
            let buffer = editor.buffer();
            let mut buf = buffer.borrow_mut();
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
        }

        let window = ApplicationWindow::builder()
            .application(app)
            .title("RustEditorKit Demo")
            .default_width(800)
            .default_height(600)
            .child(editor.widget())
            .build();

        window.show();
        // Explicitly grab focus on the DrawingArea after window is shown
        editor.widget().grab_focus();
    });

    app.run();
}
