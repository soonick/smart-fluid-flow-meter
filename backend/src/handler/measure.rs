use crate::api::measure::{Measure, SaveMeasureInput};
use crate::error::app_error::AppError;
use crate::json::extractor::Extractor;
use crate::storage::{mysql, Storage};

pub async fn save_measure(
    Extractor(input): Extractor<SaveMeasureInput>,
) -> Result<Extractor<Measure>, AppError> {
    let storage = mysql::from_connection_string("mysql://root:password@localhost:3307/test").await;
    let measure = Measure {
        id: "".to_string(),
        device_id: input.device_id,
        measure: input.measure,
        recorded_at: input.recorded_at,
    };
    let inserted = storage.save_measure(measure).await?;

    Ok(Extractor(Measure {
        id: inserted.id,
        device_id: inserted.device_id,
        measure: inserted.measure,
        recorded_at: inserted.recorded_at,
    }))
}
