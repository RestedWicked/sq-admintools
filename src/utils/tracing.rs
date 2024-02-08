use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn sq_trace() {
    std::env::set_var("RUST_LOG", "trace");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sq-admintools=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
