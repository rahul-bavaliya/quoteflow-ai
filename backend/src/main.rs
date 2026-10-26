// backend/src/main.rs

//! Entry point: set up logging, load configuration, open a network port, serve the app.

use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use quoteflow::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logging. Level comes from RUST_LOG (e.g. RUST_LOG=debug), default "info".
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Configuration from environment variables (APP_HOST, APP_PORT), with defaults.
    // Loaded after logging so a bad value is reported clearly before exiting.
    let config = Config::from_env().unwrap_or_else(|err| {
        tracing::error!("configuration error: {err}");
        std::process::exit(1);
    });
    let addr = config.socket_addr();
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("QuoteFlow API listening on http://{addr}");

    axum::serve(listener, quoteflow::build_app())
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
