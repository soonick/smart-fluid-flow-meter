use config::{Config, Environment, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Service {
    pub port: u32,
}

#[derive(Debug, Deserialize)]
pub struct Firestore {
    pub project_id: String,
    pub database_id: String,
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
pub struct Captcha {
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub captcha: Captcha,
    pub database: Database,
    pub service: Service,
}

impl Settings {
    pub fn new() -> Self {
        let s = match Config::builder()
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

    // Creates a configuration from a given file. Used for testing
    pub fn from_file(config_path: &str) -> Self {
        let s = match Config::builder()
            .add_source(File::new(config_path, FileFormat::Yaml))
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
