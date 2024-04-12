pub mod api;
mod error;
mod handler;
mod json;
pub mod storage;

use crate::handler::measure::save_measure;
use crate::storage::Storage;

use axum::{extract::FromRef, routing::post, Router};
use std::sync::Arc;

#[derive(Clone, FromRef)]
struct AppState {
    storage: Arc<dyn Storage>,
}

pub async fn app(storage: Arc<dyn Storage>) -> Router {
    let state = AppState { storage };
    Router::new()
        .route("/measure", post(save_measure))
        .with_state(state)
}
