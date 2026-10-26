// backend/src/api/health.rs

//! Health endpoints used by developers, monitoring and deployment platforms.

use axum::Json;
use serde::Serialize;

/// JSON body returned by the health endpoints.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

/// `GET /health/live` — "is the process running?"
///
/// Deliberately does NOT touch the database: if the database is down,
/// the process is still alive. Readiness (with a DB check) comes next.
pub async fn live() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
