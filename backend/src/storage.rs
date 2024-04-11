pub mod error;
pub mod firestore;
pub mod mysql;

use crate::api::measure::Measure;

pub trait Storage {
    async fn save_measure(&self, measure: Measure) -> Result<Measure, error::Error>;
}
