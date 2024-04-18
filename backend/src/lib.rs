pub mod api;
mod error;
mod handler;
mod json;
pub mod settings;
pub mod storage;

use crate::handler::measure::save_measure;
use crate::storage::Storage;

use axum::{extract::FromRef, routing::post, Router};
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
        .route("/measure", post(save_measure))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
