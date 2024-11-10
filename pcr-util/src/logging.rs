use tracing_subscriber::{Registry, filter, reload};

pub type ReloadHandle = reload::Handle<filter::LevelFilter, Registry>;

pub(crate) fn register_global() -> ReloadHandle {
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{filter, fmt, reload};
    let filter = filter::LevelFilter::INFO;
    let (filter, reload_handle) = reload::Layer::new(filter);

    let fmt_layer = fmt::layer()
        .with_level(true) // don't include levels in formatted output
        .with_target(false) // don't include targets
        .with_thread_ids(false) // include the thread ID of the current thread
        .with_thread_names(false)
        .without_time() // include the name of the current thread
        .compact();

    tracing_subscriber::registry().with(filter).with(fmt_layer).init();
    reload_handle
}
