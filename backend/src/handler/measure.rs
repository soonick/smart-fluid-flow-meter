use crate::api::measure::{Measure, SaveMeasureInput};
use crate::error::app_error::AppError;
use crate::json::extractor::Extractor;
use crate::storage::Storage;
use crate::AppState;

use axum::extract::State;

pub async fn save_measure<T>(
    State(state): State<AppState<T>>,
    Extractor(input): Extractor<SaveMeasureInput>,
) -> Result<Extractor<Measure>, AppError>
where
    T: Storage,
{
    let measure = Measure {
        id: "".to_string(),
        device_id: input.device_id,
        measure: input.measure,
        recorded_at: input.recorded_at,
    };
    let inserted = state.storage.save_measure(measure).await?;

    Ok(Extractor(Measure {
        id: inserted.id,
        device_id: inserted.device_id,
        measure: inserted.measure,
        recorded_at: inserted.recorded_at,
    }))
}
