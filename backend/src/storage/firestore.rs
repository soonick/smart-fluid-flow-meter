use crate::api::measure::Measure;
use crate::storage::{error::Error, error::ErrorCode, Storage};

use async_trait::async_trait;
use firestore::FirestoreDb;
use firestore::FirestoreDbOptions;
use tracing::error;

const MEASURE_COLLECTION: &'static str = "measure";

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
    // The id of the passed measure is ignored. An id will be assigned automtically
    async fn save_measure(&self, measure: Measure) -> Result<Measure, Error> {
        let inserted: Measure = match self
            .db
            .fluent()
            .insert()
            .into(MEASURE_COLLECTION)
            .generate_document_id()
            .object(&measure)
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

        Ok(Measure {
            id: inserted.id,
            ..measure
        })
    }
}
