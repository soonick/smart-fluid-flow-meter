//! Memory storage for easy testing.
//!
//! NOTE: Not a good idea for production since storage is lost when the server
//! restarts

use crate::api::measure::Measure;
use crate::storage::{
    error::{Error, ErrorCode},
    Storage,
};

use async_trait::async_trait;
use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct MemoryStorage {
    // The key is the device id, the value is the measurements for that device,
    // ordered by insertion date (newest meaurements come later)
    measure: Arc<Mutex<HashMap<String, Vec<Measure>>>>,
}

impl MemoryStorage {
    pub async fn new() -> MemoryStorage {
        return MemoryStorage {
            measure: Arc::new(Mutex::new(HashMap::new())),
        };
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    async fn save_measure(&self, measure: Measure) -> Result<Measure, Error> {
        if measure.device_id == "" {
            return Err(Error {
                code: ErrorCode::UndefinedError,
            });
        }

        let new_measure = Measure {
            id: Some(measure.device_id.clone()),
            ..measure
        };

        {
            let m = Arc::clone(&self.measure);
            {
                let mut measures = m.lock().unwrap();
                let id = new_measure.id.clone().unwrap();
                if measures.contains_key(&id) {
                    let device_measures = measures.get_mut(&id).unwrap();
                    device_measures.push(new_measure.clone());
                } else {
                    let v = vec![new_measure.clone()];
                    measures.insert(new_measure.id.clone().unwrap(), v);
                }
            }
        }

        Ok(new_measure)
    }

    async fn get_measurements(
        &self,
        device_id: String,
        since: DateTime<Local>,
        num_records: u32,
    ) -> Result<Vec<Measure>, Error> {
        let mut found = vec![];
        let mut i = 0;
        {
            let m = Arc::clone(&self.measure);
            {
                let measures = m.lock().unwrap();
                let device_measures = measures.get(&device_id);
                if device_measures.is_some() {
                    for m in device_measures.unwrap().iter().rev() {
                        if m.recorded_at <= since {
                            found.push(m.clone());
                            i += 1;

                            if i == num_records {
                                break;
                            }
                        }
                    }
                }

                return Ok(found);
            }
        }
    }
}
