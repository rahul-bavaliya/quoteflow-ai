// backend/src/lib.rs

//! QuoteFlow AI backend library.
//!
//! `main.rs` only starts the server. Everything else lives here so that
//! integration tests can build the exact same application without a network.

pub mod api;

use axum::Router;

/// Builds the complete HTTP application (all routes).
pub fn build_app() -> Router {
    api::router()
}
