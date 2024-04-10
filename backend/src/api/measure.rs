use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SaveMeasureInput {
    pub device_id: String,
    pub measure: String,
    #[serde(serialize_with = "serialize_dt")]
    pub recorded_at: DateTime<Local>,
}

#[derive(Serialize, Clone)]
pub struct Measure {
    pub id: String,
    pub device_id: String,
    pub measure: String,
    pub recorded_at: DateTime<Local>,
}
