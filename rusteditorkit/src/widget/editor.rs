use crate::keybinds::{EditorAction, KeyCombo};
// Main EditorWidget implementation using the modular render system
// This widget properly delegates rendering to the src/render/ modules

use gtk4::prelude::*;
use gtk4::{DrawingArea,};
use glib::{ControlFlow,};
use std::cell::RefCell;
use std::rc::Rc;
use crate::corelogic::EditorBuffer;
use crate::imcontext::EditorIMContext;
use crate::render::layout::LayoutMetrics;

/// Main editor widget struct that uses the modular render system
pub struct EditorWidget {
    pub buffer: Rc<RefCell<EditorBuffer>>,
    pub drawing_area: DrawingArea,
    pub im_context: EditorIMContext,
    pub blink_source_id: Rc<RefCell<Option<glib::SourceId>>>,
    pub keymap: std::collections::HashMap<EditorAction, KeyCombo>,
    /// Cached layout metrics and font description for event handlers
    pub cached_metrics: RefCell<Option<CachedEditorMetrics>>,
    /// Cached Pango context for event handlers
    pub cached_pango_ctx: RefCell<Option<gtk4::pango::Context>>,
}

/// Struct to cache layout metrics and font description
#[derive(Clone)]
pub struct CachedEditorMetrics {
    pub layout: crate::render::layout::LayoutMetrics,
}
impl EditorWidget {
    /// Connects a debug handler to print key events and dispatched actions
    pub fn connect_keybind_debug(&self) {
        // Debug mode: Set a flag that the main signal handler can check
        // This avoids adding multiple key controllers to the same widget
        // The actual debug printing will be done in signals.rs
        // For now, we just print that debug mode is enabled
        println!("[KEYBIND DEBUG] Debug mode enabled for keybind events");
    }
    /// Update cursor config and restart blink timer (call after config changes)
    pub fn update_cursor_config(&self) {
        let mut buf = self.buffer.borrow_mut();
        buf.update_cursor_state_from_config();
        // Cancel previous blink timer if any
        if let Some(id) = self.blink_source_id.borrow_mut().take() {
            id.remove();
        }
        // If blinking is disabled, ensure cursor is visible and redraw
        if !buf.config.cursor.cursor_blink {
            buf.cursor_state.visible = true;
            self.drawing_area.queue_draw();
            return;
        }
        let drawing_area = self.drawing_area.clone();
        let buffer_clone = self.buffer.clone();
        let blink_source_id = self.blink_source_id.clone();
        let id = glib::timeout_add_local(std::time::Duration::from_millis(buf.config.cursor.cursor_blink_rate), move || {
            let mut buf = buffer_clone.borrow_mut();
            // Always use latest config
            buf.cursor_state_mut().tick_blink();
            buf.cursor_state_mut().check_restore_after_typing();
            drawing_area.queue_draw();
            // If blink is disabled, stop timer
            if !buf.config.cursor.cursor_blink {
                return ControlFlow::Break;
            }
            ControlFlow::Continue
        });
        *blink_source_id.borrow_mut() = Some(id);
    }
    /// Create a new EditorWidget
    pub fn new() -> Self {
        let buffer = Rc::new(RefCell::new(EditorBuffer::new()));
        let drawing_area = DrawingArea::new();
        let blink_source_id: Rc<RefCell<Option<glib::SourceId>>> = Rc::new(RefCell::new(None));
        // Load platform keymap
        #[cfg(target_os = "linux")]
        let keymap = crate::keybinds::linux::linux_keymap();
        #[cfg(target_os = "macos")]
        let keymap = crate::keybinds::mac::mac_keymap();
        #[cfg(target_os = "windows")]
        let keymap = crate::keybinds::win::win_keymap();

        // Set redraw callback so buffer.request_redraw() triggers UI update
        {
            let da_clone = drawing_area.clone();
            buffer.borrow_mut().redraw_callback = Some(Box::new(move || {
                da_clone.queue_draw();
            }));
        }

    // Use new trait-based API for sizing and focus
    use crate::corelogic::widget_sizing::{ConfigurableSize, SizeMode};
    drawing_area.configure_size(SizeMode::Minimum(400, 300));
    
    // Ensure the drawing area can receive keyboard focus
    drawing_area.set_can_focus(true);
    drawing_area.set_focusable(true);

        // IMContext integration
        let buffer_clone = buffer.clone();
        let im_context = EditorIMContext::new(move |text| {
            println!("IMContext commit: {}", text);
            let mut buf = buffer_clone.borrow_mut();
            for c in text.chars() {
                let row = buf.cursor.row;
                let col = buf.cursor.col;
                if row < buf.lines.len() {
                    let line = &mut buf.lines[row];
                    let byte_idx = line.char_indices().nth(col).map(|(i, _)| i).unwrap_or(line.len());
                    line.insert(byte_idx, c);
                    buf.cursor.col += 1;
                }
            }
            buf.request_redraw();
        });

        // Cursor blinking logic is now managed only by update_cursor_config after config is loaded

        let widget = Self {
            buffer,
            drawing_area,
            im_context,
            blink_source_id,
            keymap,
            cached_metrics: RefCell::new(None),
            cached_pango_ctx: RefCell::new(None),
        };
        widget.update_cursor_config();
        widget.initialize_cache();
        widget
    }

