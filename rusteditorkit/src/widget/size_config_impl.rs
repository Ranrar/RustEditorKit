//! Implementation of ConfigurableSize for gtk::DrawingArea

use gtk4::prelude::*;
use gtk4::prelude::WidgetExtManual;
use gtk4::DrawingArea;
use crate::corelogic::size_config::{ConfigurableSize, SizeMode, ContainerMode};
use crate::utilits::size_utils;
use std::cell::RefCell;

thread_local! {
    static CURRENT_MODE: RefCell<Option<SizeMode>> = RefCell::new(None);
}

impl ConfigurableSize for DrawingArea {
    fn configure_size(&self, mode: SizeMode) {
        self.set_can_focus(true);
        match mode {
            SizeMode::Fixed(w, h) => {
                self.set_size_request(w as i32, h as i32);
                self.set_hexpand(false);
                self.set_vexpand(false);
            },
            SizeMode::Minimum(w, h) => {
                self.set_size_request(w as i32, h as i32);
                self.set_hexpand(true);
                self.set_vexpand(true);
            },
            SizeMode::Dynamic => {
                self.set_size_request(-1, -1);
                self.set_hexpand(true);
                self.set_vexpand(true);
            },
            SizeMode::Maximum(w, h) => {
                self.set_hexpand(true);
                self.set_vexpand(true);
                let da = self.clone();
                self.connect_width_request_notify(move |da| {
                    let width = size_utils::clamp_dim(da.width_request(), 0, w as i32);
                    da.set_width_request(width);
                });
                self.connect_height_request_notify(move |da| {
                    let height = size_utils::clamp_dim(da.height_request(), 0, h as i32);
                    da.set_height_request(height);
                });
            },
            SizeMode::MinMax { min_w, min_h, max_w, max_h } => {
                self.set_hexpand(true);
                self.set_vexpand(true);
                let da = self.clone();
                self.connect_width_request_notify(move |da| {
                    let width = size_utils::clamp_dim(da.width_request(), min_w as i32, max_w as i32);
                    da.set_width_request(width);
                });
                self.connect_height_request_notify(move |da| {
                    let height = size_utils::clamp_dim(da.height_request(), min_h as i32, max_h as i32);
                    da.set_height_request(height);
                });
            },
            SizeMode::AspectRatio(ratio) => {
                self.set_hexpand(true);
                self.set_vexpand(true);
                let da = self.clone();
                self.connect_width_request_notify(move |da| {
                    let width = da.width_request();
                    let (w, h) = size_utils::aspect_ratio_dims(width, ratio);
                    da.set_size_request(w, h);
                });
            },
            SizeMode::AutoFitToParent => {
                self.set_hexpand(true);
                self.set_vexpand(true);
                // No signal needed; hexpand/vexpand will auto-fit in most containers
            },
            SizeMode::PercentOfParent(wp, hp) => {
                self.set_hexpand(true);
                self.set_vexpand(true);
                let da = self.clone();
                // No direct signal; can be handled by parent allocation in container-aware logic
                let (w, h) = size_utils::percent_of_parent(&da, wp, hp);
                da.set_size_request(w, h);
            },
            SizeMode::ContainerAware(mode) => {
                let container = size_utils::detect_container(self);
                // For now, just log; can add more logic per container type
                println!("Container detected: {:?}", container);
                self.set_hexpand(true);
                self.set_vexpand(true);
            },
        }
        CURRENT_MODE.with(|m| *m.borrow_mut() = Some(mode));
    }
    fn switch_mode(&self, new_mode: SizeMode) {
        self.configure_size(new_mode);
    }
}
