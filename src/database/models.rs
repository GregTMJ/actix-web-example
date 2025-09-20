use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ServiceMockResponse {
    pub id: i32,
    pub service_id: i32,
    pub data: String,
    pub data_hash: String,
}
