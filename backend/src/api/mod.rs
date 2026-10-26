// backend/src/api/mod.rs

//! API layer: maps URLs to handler functions.

mod health;

use axum::{Router, routing::get};

/// All HTTP routes of the application.
pub fn router() -> Router {
    Router::new().route("/health/live", get(health::live))
}
