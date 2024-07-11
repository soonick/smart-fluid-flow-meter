use crate::api::measure::Measure;
use crate::storage::{error::Error, error::ErrorCode, Storage};

use async_trait::async_trait;
use sqlx::mysql::{MySql, MySqlPoolOptions};
use sqlx::Pool;
use tracing::error;
use tracing::info;

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
            Err(err) => panic!(
                "Unable to create MySql connection pool using {}. Error: {}",
                connection_string, err
            ),
        };

        match sqlx::migrate!("assets/db-migrations").run(&pool).await {
            Ok(()) => info!("DB migrations ran successfully"),
            Err(err) => panic!("Unable to run MySql migrations. Error: {}", err),
        };

        return MySqlStorage { pool };
    }
}

#[async_trait]
impl Storage for MySqlStorage {
    // The id of the passed measure is ignored. An id will be assigned automatically
    async fn save_measure(&self, measure: Measure) -> Result<Measure, Error> {
        let inserted = match sqlx::query(
            "INSERT INTO measurement(device_id, measure, recorded_at) VALUES(?, ?, ?)",
        )
        .bind(measure.device_id.clone())
        .bind(measure.measure.clone())
        .bind(measure.recorded_at.to_rfc3339())
        .execute(&self.pool)
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
            id: inserted.last_insert_id().to_string(),
            ..measure
        })
    }
}
