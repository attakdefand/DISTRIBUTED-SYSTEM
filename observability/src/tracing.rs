use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn init_tracing(_service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize simple tracing
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    Ok(())
}

pub fn shutdown_tracing(_tracer: ()) {
    // Nothing to shutdown for simple tracing
}