    /// Initialize cached metrics and Pango context immediately after widget creation
    pub fn initialize_cache(&self) {
        // Create a dummy Cairo context for initial metrics
        use cairo::ImageSurface;
        use cairo::Context;
        let surface = ImageSurface::create(cairo::Format::ARgb32, 1, 1).expect("Failed to create dummy surface");
        let ctx = Context::new(&surface).expect("Failed to create Cairo context");
        let buf = self.buffer.borrow();
        let layout = crate::render::layout::LayoutMetrics::calculate(&buf, &ctx);
        let font_cfg = &buf.config.font;
        let font_string = format!("{} {}", font_cfg.font_name(), font_cfg.font_size());
        let font_desc = gtk4::pango::FontDescription::from_string(&font_string);
        let pango_layout = pangocairo::functions::create_layout(&ctx);
        pango_layout.set_font_description(Some(&font_desc));
        let row = buf.cursor.row.min(buf.lines.len().saturating_sub(1));
        let line_text = buf.lines.get(row).cloned().unwrap_or_default();
        pango_layout.set_text(&line_text);
        // Cache metrics for event handlers
        let metrics = CachedEditorMetrics {
            layout,
        };
        *self.cached_metrics.borrow_mut() = Some(metrics);
        // Cache Pango context for event handlers
        let pango_ctx = pango_layout.context().clone();
        *self.cached_pango_ctx.borrow_mut() = Some(pango_ctx);
    }

    /// Get a reference to the buffer (for integration/testing)
    pub fn buffer(&self) -> Rc<RefCell<EditorBuffer>> {
        self.buffer.clone()
    }

    /// Get the GTK4 widget for integration
    pub fn widget(&self) -> &DrawingArea {
        &self.drawing_area
    }

    /// Connect the draw signal using the modular render system
    pub fn connect_draw_signal(&self) {
        let buffer = self.buffer.clone();
        let cached_metrics = self.cached_metrics.clone();
        let cached_pango_ctx = self.cached_pango_ctx.clone();
        
        self.drawing_area.set_draw_func(move |_area, ctx, width, height| {
            
            let buf = buffer.borrow();
            let mut layout = LayoutMetrics::calculate(&buf, ctx);
            
            // Update content size based on layout
            let max_line_length = buf.lines.iter()
                .map(|line| line.len())
                .max()
                .unwrap_or(0);
            let content_width = layout.text_left_offset + (max_line_length as f64 * layout.text_metrics.average_char_width);
            let content_height = buf.lines.len() as f64 * layout.line_height;
            
            crate::render::background::render_background_layer(&buf, ctx, width, height);
            // Text layer must be rendered before other layers as it calculates the line metrics
            crate::render::text::render_text_layer(&buf, ctx, &mut layout);
            crate::render::gutter::render_gutter_layer(&buf, ctx, &layout, height);
            crate::render::highlight::render_highlight_layer(&buf, ctx, &layout, width);
            crate::render::selection::render_selection_layer(&buf, ctx, &layout, width);

            // Cursor rendering
            let font_cfg = &buf.config.font;
            let font_string = format!("{} {}", font_cfg.font_name(), font_cfg.font_size());
            let font_desc = gtk4::pango::FontDescription::from_string(&font_string);
            let pango_layout = pangocairo::functions::create_layout(ctx);
            pango_layout.set_font_description(Some(&font_desc));
            let row = buf.cursor.row.min(buf.lines.len().saturating_sub(1));
            let line_text = buf.lines.get(row).cloned().unwrap_or_default();
            pango_layout.set_text(&line_text);
            // Use unified y-offsets from corelogic/layout.rs
            let mut y_offsets = buf.line_y_offsets(layout.line_height, buf.config.font.font_paragraph_spacing(), layout.top_offset);

            let y_line = y_offsets.get(row).copied().unwrap_or(layout.top_offset);
            // Apply same tab stops for the cursor's layout
            let tabs = layout.build_tab_array(&buf.config);
            pango_layout.set_tabs(Some(&tabs));
            crate::render::cursor::render_cursor_layer(&buf, ctx, &pango_layout, &layout, y_line);

            // Cache metrics for event handlers
            let metrics = CachedEditorMetrics {
                layout,
            };
            *cached_metrics.borrow_mut() = Some(metrics);

            // Cache Pango context for event handlers
            let pango_ctx = pango_layout.context().clone();
            *cached_pango_ctx.borrow_mut() = Some(pango_ctx);
            
            // We'll draw mouse position marker by adding it in the render system
        });
    }

}
