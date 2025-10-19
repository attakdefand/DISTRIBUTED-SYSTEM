use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
};
use http::header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE};
use std::net::SocketAddr;
use tracing_subscriber;

mod handlers;
mod services;
mod repositories;
mod models;
mod config;
mod error;
mod uow;
mod cache;
mod auth;
mod jobs;
mod migration;
mod rbac;

#[cfg(test)]
mod integration_tests;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = config::Config::from_env().expect("Failed to load configuration");

    // Run database migrations
    migration::MigrationRunner::run_migrations(&config.database_url)
        .await
        .expect("Failed to run migrations");

    // Initialize job scheduler
    let scheduler = jobs::JobScheduler::new();
    scheduler.start().await;

    // Build versioned API routes
    let api_v1 = Router::new()
        .route("/users", post(handlers::v1::users::create))
        .route("/users/:id", get(handlers::v1::users::get_by_id));
    
    let api_v2 = Router::new()
        .route("/users", post(handlers::v2::users::create))
        .route("/users/:id", get(handlers::v2::users::get_by_id))
        .route("/users/:id", axum::routing::patch(handlers::v2::users::update));
    
    // Build admin routes with RBAC middleware
    let admin_routes = Router::new()
        .route("/stats", get(handlers::admin::get_stats))
        .route("/health", get(handlers::admin::health_check))
        .route("/protected", get(handlers::admin::get_stats))
        .layer(axum::middleware::from_fn(rbac::admin_middleware));
    
    let app = Router::new()
        .route("/", get(handlers::root::handler))
        .route("/healthz", get(handlers::health::handler))
        .route("/api/auth/login", post(handlers::auth::login))
        .nest("/api/v1", api_v1)
        .nest("/api/v2", api_v2)
        .nest("/admin", admin_routes)
        .nest_service("/static", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]))
        .with_state(config);

    // Run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    
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
    
    tracing::info!("signal received, starting graceful shutdown");
}