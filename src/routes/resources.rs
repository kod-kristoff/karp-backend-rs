use axum::extract::{Extension, Json};
use http::StatusCode;
use sqlx::MySqlPool;

#[derive(serde::Deserialize)]
pub struct ResourceCreate {
    resource_id: String,
}

pub async fn create_resource(
    Json(data): Json<ResourceCreate>,
    Extension(db_pool): Extension<MySqlPool>,
) -> StatusCode {
    StatusCode::OK
}
