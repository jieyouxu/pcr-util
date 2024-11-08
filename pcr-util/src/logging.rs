use tracing_subscriber::{filter, reload, Registry};

pub type ReloadHandle = reload::Handle<filter::LevelFilter, Registry>;

pub(crate) fn register_global() -> ReloadHandle {
    use tracing_subscriber::{filter, fmt, prelude::*, reload};
    let filter = filter::LevelFilter::INFO;
    let (filter, reload_handle) = reload::Layer::new(filter);
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .init();
    reload_handle
}
