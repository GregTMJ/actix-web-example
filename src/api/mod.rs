use std::{env::current_dir, fs::exists, path::PathBuf};
use tokio::fs;

use actix_web::{Error, HttpResponse, error::ErrorNotFound, http::StatusCode};
use sqlx::{Pool, Postgres};

use crate::{
    api::schemas::HashGenerate,
    database::fetch_mock_response,
    helpers::errors::{ApiError, StatusCode as CustomStatusCode},
};

pub mod equifax_block;
pub mod equifax_converter;
pub mod equifax_fps;
pub mod equifax_payment;
pub mod equifax_scoring;
pub mod equifax_v4;
pub mod fincard;
pub mod fincert;
pub mod infosfera;
pub mod mock_response;
pub mod schemas;
pub mod vk;

pub fn get_file_path(file_name: &str) -> Result<PathBuf, Error> {
    let original_path = current_dir().unwrap().join("static").join(file_name);
    match exists(&original_path) {
        Ok(_) => Ok(original_path),
        Err(msg) => Err(ErrorNotFound(msg)),
    }
}

pub async fn mock_fetch_file_response(
    conn: &Pool<Postgres>,
    data: impl HashGenerate,
    service_id: i32,
    file_path: PathBuf,
) -> Result<String, HttpResponse> {
    let result = fetch_mock_response(conn, data, service_id).await?;
    if !result.is_empty() {
        return Ok(result);
    }
    match fs::read_to_string(file_path).await {
        Ok(content) => Ok(content),
        Err(err) => Err(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .json(ApiError::new(
                err.to_string(),
                CustomStatusCode::ErrorConflict,
            ))),
    }
}
