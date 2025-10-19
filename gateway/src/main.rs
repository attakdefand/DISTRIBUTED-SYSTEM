use axum::{
    routing::{get},
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
};
use http::header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE};
use tracing_subscriber;

mod handlers;
mod config;
mod error;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = config::Config::from_env().expect("Failed to load configuration");

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(handlers::health_check))
        // Simplified routes for now
        .route("/api/users/*path", get(handlers::user_service_proxy))
        .route("/api/orders/*path", get(handlers::order_service_proxy))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]))
        .with_state(config);

    // Run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Gateway listening on {}", addr);
    
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