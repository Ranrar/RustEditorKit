//! Pointer user input handling (signals, event connections)
//!
//! This module centralizes all mouse/gesture event wiring for `EditorWidget`.

use gtk4::prelude::*;
use crate::widget::editor::EditorWidget;

impl EditorWidget {
	/// Connect mouse event handlers for selection support
	pub fn connect_mouse_signals(&self) {
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

			let mut buf = buffer_click.borrow_mut();
			let metrics_opt = cached_metrics_click.borrow();
			let pango_ctx_opt = cached_pango_ctx_click.borrow();
			if let (Some(metrics), Some(pango_ctx)) = (metrics_opt.as_ref(), pango_ctx_opt.as_ref()) {
				buf.handle_mouse_click(
					x, y, shift_held,
					&metrics.layout,
					pango_ctx,
					&metrics.layout.text_metrics.font_desc
				);
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

