use axum::{routing::get, Router};
use std::{future::Future, time::Duration};

use crate::global;
use crate::service;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::log;

pub fn init_router() -> Router {
    Router::new()
        .route("/", get(service::handler))
        .route("/user_info", get(service::user_list))
}

pub fn init_log() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api-service=debug, sea_orm=debug");
    }
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .with_file(true)
        .with_line_number(true)
        .init();
}

pub fn init_database() -> impl Future<Output = Result<DatabaseConnection, sea_orm::DbErr>> {
    Database::connect(
        ConnectOptions::new(global::CONFIGS.database.url.to_owned())
            .max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Debug)
            .to_owned(),
    )
}
