// backend/src/main.rs

//! Entry point: set up logging, load configuration, connect to the database,
//! open a network port, serve the app.

use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use quoteflow::{AppState, config::Config, db};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logging. Level comes from RUST_LOG (e.g. RUST_LOG=debug), default "info".
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Configuration from environment variables. Loaded after logging so a
    // bad value is reported clearly before exiting.
    let config = Config::from_env().unwrap_or_else(|err| {
        tracing::error!("configuration error: {err}");
        std::process::exit(1);
    });

    // Database. Fail fast at startup if it cannot be reached.
    let db = db::connect(&config.database_url)
        .await
        .unwrap_or_else(|err| {
            tracing::error!("could not connect to the database: {err}");
            std::process::exit(1);
        });
    tracing::info!("connected to the database");

    let addr = config.socket_addr();
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("QuoteFlow API listening on http://{addr}");

    let app = quoteflow::build_app(AppState { db });

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("server stopped");
    Ok(())
}

/// Resolves when the user presses Ctrl+C, so the server can finish
/// in-flight requests before exiting ("graceful shutdown").
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for Ctrl+C");
    tracing::info!("shutdown signal received");
}
