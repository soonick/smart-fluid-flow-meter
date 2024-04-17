use axum::{
    body::Body,
    http,
    http::{Request, StatusCode},
};
use chrono::Local;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use smart_fluid_flow_meter_backend::{
    api::measure::{Measure, SaveMeasureInput},
    storage::{memory::MemoryStorage, mysql::MySqlStorage, firestore::FirestoreStorage},
};
use std::sync::Arc;
use tower::util::ServiceExt;
use test_log::test;

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

#[tokio::test]
async fn save_measure_success() {
    let storage = Arc::new(MemoryStorage::new().await);
    let app = smart_fluid_flow_meter_backend::app(storage).await;

    let input = SaveMeasureInput {
        device_id: "999".to_string(),
        measure: "134".to_string(),
        recorded_at: Local::now(),
    };
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/measure")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&input).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let expected = Measure {
        id: input.recorded_at.to_string(),
        device_id: input.device_id.to_string(),
        measure: input.measure.to_string(),
        recorded_at: input.recorded_at,
    };
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, serde_json::to_value(expected).unwrap());
}

#[tokio::test]
async fn save_measure_invalid_date() {
    let storage = Arc::new(MemoryStorage::new().await);
    let app = smart_fluid_flow_meter_backend::app(storage).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/measure")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    "{\"device_id\": \"1\", \"measure\": \"1\", \"recorded_at\":\"today\"}",
                ))
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

#[tokio::test]
async fn save_measure_database_failure() {
    let storage = Arc::new(MemoryStorage::new().await);
    let app = smart_fluid_flow_meter_backend::app(storage).await;

    let input = SaveMeasureInput {
        device_id: "999".to_string(),
        measure: "134".to_string(),
        recorded_at: Local::now(),
    };
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/measure")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&input).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Try to insert with the same id so there is a failure
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/measure")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&input).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        body,
        json!({ "code": "InternalError", "message": "We made a mistake. Sorry" })
    );
}

#[test(tokio::test)]
async fn save_measure_success_mysql() {
    let storage = Arc::new(MySqlStorage::new("mysql://user:password@mysql/smart-fluid-flow-meter-backend").await);
    let app = smart_fluid_flow_meter_backend::app(storage).await;

    let input = SaveMeasureInput {
        device_id: "999".to_string(),
        measure: "134".to_string(),
        recorded_at: Local::now(),
    };
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/measure")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&input).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let expected = Measure {
        id: "1".to_string(),
        device_id: input.device_id.to_string(),
        measure: input.measure.to_string(),
        recorded_at: input.recorded_at,
    };
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, serde_json::to_value(expected).unwrap());
}

#[test(tokio::test)]
async fn save_measure_success_firestore() {
    let storage = Arc::new(FirestoreStorage::new("dummy-id").await);
    let app = smart_fluid_flow_meter_backend::app(storage).await;

    let input = SaveMeasureInput {
        device_id: "999".to_string(),
        measure: "134".to_string(),
        recorded_at: Local::now(),
    };
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/measure")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&input).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let expected = Measure {
        id: "".to_string(),
        device_id: input.device_id.to_string(),
        measure: input.measure.to_string(),
        recorded_at: input.recorded_at,
    };
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, serde_json::to_value(expected).unwrap());
}
