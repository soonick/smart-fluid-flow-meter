pub mod error;
pub mod firestore;
pub mod memory;
pub mod mysql;

use crate::api::measurement::Measurement;

use async_trait::async_trait;
use chrono::{DateTime, Local};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_measurement(&self, measurement: Measurement)
        -> Result<Measurement, error::Error>;
    async fn get_measurements(
        &self,
        device_id: String,
        since: DateTime<Local>,
        num_records: u32,
    ) -> Result<Vec<Measurement>, error::Error>;
}
