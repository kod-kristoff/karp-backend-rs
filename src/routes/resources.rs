use axum::extract::{Extension, Json};
use http::StatusCode;
use sqlx::{self, types::Json as SqlJson, PgPool};

use uuid::Uuid;
use ulid::Ulid;

#[derive(serde::Deserialize)]
pub struct ResourceCreate {
    resource_id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ResourceConfig {
    pub id: String
}

pub async fn create_resource(
    Json(data): Json<ResourceCreate>,
    Extension(db_pool): Extension<PgPool>,
) -> StatusCode {
	let entity_id: Uuid = Ulid::new().into();
	let entry_repo_id: Uuid = Ulid::new().into();
	let version: u32 = 1;
	let name = String::from(&data.resource_id);
	let config = SqlJson(ResourceConfig { id: "field".into() });
	let now = chrono::Utc::now();
	let query = "INSERT INTO resources
		(entity_id, resource_id, resource_type, entry_repo_id, version, name, config, last_modified, last_modified_by, message, discarded)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)";
	sqlx::query(query)
		.bind(entity_id)
		.bind(data.resource_id)
		.bind("resource")
		.bind(entry_repo_id)
		.bind(version)
		.bind(name)
		.bind(config)
		.bind(now)
		.bind("unknown")
		.bind("added")
		.bind(true)
		.execute(&db_pool)
		.await
		.expect("Failed to insert resource.");
    StatusCode::CREATED
}
