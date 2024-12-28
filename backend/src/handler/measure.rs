use crate::api::measure::{Measure, SaveMeasureInput};
use crate::error::app_error::AppError;
use crate::json::extractor::Extractor;
use crate::AppState;

use axum::extract::State;
use chrono::Local;
use tracing::info;

const ONE_MINUTE: i64 = 60;

pub async fn save_measure(
    State(state): State<AppState>,
    Extractor(input): Extractor<SaveMeasureInput>,
) -> Result<Extractor<Measure>, AppError> {
    // If the same measure has been recorded recently, drop this one
    let now = Local::now();
    let res = &state
        .storage
        .get_measurements(input.device_id.clone(), now, 1)
        .await?;
    if res.len() > 0 {
        if res[0].measure == input.measure {
            info!(
                "The same measure was last found {} seconds ago",
                now.timestamp() - res[0].recorded_at.timestamp()
            );
            if now.timestamp() - res[0].recorded_at.timestamp() < ONE_MINUTE {
                return Ok(Extractor(res[0].clone()));
            }
        } else {
            info!(
                "The last measure was {} at {}",
                res[0].measure,
                res[0].recorded_at.to_rfc3339()
            );
        }
    }

    let measure = Measure {
        id: None,
        device_id: input.device_id,
        measure: input.measure,
        recorded_at: now,
    };
    let inserted = state.storage.save_measure(measure).await?;

    Ok(Extractor(Measure {
        id: inserted.id,
        device_id: inserted.device_id,
        measure: inserted.measure,
        recorded_at: inserted.recorded_at,
    }))
}
