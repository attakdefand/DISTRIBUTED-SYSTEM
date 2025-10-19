use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber;

mod handlers;
mod services;
mod repositories;
mod models;
mod config;
mod error;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = config::Config::from_env().expect("Failed to load configuration");

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/users", post(handlers::create_user))
        .route("/users/:id", get(handlers::get_user_by_id))
        .with_state(config);

    // Run our app with hyper, listening globally on port 3001
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!("User service listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    
    tracing::info!("Signal received, starting graceful shutdown");
}