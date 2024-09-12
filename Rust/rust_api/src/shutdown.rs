pub fn register() -> Receiver<()> {
    let signals = Signals::new([SIGHUP, SIGTERM, SIGINT, SIGQUIT]).unwrap();
    signals.handle();
    let (tx, rx): (Sender<()>, Receiver<()>) = oneshot::channel();
    tokio::spawn(handle_signals(signals, tx));
    rx
}

async fn handle_signals(mut signals: Signals, tx: Sender<()>) {
    while let Some(signal) = signals.next().await {
        match signal {
            SIGHUP => {
                // Reload configuration, reopen the log file...etc
            }
            SIGTERM | SIGINT | SIGQUIT => {
                // Gracefully shut down
                let _ = tx.send(());
                return;
            }
            _ => unreachable!(),
        }
    }
}



let rx = shutdown::register();

...
.with_graceful_shutdown(async {
    rx.await.ok(); // This will block until a shutdown signal is received
    info!("Handling graceful shutdown");
    info!("Close resources, drain and shutdown event handler... etc");
    shutdown_tracer_provider();
})
...
