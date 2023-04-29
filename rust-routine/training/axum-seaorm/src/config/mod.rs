use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app: App,
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct App {
    pub name: String,
    pub version: String,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

static GLOBAL_CONFIG: Lazy<Config> = Lazy::new(|| init_config());

fn init_config() -> Config {
    let config_yml = include_str!("./config.yml");
    serde_yaml::from_str(config_yml).expect("config.yaml read failed!")
}

pub fn get_app() -> &'static App {
    &GLOBAL_CONFIG.app
}

pub fn get_database() -> &'static Database {
    &GLOBAL_CONFIG.database
}
