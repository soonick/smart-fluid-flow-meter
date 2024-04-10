mod api;
mod error;
mod handler;
mod json;
mod storage;

use crate::handler::measure::save_measure;

use axum::{routing::post, Router};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_thread_names(true)
        .with_line_number(true)
        .init();

    let app = Router::new().route("/measure", post(save_measure));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
