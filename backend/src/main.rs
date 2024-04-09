use axum::{routing::post, Json, Router};
use chrono::{DateTime, Local};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
struct SaveMeasureInput {
    measure: String,
    #[serde(serialize_with = "serialize_dt")]
    recorded_at: DateTime<Local>,
}

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

async fn save_measure(Json(input): Json<SaveMeasureInput>) -> &'static str {
    println!("The value is {} the time: {}", input.measure, input.recorded_at);
    "{}"
}
