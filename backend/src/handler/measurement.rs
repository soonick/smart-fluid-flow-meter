use crate::api::measurement::{Measurement, SaveMeasurementInput};
use crate::error::app_error::AppError;
use crate::json::extractor::Extractor;
use crate::AppState;

use axum::extract::State;
use chrono::Local;
use tracing::info;

const ONE_MINUTE: i64 = 60;

pub async fn save_measurement(
    State(state): State<AppState>,
    Extractor(input): Extractor<SaveMeasurementInput>,
) -> Result<Extractor<Measurement>, AppError> {
    // If the same measurement has been recorded recently, drop this one
    let now = Local::now();
    let res = &state
        .storage
        .get_measurements(input.device_id.clone(), now, 1)
        .await?;
    if res.len() > 0 {
        if res[0].measurement == input.measurement {
            info!(
                "The same measurement was last found {} seconds ago",
                now.timestamp() - res[0].recorded_at.timestamp()
            );
            if now.timestamp() - res[0].recorded_at.timestamp() < ONE_MINUTE {
                return Ok(Extractor(res[0].clone()));
            }
        } else {
            info!(
                "The last measurement was {} at {}",
                res[0].measurement,
                res[0].recorded_at.to_rfc3339()
            );
        }
    }

    let measurement = Measurement {
        id: None,
        device_id: input.device_id,
        measurement: input.measurement,
        recorded_at: now,
    };
    let inserted = state.storage.save_measurement(measurement).await?;

    Ok(Extractor(Measurement {
        id: inserted.id,
        device_id: inserted.device_id,
        measurement: inserted.measurement,
        recorded_at: inserted.recorded_at,
    }))
}
