pub mod api;
mod error;
mod handler;
pub mod http_client;
mod json;
pub mod settings;
pub mod storage;

use crate::handler::health::health_check;
use crate::handler::measurement::save_measurement;
use crate::storage::Storage;

use axum::{extract::FromRef, routing::get, routing::post, Router};
use std::sync::Arc;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[derive(Clone, FromRef)]
struct AppState {
    storage: Arc<dyn Storage>,
}

pub async fn app(storage: Arc<dyn Storage>) -> Router {
    let state = AppState { storage };
    Router::new()
        .route("/health", get(health_check))
        .route("/measurement", post(save_measurement))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
