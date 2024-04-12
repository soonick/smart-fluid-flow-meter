use crate::api::measure::Measure;
use crate::storage::{error::Error, error::ErrorCode, Storage};

use async_trait::async_trait;
use firestore::FirestoreDb;

const MEASURE_COLLECTION: &'static str = "measure";

#[derive(Clone)]
pub struct FirestoreStorage {
    db: FirestoreDb,
}

impl FirestoreStorage {
    pub async fn new(project_id: &str) -> FirestoreStorage {
        let db = match FirestoreDb::new(project_id).await {
            Ok(db) => db,
            Err(_err) => panic!("Unable create firestore db for project: {}", project_id),
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
            Err(_err) => {
                return Err(Error {
                    code: ErrorCode::UndefinedError,
                })
            }
        };

        Ok(Measure {
            id: inserted.id,
            ..measure
        })
    }
}
