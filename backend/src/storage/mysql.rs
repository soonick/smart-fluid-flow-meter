use crate::api::measure::Measure;
use crate::storage::{error::Error, error::ErrorCode, Storage};

use sqlx::mysql::{MySql, MySqlPoolOptions};
use sqlx::Pool;

#[derive(Clone)]
pub struct MySqlStorage {
    pool: Pool<MySql>,
}

impl MySqlStorage {
    pub async fn new(connection_string: &str) -> MySqlStorage {
        let pool = match MySqlPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await
        {
            Ok(pool) => pool,
            Err(_err) => panic!(
                "Unable to create MySql connection pool using {}",
                connection_string
            ),
        };

        return MySqlStorage { pool };
    }
}

impl Storage for MySqlStorage {
    // The id of the passed measure is ignored. An id will be assigned automtically
    async fn save_measure(&self, measure: Measure) -> Result<Measure, Error> {
        let inserted = match sqlx::query(
            "INSERT INTO measurements(device_id, measure, recorded_at) VALUES(?, ?, ?)",
        )
        .bind(measure.device_id.clone())
        .bind(measure.measure.clone())
        .bind(measure.recorded_at.to_rfc3339())
        .execute(&self.pool)
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
            id: inserted.last_insert_id().to_string(),
            ..measure
        })
    }
}
