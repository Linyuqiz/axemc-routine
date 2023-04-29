use std::sync::Arc;

use crate::{common, model::articles};
use axum::{response::IntoResponse, Extension};
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn list(
    Extension(ref connection): Extension<Arc<DatabaseConnection>>,
) -> impl IntoResponse {
    let res = articles::Entity::find()
        .all(connection.as_ref())
        .await
        .expect("test failed");

    common::ResResponse::success("test success".to_string(), Some(res))
}
