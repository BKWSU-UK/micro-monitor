use tracing_subscriber::{EnvFilter};

pub fn init_tracing() {
    // Respect RUST_LOG if set, else default to info.
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)        // hide module path if you want
        .with_thread_ids(false)
        .with_line_number(true)
        .init();
}