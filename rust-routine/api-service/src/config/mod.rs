use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Configs {
    pub app: App,
    pub server: Server,
    pub database: Database,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct App {
    pub name: String,
    pub version: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub url: String,
}

impl Configs {
    pub fn init_config() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name("./dev.yaml"))
            .build()
            .unwrap()
            .try_deserialize()
    }
}
