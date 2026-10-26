// backend/src/api/health.rs

//! Health endpoints used by developers, monitoring and deployment platforms.

use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;

use crate::AppState;

/// JSON body returned by the health endpoints.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

/// `GET /health/live`: "is the process running?"
///
/// Deliberately does NOT touch the database: if the database is down,
/// the process is still alive.
pub async fn live() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

/// `GET /health/ready`: "can this instance do real work right now?"
///
/// Runs a trivial query. 200 if the database answers, 503 if it does not.
/// The error detail goes to the server log, never to the HTTP response.
pub async fn ready(State(state): State<AppState>) -> (StatusCode, Json<HealthResponse>) {
    match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => (StatusCode::OK, Json(HealthResponse { status: "ok" })),
        Err(err) => {
            tracing::warn!(error = %err, "readiness check failed: database unavailable");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(HealthResponse {
                    status: "unavailable",
                }),
            )
        }
    }
}
