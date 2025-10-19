use tracing::info;

pub fn init_metrics(_service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Simple metrics logging
    info!("Initializing metrics for service: {}", _service_name);
    Ok(())
}

pub fn shutdown_metrics() {
    // Nothing to shutdown for simple metrics
}