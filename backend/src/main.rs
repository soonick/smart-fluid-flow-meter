mod api;
mod error;
mod handler;
mod json;
mod storage;

use crate::handler::measure::save_measure;
use crate::storage::{firestore::FirestoreStorage, Storage};

use axum::{extract::FromRef, routing::post, Router};
use std::sync::Arc;
use tracing::info;

#[derive(Clone, FromRef)]
struct AppState {
    storage: Arc<dyn Storage>,
}

async fn app(storage: Arc<dyn Storage>) -> Router {
    let state = AppState { storage };
    Router::new()
        .route("/measure", post(save_measure))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_thread_names(true)
        .with_line_number(true)
        .init();

    let storage =
        Arc::new(FirestoreStorage::new("something").await);
    let app = app(storage).await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
