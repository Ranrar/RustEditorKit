//! Pointer user input handling (signals, event connections)
//!
//! This module centralizes all mouse/gesture event wiring for `EditorWidget`.

use gtk4::prelude::*;
use crate::widget::editor::EditorWidget;

impl EditorWidget {
	/// Connect mouse event handlers for selection support
	pub fn connect_mouse_signals(&self) {
		// Create a tracking controller to follow mouse position for debugging
		let mouse_motion = gtk4::EventControllerMotion::new();
		let buffer_motion = self.buffer().clone();
		mouse_motion.connect_motion(move |_, x, y| {
			// Update debug info in the buffer
			let mut buf = buffer_motion.borrow_mut();
			buf.last_mouse_x = x;
			buf.last_mouse_y = y;
			// Don't trigger redraws on motion - would be too intensive
		});
		self.drawing_area.add_controller(mouse_motion);
        
        // Add scroll wheel controller
        let scroll_controller = gtk4::EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);
        let buffer_scroll = self.buffer().clone();
        scroll_controller.connect_scroll(move |_, _dx, dy| {
            let mut buf = buffer_scroll.borrow_mut();
            
            // Calculate scroll direction and amount
            let scroll_dir = if dy > 0.0 { 1 } else { -1 };
            let scroll_amount = (dy.abs().ceil() as usize).max(1);
            
            // Update scroll offset
            if scroll_dir > 0 {
                // Scroll down: increase offset
                buf.scroll_offset = buf.scroll_offset.saturating_add(scroll_amount);
            } else {
                // Scroll up: decrease offset
                buf.scroll_offset = buf.scroll_offset.saturating_sub(scroll_amount);
            }
            
            // Limit scroll offset to valid range
            buf.scroll_offset = buf.scroll_offset.min(buf.lines.len().saturating_sub(1));
            
            println!("[SCROLL DEBUG] Scrolled {} lines, new offset: {}", 
                   if scroll_dir > 0 { format!("+{}", scroll_amount) } else { format!("-{}", scroll_amount) }, 
                   buf.scroll_offset);
            
            // Request redraw with updated scroll position
            buf.request_redraw();
            
            // Returning false allows the event to propagate
            gtk4::glib::Propagation::Proceed
        });
        self.drawing_area.add_controller(scroll_controller);
		// Primary mouse button controller (for clicking and dragging)
		let buffer_primary = self.buffer().clone();
		let cached_metrics = self.cached_metrics.clone();
		let cached_pango_ctx = self.cached_pango_ctx.clone();
		let mouse_primary = gtk4::GestureClick::new();
		mouse_primary.set_button(1); // Left mouse button

		// Handle single clicks
		let buffer_click = buffer_primary.clone();
		let cached_metrics_click = cached_metrics.clone();
		let cached_pango_ctx_click = cached_pango_ctx.clone();
		mouse_primary.connect_pressed(move |gesture, _n_press, x, y| {
			let state = gesture.current_event_state();
			let shift_held = state.contains(gtk4::gdk::ModifierType::SHIFT_MASK);

			println!("[MOUSE DEBUG] Click at ({:.1}, {:.1}), shift: {}", x, y, shift_held);
            
            // We need to access the EditorWidget to update mouse_debug_info
            // This will be done when we modify the rendering system

			let mut buf = buffer_click.borrow_mut();
			let metrics_opt = cached_metrics_click.borrow();
			let pango_ctx_opt = cached_pango_ctx_click.borrow();
			if let (Some(metrics), Some(pango_ctx)) = (metrics_opt.as_ref(), pango_ctx_opt.as_ref()) {
				// Calculate line index using global line_height for debugging
                let top_offset = metrics.layout.top_offset;
                let line_height = metrics.layout.line_height;
                let relative_y = y - top_offset;
                let global_line_index = (relative_y / line_height).floor() as usize;
                println!("[MOUSE DEBUG] Computed line index (global line_height): {}", global_line_index);
                
				// Get the cursor position before the click
				let old_row = buf.cursor.row;
				let old_col = buf.cursor.col;
				
				buf.handle_mouse_click(
					x, y, shift_held,
					&metrics.layout,
					pango_ctx,
					&metrics.layout.text_metrics.font_desc
				);
                
                // Print cursor position after placement with before/after comparison
                println!("[MOUSE DEBUG] Caret moved from ({},{}) to ({},{})", 
                         old_row, old_col, buf.cursor.row, buf.cursor.col);
                println!("[MOUSE DEBUG] Expected line: {}, Actual line: {}", 
                         global_line_index, buf.cursor.row);
                
				buf.request_redraw();
			} else {
				println!("[ERROR] Mouse event: metrics or pango context cache missing. Mouse event ignored.");
			}
		});

