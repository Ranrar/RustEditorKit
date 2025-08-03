
use gtk4::gio;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, PopoverMenuBar};
use rusteditorkit::widget::EditorWidget;

/// Create the minimal menu structure
fn create_menu() -> PopoverMenuBar {
    let menu_model = gio::Menu::new();
    
    // File menu
    let file_menu = gio::Menu::new();
    file_menu.append(Some("New"), Some("app.new"));
    file_menu.append(Some("Open…"), Some("app.open"));
    file_menu.append(Some("Save"), Some("app.save"));
    file_menu.append(Some("Save As…"), Some("app.save_as"));
    file_menu.append(Some("Quit"), Some("app.quit"));
    menu_model.append_submenu(Some("File"), &file_menu);
    
    // Edit menu
    let edit_menu = gio::Menu::new();
    edit_menu.append(Some("Undo"), Some("app.undo"));
    edit_menu.append(Some("Redo"), Some("app.redo"));
    edit_menu.append(Some("Cut"), Some("app.cut"));
    edit_menu.append(Some("Copy"), Some("app.copy"));
    edit_menu.append(Some("Paste"), Some("app.paste"));
    edit_menu.append(Some("Indent"), Some("app.indent"));
    edit_menu.append(Some("Unindent"), Some("app.unindent"));
    edit_menu.append(Some("Select All"), Some("app.select_all"));
    
    // Font submenu
    let font_menu = gio::Menu::new();
    font_menu.append(Some("Choose Font…"), Some("app.choose_font"));
    font_menu.append(Some("Increase Font Size"), Some("app.increase_font"));
    font_menu.append(Some("Decrease Font Size"), Some("app.decrease_font"));
    font_menu.append(Some("Reset Font Size"), Some("app.reset_font"));
    edit_menu.append_submenu(Some("Font"), &font_menu);
    
    menu_model.append_submenu(Some("Edit"), &edit_menu);
    
    // Help menu
    let help_menu = gio::Menu::new();
    help_menu.append(Some("About"), Some("app.about"));
    menu_model.append_submenu(Some("Help"), &help_menu);
    
    PopoverMenuBar::from_model(Some(&menu_model))
}

fn main() {
    let app = Application::builder()
        .application_id("com.example.rusteditorkit.demo")
        .build();

    app.connect_activate(|app| {
        use std::rc::Rc;
        
        // Create editor
        let editor = Rc::new(EditorWidget::new());
        editor.connect_signals();
        editor.load_config_from_file("demo/src/config.ron");
        
        // Add some sample text
        {
            let buffer = editor.buffer();
            let mut buf = buffer.borrow_mut();
            buf.lines = vec![
                "// RustEditorKit Demo".to_string(),
                "fn main() {".to_string(),
                "    println!(\"Hello, world!\");".to_string(),
                "}".to_string(),
                "".to_string(),
                "// Multi-language support:".to_string(),
                "// Emoji: 😀 😁 😂 🤔".to_string(),
                "// Chinese/Japanese: 漢字 かな カタカナ".to_string(),
                "// Accents: é ü ñ å".to_string(),
            ];
        }
        
        // Create menu
        let menu_bar = create_menu();
        
        // Register actions
        register_actions(app, editor.clone());
        
        // Layout
        let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        vbox.append(&menu_bar);
        vbox.append(editor.widget());
        
        // Set the editor widget minimum size
        editor.widget().set_size_request(800, 600);
        
        let window = ApplicationWindow::builder()
            .application(app)
            .title("RustEditorKit Demo")
            .default_width(800)
            .default_height(650)  // Slightly larger to account for menu bar
            .child(&vbox)
            .build();

        // Set up keyboard shortcuts
        setup_keyboard_shortcuts(app);
        
        window.show();
        editor.widget().grab_focus();
    });

    app.run();
}

