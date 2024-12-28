use crate::api::measurement::Measurement;
use crate::storage::{error::Error, error::ErrorCode, Storage};

use async_trait::async_trait;
use chrono::{DateTime, Local};
use firestore::{path, FirestoreDb, FirestoreDbOptions, FirestoreQueryDirection};
use tracing::error;

const MEASUREMENT_COLLECTION: &'static str = "measurement";

#[derive(Clone)]
pub struct FirestoreStorage {
    db: FirestoreDb,
}

impl FirestoreStorage {
    pub async fn new(project_id: &str, database_id: &str) -> FirestoreStorage {
        let db = match FirestoreDb::with_options(
            FirestoreDbOptions::new(project_id.to_string())
                .with_database_id(database_id.to_string()),
        )
        .await
        {
            Ok(db) => db,
            Err(err) => panic!(
                "Unable create firestore db for project: {}. Error: {}",
                project_id, err
            ),
        };

        return FirestoreStorage { db };
    }
}

#[async_trait]
impl Storage for FirestoreStorage {
    // The id of the passed measurement is ignored. An id will be assigned automtically
    // We only allow saving at most on
    async fn save_measurement(&self, measurement: Measurement) -> Result<Measurement, Error> {
        let inserted: Measurement = match self
            .db
            .fluent()
            .insert()
            .into(MEASUREMENT_COLLECTION)
            .generate_document_id()
            .object(&measurement)
            .execute()
            .await
        {
            Ok(inserted) => inserted,
            Err(err) => {
                error!("Error: {}", err);
                return Err(Error {
                    code: ErrorCode::UndefinedError,
                });
            }
        };

        Ok(Measurement {
            id: inserted.id,
            ..measurement
        })
    }

    async fn get_measurements(
        &self,
        device_id: String,
        since: DateTime<Local>,
        num_records: u32,
    ) -> Result<Vec<Measurement>, Error> {
        match self
            .db
            .fluent()
            .select()
            .from(MEASUREMENT_COLLECTION)
            .filter(|q| {
                q.for_all([
                    q.field(path!(Measurement::device_id)).eq(device_id.clone()),
                    q.field(path!(Measurement::recorded_at))
                        .less_than_or_equal(since),
                ])
            })
            .order_by([(
                path!(Measurement::recorded_at),
                FirestoreQueryDirection::Descending,
            )])
            .limit(num_records)
            .obj()
            .query()
            .await
        {
            Ok(found) => {
                return Ok(found);
            }
            Err(err) => {
                error!("Error: {}", err);
                return Err(Error {
                    code: ErrorCode::UndefinedError,
                });
            }
        };
    }
}
