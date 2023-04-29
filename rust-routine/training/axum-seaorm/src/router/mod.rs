use axum::{response::Html, routing::get, Router};

use crate::dao;

pub fn init_router() -> Router {
    Router::new()
        .route("/", get(test_handler))
        .route("/article/list", get(dao::list))
}

async fn test_handler() -> Html<&'static str> {
    tracing::info!("test_handler: Hello, World!");
    Html("<h1>Hello, World!</h1>")
}
