use axum::{
    body::Body,
    http,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use smart_fluid_flow_meter_backend::storage::memory::MemoryStorage;
use std::sync::Arc;
use tower::util::ServiceExt;

#[tokio::test]
async fn save_measure_invalid_json() {
    let storage = Arc::new(MemoryStorage::new().await);
    let app = smart_fluid_flow_meter_backend::app(storage).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/measure")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        body,
        json!({ "code": "InvalidInput", "message": "Invalid input" })
    );
}
