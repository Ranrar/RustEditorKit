#[cfg(test)]
mod ui_channel_smoke {
    use rusteditorkit::ui;
    use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
    use std::time::Duration;

    // Headless smoke test: send N messages from a worker and count them on the GLib main loop.
    #[test]
    fn channel_delivers_messages_on_main_context() {
        let n = 10usize;
        let (tx, rx) = ui::ui_channel::<usize>();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        // Attach receiver to main loop and count messages.
        let _source = ui::ui_attach(rx, move |_val| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Spawn a worker thread to send messages.
        std::thread::spawn(move || {
            for i in 0..n {
                let _ = tx.send(i);
                std::thread::sleep(Duration::from_millis(5));
            }
        });

        // Drive the GLib main context manually until we observe all messages or timeout.
        let ctx = glib::MainContext::default();
        let start = std::time::Instant::now();
        while counter.load(Ordering::SeqCst) < n {
            // Iterate pending events; if none pending, block briefly.
            while ctx.iteration(false) {}
            if start.elapsed() > Duration::from_secs(2) {
                break;
            }
            // Let other threads progress.
            std::thread::sleep(Duration::from_millis(1));
        }

        assert_eq!(counter.load(Ordering::SeqCst), n, "Did not receive all messages on main context");
    }
}
