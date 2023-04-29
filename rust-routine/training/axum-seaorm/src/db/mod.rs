use std::{future::Future, time::Duration};

use crate::config;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::log;

pub fn init_database() -> impl Future<Output = Result<DatabaseConnection, sea_orm::DbErr>> {
    let database = ConnectOptions::new(config::get_database().url.to_owned())
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug)
        .to_owned();

    Database::connect(database)
}
