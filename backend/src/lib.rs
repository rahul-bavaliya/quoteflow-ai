// backend/src/lib.rs

//! QuoteFlow AI backend library.
//!
//! `main.rs` only starts the server. Everything else lives here so that
//! integration tests can build the exact same application without a network.

pub mod api;
pub mod config;
pub mod db;

use axum::Router;
use sqlx::PgPool;

/// Shared resources available to every request handler.
///
/// Cloning is cheap: `PgPool` is a handle to one shared set of connections.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

/// Builds the complete HTTP application (all routes) with its shared state.
pub fn build_app(state: AppState) -> Router {
    api::router(state)
}
