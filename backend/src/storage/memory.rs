//! Memory storage for easy testing.
//!
//! NOTE: Not a good idea for production since storage is lost when the server
//! restarts

use crate::api::measure::Measure;
use crate::storage::{
    error::{
        Error,
        ErrorCode,
    },
    Storage
};

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct MemoryStorage {
    measure: Arc<Mutex<HashMap<String, Measure>>>,
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
    // Uses recorded_at as id
    async fn save_measure(&self, measure: Measure) -> Result<Measure, Error> {
        let new_measure = Measure {
            id: measure.recorded_at.to_string(),
            ..measure
        };

        {
            let m = Arc::clone(&self.measure);
            // Fail if there is already a record with that ID
            {
                let measures = m.lock().unwrap();
                if measures.contains_key(&new_measure.id) {
                    return Err(Error {
                        code: ErrorCode::UndefinedError,
                    })
                }
            }

            {
                let mut measures = m.lock().unwrap();
                measures.insert(new_measure.id.clone(), new_measure.clone());
            }
        }

        Ok(new_measure)
    }
}
