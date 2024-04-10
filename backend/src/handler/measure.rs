use crate::api::measure::{Measure, SaveMeasureInput};
use crate::error::app_error::AppError;
use crate::json::extractor::Extractor;

pub async fn save_measure(
    Extractor(input): Extractor<SaveMeasureInput>,
) -> Result<Extractor<Measure>, AppError> {
    Ok(Extractor(Measure {
        id: "hello".to_string(),
        device_id: input.device_id,
        measure: input.measure,
        recorded_at: input.recorded_at,
    }))
}
