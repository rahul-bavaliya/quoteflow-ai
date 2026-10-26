// backend/tests/health.rs

//! Integration tests for the health endpoints.
//! They call the app directly in memory — no real network port needed.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt; // gives us `.collect()` to read a response body
use tower::ServiceExt; // gives us `.oneshot()` to send one request to the app

#[tokio::test]
async fn live_returns_200_with_status_ok() {
    let app = quoteflow::build_app();

    let response = app
        .oneshot(Request::get("/health/live").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body, serde_json::json!({ "status": "ok" }));
}

#[tokio::test]
async fn unknown_route_returns_404() {
    let app = quoteflow::build_app();

    let response = app
        .oneshot(Request::get("/does-not-exist").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
