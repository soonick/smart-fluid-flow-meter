use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Service {
    pub port: u32,
}

#[derive(Debug, Deserialize)]
pub struct Firestore {
    pub project_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Mysql {
    pub connection_string: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub firestore: Firestore,
    pub mysql: Mysql,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub service: Service,
    pub database: Database,
}

impl Settings {
    pub fn new() -> Self {
        let s = match Config::builder()
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1` would set the `debug` key
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()
        {
            Ok(s) => s,
            Err(err) => panic!("Couldn't build configuration. Error: {}", err),
        };

        match s.try_deserialize() {
            Ok(s) => s,
            Err(err) => panic!("Couldn't deserialize configuration. Error: {}", err),
        }
    }
}
