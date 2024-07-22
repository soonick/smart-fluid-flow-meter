use crate::api::measure::{Measure, SaveMeasureInput};
use crate::error::app_error::AppError;
use crate::json::extractor::Extractor;
use crate::AppState;

use axum::extract::State;
use chrono::Local;

pub async fn save_measure(
    State(state): State<AppState>,
    Extractor(input): Extractor<SaveMeasureInput>,
) -> Result<Extractor<Measure>, AppError> {
    let measure = Measure {
        id: None,
        device_id: input.device_id,
        measure: input.measure,
        recorded_at: Local::now(),
    };
    let inserted = state.storage.save_measure(measure).await?;

    Ok(Extractor(Measure {
        id: inserted.id,
        device_id: inserted.device_id,
        measure: inserted.measure,
        recorded_at: inserted.recorded_at,
    }))
}