/// Register all menu actions
fn register_actions(app: &Application, editor: std::rc::Rc<EditorWidget>) {
    use rusteditorkit::keybinds::EditorAction;
    
    // File actions
    let new_action = gio::SimpleAction::new("new", None);
    new_action.connect_activate({
        let editor = editor.clone();
        move |_, _| {
            let buffer = editor.buffer();
            let mut buf = buffer.borrow_mut();
            buf.handle_editor_action(EditorAction::NewFile);
        }
    });
    app.add_action(&new_action);
    
    let open_action = gio::SimpleAction::new("open", None);
    open_action.connect_activate({
        let editor = editor.clone();
        move |_, _| {
            let buffer = editor.buffer();
            let mut buf = buffer.borrow_mut();
            buf.handle_editor_action(EditorAction::OpenFile);
        }
    });
    app.add_action(&open_action);
    
    let save_action = gio::SimpleAction::new("save", None);
    save_action.connect_activate({
        let editor = editor.clone();
        move |_, _| {
            let buffer = editor.buffer();
            let mut buf = buffer.borrow_mut();
            buf.handle_editor_action(EditorAction::SaveFile);
        }
    });
    app.add_action(&save_action);
    
    let save_as_action = gio::SimpleAction::new("save_as", None);
    save_as_action.connect_activate({
        let editor = editor.clone();
        move |_, _| {
            let buffer = editor.buffer();
            let mut buf = buffer.borrow_mut();
            buf.handle_editor_action(EditorAction::SaveAs);
        }
    });
    app.add_action(&save_as_action);
    
    let quit_action = gio::SimpleAction::new("quit", None);
    quit_action.connect_activate({
        let app = app.clone();
        move |_, _| app.quit()
    });
    app.add_action(&quit_action);
    
    // Edit actions
    let undo_action = gio::SimpleAction::new("undo", None);
    undo_action.connect_activate({
        let editor = editor.clone();
        move |_, _| {
            let buffer = editor.buffer();
            let mut buf = buffer.borrow_mut();
            buf.handle_editor_action(EditorAction::Undo);
        }
    });
    app.add_action(&undo_action);
    
    let redo_action = gio::SimpleAction::new("redo", None);
    redo_action.connect_activate({
        let editor = editor.clone();
        move |_, _| {
            let buffer = editor.buffer();
            let mut buf = buffer.borrow_mut();
            buf.handle_editor_action(EditorAction::Redo);
        }
    });
    app.add_action(&redo_action);
    
    let cut_action = gio::SimpleAction::new("cut", None);
    cut_action.connect_activate({
        let editor = editor.clone();
        move |_, _| {
            let buffer = editor.buffer();
            let mut buf = buffer.borrow_mut();
            buf.handle_editor_action(EditorAction::CutSelection);
        }
    });
    app.add_action(&cut_action);
    
    let copy_action = gio::SimpleAction::new("copy", None);
    copy_action.connect_activate({
        let editor = editor.clone();
        move |_, _| {
            let buffer = editor.buffer();
            let mut buf = buffer.borrow_mut();
            buf.handle_editor_action(EditorAction::CopySelection);
        }
    });
    app.add_action(&copy_action);
    
    let paste_action = gio::SimpleAction::new("paste", None);
    paste_action.connect_activate({
        let editor = editor.clone();
        move |_, _| {
            let buffer = editor.buffer();
            let mut buf = buffer.borrow_mut();
            buf.handle_editor_action(EditorAction::PasteClipboard);
        }
    });
    app.add_action(&paste_action);
    
    let indent_action = gio::SimpleAction::new("indent", None);
    indent_action.connect_activate({
        let editor = editor.clone();
        move |_, _| {
            let buffer = editor.buffer();
            let mut buf = buffer.borrow_mut();
            buf.handle_editor_action(EditorAction::Indent);
        }
    });
    app.add_action(&indent_action);
    
    let unindent_action = gio::SimpleAction::new("unindent", None);
    unindent_action.connect_activate({
        let editor = editor.clone();
        move |_, _| {
            let buffer = editor.buffer();
            let mut buf = buffer.borrow_mut();
            buf.handle_editor_action(EditorAction::Unindent);
        }
    });
    app.add_action(&unindent_action);
    
    let select_all_action = gio::SimpleAction::new("select_all", None);
    select_all_action.connect_activate({
        let editor = editor.clone();
        move |_, _| {
            let buffer = editor.buffer();
            let mut buf = buffer.borrow_mut();
            buf.handle_editor_action(EditorAction::SelectAll);
        }
    });
    app.add_action(&select_all_action);
    
    // Font actions (placeholder implementations)
    let choose_font_action = gio::SimpleAction::new("choose_font", None);
    choose_font_action.connect_activate(|_, _| {
        println!("Font chooser dialog would open here");
    });
    app.add_action(&choose_font_action);
    
    let increase_font_action = gio::SimpleAction::new("increase_font", None);
    increase_font_action.connect_activate(|_, _| {
        println!("Increase font size");
        // TODO: Implement font size increase
    });
    app.add_action(&increase_font_action);
    
    let decrease_font_action = gio::SimpleAction::new("decrease_font", None);
    decrease_font_action.connect_activate(|_, _| {
        println!("Decrease font size");
        // TODO: Implement font size decrease
    });
    app.add_action(&decrease_font_action);
    
    let reset_font_action = gio::SimpleAction::new("reset_font", None);
    reset_font_action.connect_activate(|_, _| {
        println!("Reset font size");
        // TODO: Implement font size reset
    });
    app.add_action(&reset_font_action);
    
    // Help actions
    let about_action = gio::SimpleAction::new("about", None);
    about_action.connect_activate(|_, _| {
        println!("About RustEditorKit Demo\n\nA minimal text editor demo showcasing RustEditorKit.");
    });
    app.add_action(&about_action);
}

/// Set up keyboard shortcuts
fn setup_keyboard_shortcuts(app: &Application) {
    // File shortcuts
    app.set_accels_for_action("app.new", &["<Ctrl>n"]);
    app.set_accels_for_action("app.open", &["<Ctrl>o"]);
    app.set_accels_for_action("app.save", &["<Ctrl>s"]);
    app.set_accels_for_action("app.save_as", &["<Ctrl><Shift>s"]);
    app.set_accels_for_action("app.quit", &["<Ctrl>q"]);
    
    // Edit shortcuts
    app.set_accels_for_action("app.undo", &["<Ctrl>z"]);
    app.set_accels_for_action("app.redo", &["<Ctrl>y"]);
    app.set_accels_for_action("app.cut", &["<Ctrl>x"]);
    app.set_accels_for_action("app.copy", &["<Ctrl>c"]);
    app.set_accels_for_action("app.paste", &["<Ctrl>v"]);
    app.set_accels_for_action("app.indent", &["Tab"]);
    app.set_accels_for_action("app.unindent", &["<Shift>Tab"]);
    app.set_accels_for_action("app.select_all", &["<Ctrl>a"]);
    
    // Help shortcuts
    app.set_accels_for_action("app.about", &["F1"]);
}
