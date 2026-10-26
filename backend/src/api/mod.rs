// backend/src/api/mod.rs

//! API layer: maps URLs to handler functions.

mod health;

use axum::{Router, routing::get};

use crate::AppState;

/// All HTTP routes of the application.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health/live", get(health::live))
        .route("/health/ready", get(health::ready))
        .with_state(state)
}