		// Handle double and triple clicks
		let buffer_multi = buffer_primary.clone();
		let cached_metrics_multi = cached_metrics.clone();
		let cached_pango_ctx_multi = cached_pango_ctx.clone();
		mouse_primary.connect_released(move |gesture, n_press, x, y| {
			let _ = gesture; // suppress unused variable warning
			let mut buf = buffer_multi.borrow_mut();
			let metrics_opt = cached_metrics_multi.borrow();
			let pango_ctx_opt = cached_pango_ctx_multi.borrow();
			if let (Some(metrics), Some(pango_ctx)) = (metrics_opt.as_ref(), pango_ctx_opt.as_ref()) {
				// Robust click count handling: always expand selection as expected
				match n_press {
					2 => {
						println!("[MOUSE DEBUG] Double-click at ({:.1}, {:.1})", x, y);
						buf.handle_double_click(
							x, y,
							&metrics.layout,
							pango_ctx,
							&metrics.layout.text_metrics.font_desc
						);
						buf.request_redraw();
					},
					3 => {
						println!("[MOUSE DEBUG] Triple-click at ({:.1}, {:.1})", x, y);
						buf.handle_triple_click(
							x, y,
							&metrics.layout,
							pango_ctx,
							&metrics.layout.text_metrics.font_desc
						);
						buf.request_redraw();
					},
					_ => {
						// For single click, ensure selection is cleared and cursor is set
						buf.handle_mouse_click(
							x, y,
							false,
							&metrics.layout,
							pango_ctx,
							&metrics.layout.text_metrics.font_desc
						);
						buf.request_redraw();
					}
				}
			} else {
				println!("[ERROR] Mouse event: metrics or pango context cache missing. Mouse event ignored.");
			}
		});

		self.drawing_area.add_controller(mouse_primary);

		// Drag controller for selection
		let buffer_drag = self.buffer().clone();
		let drag_controller = gtk4::GestureDrag::new();
        
		let buffer_drag_update = buffer_drag.clone();
		let cached_metrics_drag = cached_metrics.clone();
		let cached_pango_ctx_drag = cached_pango_ctx.clone();
		drag_controller.connect_drag_update(move |drag_ctrl, _x, _y| {
			// Get absolute position
			if let Some((_start_x, _start_y)) = drag_ctrl.start_point() {
				if let Some((dx, dy)) = drag_ctrl.offset() {
					let current_x = _start_x + dx;
					let current_y = _start_y + dy;

					println!("[MOUSE DEBUG] Drag to ({:.1}, {:.1})", current_x, current_y);

					let mut buf = buffer_drag_update.borrow_mut();
					let metrics_opt = cached_metrics_drag.borrow();
					let pango_ctx_opt = cached_pango_ctx_drag.borrow();
					if let (Some(metrics), Some(pango_ctx)) = (metrics_opt.as_ref(), pango_ctx_opt.as_ref()) {
						buf.handle_mouse_drag(
							current_x, current_y,
							&metrics.layout,
							pango_ctx,
							&metrics.layout.text_metrics.font_desc
						);
						buf.request_redraw();
					} else {
						println!("[ERROR] Mouse drag: metrics or pango context cache missing. Mouse drag ignored.");
					}
				}
			}
		});

		let buffer_drag_end = buffer_drag.clone();
		drag_controller.connect_drag_end(move |_, _x, _y| {
			println!("[MOUSE DEBUG] Drag ended");
			let mut buf = buffer_drag_end.borrow_mut();
			buf.handle_mouse_release();
		});

		self.drawing_area.add_controller(drag_controller);
	}
}

