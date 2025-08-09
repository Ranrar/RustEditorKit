//! Implementation of ConfigurableSize for gtk::DrawingArea

use gtk4::prelude::*;
use gtk4::DrawingArea;
use crate::corelogic::widget_sizing::{ConfigurableSize, SizeMode};
use crate::utilits::widget_sizing;
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
                self.connect_width_request_notify(move |da| {
                    let width = widget_sizing::clamp_dim(da.width_request(), 0, w as i32);
                    da.set_width_request(width);
                });
                self.connect_height_request_notify(move |da| {
                    let height = widget_sizing::clamp_dim(da.height_request(), 0, h as i32);
                    da.set_height_request(height);
                });
            },
            SizeMode::MinMax { min_w, min_h, max_w, max_h } => {
                self.set_hexpand(true);
                self.set_vexpand(true);
                self.connect_width_request_notify(move |da| {
                    let width = widget_sizing::clamp_dim(da.width_request(), min_w as i32, max_w as i32);
                    da.set_width_request(width);
                });
                self.connect_height_request_notify(move |da| {
                    let height = widget_sizing::clamp_dim(da.height_request(), min_h as i32, max_h as i32);
                    da.set_height_request(height);
                });
            },
            SizeMode::AspectRatio(ratio) => {
                self.set_hexpand(true);
                self.set_vexpand(true);
                self.connect_width_request_notify(move |da| {
                    let width = da.width_request();
                    let (w, h) = widget_sizing::aspect_ratio_dims(width, ratio);
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
                let (w, h) = widget_sizing::percent_of_parent(self, wp, hp);
                self.set_size_request(w, h);
            },
            SizeMode::ContainerAware(_mode) => {
                let container = widget_sizing::detect_container(self);
                match container {
                    crate::corelogic::widget_sizing::ContainerMode::Box => {
                        self.set_hexpand(true);
                        self.set_vexpand(true);
                        println!("ContainerAware: Box detected, using hexpand/vexpand");
                    },
                    crate::corelogic::widget_sizing::ContainerMode::Grid => {
                        self.set_hexpand(true);
                        self.set_vexpand(true);
                        println!("ContainerAware: Grid detected, using hexpand/vexpand");
                    },
                    crate::corelogic::widget_sizing::ContainerMode::Overlay => {
                        self.set_hexpand(true);
                        self.set_vexpand(true);
                        println!("ContainerAware: Overlay detected, using hexpand/vexpand");
                    },
                    crate::corelogic::widget_sizing::ContainerMode::Other => {
                        self.set_hexpand(true);
                        self.set_vexpand(true);
                        println!("ContainerAware: Other/Unknown container, using hexpand/vexpand");
                    },
                }
            },
        }
        CURRENT_MODE.with(|m| *m.borrow_mut() = Some(mode));
    }
    fn switch_mode(&self, new_mode: SizeMode) {
        self.configure_size(new_mode);
    }
}
