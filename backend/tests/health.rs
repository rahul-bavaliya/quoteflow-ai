// backend/tests/health.rs

//! Integration tests for the health endpoints.
//! They call the app directly in memory: no real network port and no real
//! database needed.

use std::time::Duration;

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt; // gives us `.collect()` to read a response body
use tower::ServiceExt; // gives us `.oneshot()` to send one request to the app

use quoteflow::{AppState, db};

/// Builds the app with a "lazy" pool pointing at a closed port.
///
/// `connect_lazy` does not connect until a query runs, so routes that never
/// touch the database work normally, and routes that do will fail fast.
fn app_without_database() -> Router {
    let pool = db::pool_options()
        .acquire_timeout(Duration::from_millis(500))
        .connect_lazy("postgres://test:test@127.0.0.1:1/unreachable")
        .expect("test database URL should be valid");

    quoteflow::build_app(AppState { db: pool })
}

async fn json_body(response: axum::response::Response) -> serde_json::Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn live_returns_200_even_without_database() {
    let response = app_without_database()
        .oneshot(Request::get("/health/live").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        json_body(response).await,
        serde_json::json!({ "status": "ok" })
    );
}

#[tokio::test]
async fn ready_returns_503_when_database_is_unreachable() {
    let response = app_without_database()
        .oneshot(Request::get("/health/ready").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        json_body(response).await,
        serde_json::json!({ "status": "unavailable" })
    );
}

#[tokio::test]
async fn unknown_route_returns_404() {
    let response = app_without_database()
        .oneshot(Request::get("/does-not-exist").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
