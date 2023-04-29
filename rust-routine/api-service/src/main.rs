use axum::{Extension, Router};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;

mod common;
mod config;
mod dao;
mod global;
mod init;
mod model;
mod service;

#[tokio::main]
async fn main() {
    init::init_log();
    let s = &global::CONFIGS.server;
    tracing::info!("Server started on {}:{}", s.host, s.port);

    axum::Server::bind(&(format!("{}:{}", s.host, s.port).parse().unwrap()))
        .serve(init_layer(init::init_router()).await.into_make_service())
        .await
        .unwrap();
}

pub async fn init_layer(router: Router) -> Router {
    router.layer(
        ServiceBuilder::new()
            .layer(CookieManagerLayer::new())
            .layer(Extension(Arc::new(
                init::init_database()
                    .await
                    .expect("Database connection failed"),
            ))),
    )
}
