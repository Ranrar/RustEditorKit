//! UI scheduler helpers for posting work to the GTK/GLib main loop.
//!
//! Guarantees:
//! - All GTK/GDK/Pango widget operations must run on the main thread.
//! - Use these helpers to schedule closures/futures or create channels that deliver on the main loop.
//!
//! Examples
//! - Post a one-off UI closure:
//!   ui::ui_invoke(|| {/* touch GTK widgets safely here */});
//! - Spawn a future on the GLib main loop:
//!   ui::ui_spawn(async move { /* await, then update UI */ });
//! - Create a cross-thread channel and attach its receiver to the main loop:
//!   let (tx, rx) = ui::ui_channel::<String>();
//!   ui::ui_attach(rx, move |msg| { /* update UI */ });

use glib::MainContext;
use glib::source::{idle_add, SourceId};
use glib::ControlFlow;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};

/// Invoke a closure on the GLib main context (main thread). May run synchronously
/// if already on the owner thread; otherwise schedules it.
pub fn ui_invoke<F>(f: F)
where
    F: FnOnce() + Send + 'static,
{
    MainContext::default().invoke(f);
}

/// Spawn a Send future onto the GLib main context. Use for async work that must
/// complete on the UI loop (e.g., awaiting then updating widgets).
pub fn ui_spawn<Fut>(fut: Fut)
where
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    MainContext::default().spawn(fut);
}

/// Create a standard multi-producer, single-consumer channel for cross-thread messaging.
///
/// Pair this with [`ui_attach`] to deliver messages on the GTK main loop.
pub fn ui_channel<T>() -> (Sender<T>, Receiver<T>)
where
    T: Send + 'static,
{
    mpsc::channel()
}

/// Attach an `mpsc::Receiver<T>` to the GLib default main loop and invoke `on_msg`
/// for each received message on the UI thread.
///
/// - Messages are drained in small batches per idle tick to avoid starving the loop.
/// - The source automatically removes itself once the sender is disconnected and
///   the receiver is fully drained.
pub fn ui_attach<T>(rx: Receiver<T>, mut on_msg: impl FnMut(T) + Send + 'static) -> SourceId
where
    T: Send + 'static,
{
    let mut rx = Some(rx);
    idle_add(move || {
        // If receiver was already taken (disconnected), stop.
        let Some(receiver) = rx.as_ref() else {
            return ControlFlow::Break;
        };

        // Drain up to N messages per idle iteration.
        const MAX_BATCH: usize = 64;
        let mut processed_any = false;
        for _ in 0..MAX_BATCH {
            match receiver.try_recv() {
                Ok(msg) => {
                    processed_any = true;
                    on_msg(msg);
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(TryRecvError::Disconnected) => {
                    // Drop the receiver and stop scheduling.
                    rx.take();
                    return ControlFlow::Break;
                }
            }
        }

        // If nothing processed, yield to avoid busy-looping.
        if !processed_any {
            // Sleep very briefly to let other sources progress (optional safeguard).
            // std::thread::sleep(std::time::Duration::from_micros(50));
        }

        ControlFlow::Continue
    })
}
