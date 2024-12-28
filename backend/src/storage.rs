pub mod error;
pub mod firestore;
pub mod memory;
pub mod mysql;

use crate::api::measure::Measure;

use async_trait::async_trait;
use chrono::{DateTime, Local};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_measure(&self, measure: Measure) -> Result<Measure, error::Error>;
    async fn get_measurements(
        &self,
        device_id: String,
        since: DateTime<Local>,
        num_records: u32,
    ) -> Result<Vec<Measure>, error::Error>;
}
