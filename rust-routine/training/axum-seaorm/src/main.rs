use std::sync::Arc;

use axum::{Extension, Router};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;

mod common;
mod config;
mod dao;
mod db;
mod model;
mod router;

#[tokio::main]
async fn main() {
    init_log();

    let address = config::get_app().address.parse().unwrap();
    tracing::info!("Starting server on {}", &address);

    axum::Server::bind(&address)
        .serve(init_layer(router::init_router()).await.into_make_service())
        .await
        .unwrap();
}

fn init_log() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "contract-backend=debug, sea_orm=debug");
    }
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_file(true)
        .with_line_number(true)
        .init();
}

async fn init_layer(router: Router) -> Router {
    router.layer(
        ServiceBuilder::new()
            .layer(CookieManagerLayer::new())
            .layer(Extension(Arc::new(
                db::init_database()
                    .await
                    .expect("Database connection failed"),
            ))),
    )
}
