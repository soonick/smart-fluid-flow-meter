//! Memory storage for easy testing.
//!
//! NOTE: Not a good idea for production since storage is lost when the server
//! restarts

use crate::api::measurement::Measurement;
use crate::api::user::User;
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
    measurement: Arc<Mutex<HashMap<String, Vec<Measurement>>>>,
    user: Arc<Mutex<HashMap<String, User>>>,
}

impl MemoryStorage {
    pub async fn new() -> MemoryStorage {
        return MemoryStorage {
            measurement: Arc::new(Mutex::new(HashMap::new())),
            user: Arc::new(Mutex::new(HashMap::new())),
        };
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    async fn save_measurement(&self, measurement: Measurement) -> Result<Measurement, Error> {
        if measurement.device_id == "" {
            return Err(Error {
                code: ErrorCode::UndefinedError,
            });
        }

        let new_measurement = Measurement {
            id: Some(measurement.device_id.clone()),
            ..measurement
        };

        {
            let m = Arc::clone(&self.measurement);
            {
                let mut measurements = m.lock().unwrap();
                let id = new_measurement.id.clone().unwrap();
                if measurements.contains_key(&id) {
                    let device_measurements = measurements.get_mut(&id).unwrap();
                    device_measurements.push(new_measurement.clone());
                } else {
                    let v = vec![new_measurement.clone()];
                    measurements.insert(new_measurement.id.clone().unwrap(), v);
                }
            }
        }

        Ok(new_measurement)
    }

    async fn get_measurements(
        &self,
        device_id: String,
        since: DateTime<Local>,
        num_records: u32,
    ) -> Result<Vec<Measurement>, Error> {
        let mut found = vec![];
        let mut i = 0;
        {
            let m = Arc::clone(&self.measurement);
            {
                let measurements = m.lock().unwrap();
                let device_measurements = measurements.get(&device_id);
                if device_measurements.is_some() {
                    for m in device_measurements.unwrap().iter().rev() {
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

    async fn sign_up_user(&self, user: User) -> Result<User, Error> {
        {
            let key = format!("{}+{}", user.id, user.provider);
            let u = Arc::clone(&self.user);
            {
                let users = u.lock().unwrap();
                if users.contains_key(&key) {
                    return Err(Error {
                        code: ErrorCode::UndefinedError,
                    });
                }
            }

            {
                let mut users = u.lock().unwrap();
                users.insert(key, user.clone());
            }
        }

        Ok(user)
    }
}
