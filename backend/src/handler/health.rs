use crate::api::health::Health;
use crate::error::app_error::AppError;
use crate::json::extractor::Extractor;

pub async fn health_check() -> Result<Extractor<Health>, AppError> {
    Ok(Extractor(Health {}))
}
