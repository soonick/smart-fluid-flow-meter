mod handler;
mod json;

pub mod api;
pub mod error;
pub mod helper;
pub mod http_client;
pub mod settings;
pub mod storage;

use crate::handler::health::health_check;
use crate::handler::measurement::save_measurement;
use crate::handler::user::sign_up_user;
use crate::helper::user::UserHelper;
use crate::settings::settings::Settings;
use crate::storage::Storage;

use axum::{extract::FromRef, routing::get, routing::post, Router};
use std::sync::Arc;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[derive(Clone, FromRef)]
struct AppState {
    settings: Arc<Settings>,
    storage: Arc<dyn Storage>,
    user_helper: Arc<dyn UserHelper>,
}

pub async fn app(
    settings: Arc<Settings>,
    storage: Arc<dyn Storage>,
    user_helper: Arc<dyn UserHelper>,
) -> Router {
    let state = AppState {
        settings,
        storage,
        user_helper,
    };
    Router::new()
        .route("/health", get(health_check))
        .route("/measurement", post(save_measurement))
        .route("/sign_up", post(sign_up_user))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
