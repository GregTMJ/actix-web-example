use std::env::{self};

use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use sqlx::Error as SqlError;
use sqlx::postgres::{PgConnection, PgPoolOptions};
use sqlx::{Connection, Pool, Postgres};

use crate::api::schemas::{HashGenerate, MockResponseBody};
use crate::database::models::ServiceMockResponse;
use crate::helpers::errors::{ApiError, StatusCode as CustomStatusCode};

pub mod models;

pub async fn check_connection() -> Result<(), SqlError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set for work");
    PgConnection::connect(&database_url).await?;
    Ok(())
}

pub async fn get_connection_pool(max_connection: u32) -> Result<Pool<Postgres>, SqlError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set for work");
    PgPoolOptions::new()
        .max_connections(max_connection)
        .connect(&database_url)
        .await
}

pub async fn fetch_mock_response(
    connection: &Pool<Postgres>,
    input_data: impl HashGenerate,
    service_id: i32,
) -> Result<String, HttpResponse> {
    let data_hash = input_data.get_hash();
    let query_result = sqlx::query_as::<_, ServiceMockResponse>(
        "SELECT * FROM service_mock_responses WHERE service_id = $1 AND data_hash = $2",
    )
    .bind(service_id)
    .bind(data_hash)
    .fetch_one(connection)
    .await;
    match query_result {
        Ok(row) => Ok(row.data),
        Err(error) => match error {
            SqlError::RowNotFound => Ok(String::new()),
            _ => Err(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json")
                .json(ApiError::new(
                    error.to_string(),
                    CustomStatusCode::DatabaseError,
                ))),
        },
    }
}

pub async fn insert_mock_response(
    connection: &Pool<Postgres>,
    input_data: MockResponseBody,
) -> Result<bool, String> {
    let query_result = sqlx::query(
        "INSERT INTO service_mock_responses (data_hash, data, service_id) VALUES ($1, $2, $3)",
    )
    .bind(input_data.data_hash)
    .bind(input_data.data)
    .bind(input_data.service_id)
    .execute(connection)
    .await;
    match query_result {
        Ok(_) => Ok(true),
        Err(err) => Err(err.to_string()),
    }
}
