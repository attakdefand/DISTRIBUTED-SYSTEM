use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn init_logging() {
    // Create a subscriber with environment-based filtering
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(true)
        .finish();

    // Set the global default subscriber
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");
}

pub fn log_error(error: &impl std::error::Error) {
    tracing::error!("{}", error);
}

pub fn log_info(message: &str) {
    tracing::info!("{}", message);
}