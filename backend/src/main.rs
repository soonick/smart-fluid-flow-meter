use smart_fluid_flow_meter_backend::storage::firestore::FirestoreStorage;

use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_thread_names(true)
        .with_line_number(true)
        .init();

    let storage = Arc::new(FirestoreStorage::new("something").await);
    let app = smart_fluid_flow_meter_backend::app(storage).await;

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
