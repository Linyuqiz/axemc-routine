use axum::response::Html;

use sea_orm::{DatabaseConnection, EntityTrait};

use std::sync::Arc;

use axum::{response::IntoResponse, Extension};

pub async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub async fn user_list(
    Extension(ref conn): Extension<Arc<DatabaseConnection>>,
) -> impl IntoResponse {
    let res = crate::model::user_info::Entity::find()
        .all(conn.as_ref())
        .await
        .expect("test failed");

    crate::common::ResResponse::success("test success".to_string(), Some(res))
}
