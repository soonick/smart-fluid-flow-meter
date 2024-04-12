pub mod error;
pub mod firestore;
pub mod mysql;

use crate::api::measure::Measure;

use async_trait::async_trait;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_measure(&self, measure: Measure) -> Result<Measure, error::Error>;
}
