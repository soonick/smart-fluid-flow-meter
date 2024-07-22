use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct SaveMeasureInput {
    pub device_id: String,
    pub measure: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Measure {
    #[serde(alias = "_firestore_id")]
    pub id: Option<String>,
    pub device_id: String,
    pub measure: String,
    pub recorded_at: DateTime<Local>,
}
