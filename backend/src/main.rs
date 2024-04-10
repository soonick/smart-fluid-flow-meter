use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize)]
struct SaveMeasureInput {
    device_id: String,
    measure: String,
    #[serde(serialize_with = "serialize_dt")]
    recorded_at: DateTime<Local>,
}

#[derive(Serialize, Clone)]
struct Measure {
    id: String,
    device_id: String,
    measure: String,
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

async fn save_measure(
    AppJson(input): AppJson<SaveMeasureInput>,
) -> Result<AppJson<Measure>, AppError> {
    Ok(AppJson(Measure {
        id: "hello".to_string(),
        device_id: input.device_id,
        measure: input.measure,
        recorded_at: input.recorded_at,
    }))
}

// Our own JSON extractor that wraps `axum::Json`. This allows us to format errors
// whichever way we want
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
struct JsonExtractor<T>(T);

impl<T> IntoResponse for JsonExtractor<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

// The kinds of errors we can hit in our application.
enum AppError {
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
}

// Tell axum how `AppError` should be converted into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            code: String,
            message: String,
        }

        let (response_status, error_code, error_message) = match self {
            // This error is caused by bad user input
            AppError::JsonRejection(rejection) => (
                rejection.status(),
                "INVALID_INPUT".to_string(),
                "Invalid input".to_string(),
            ),
        };

        (
            response_status,
            AppJson(ErrorResponse {
                code: error_code,
                message: error_message,
            }),
        )
            .into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

// Our own JSON extractor that wraps `axum::Json`. This way we can override
// json rejections and provide error in whichever format we want
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}
