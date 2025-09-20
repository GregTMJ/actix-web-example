use actix_web::{
    HttpResponse, Responder,
    http::StatusCode,
    post,
    web::{self, Bytes},
};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::{
    api::schemas::MockResponseBody,
    database::insert_mock_response,
    helpers::errors::{ApiError, StatusCode as CustomStatusCode},
};

#[post("/mock_response/{service_id}/{data_hash}")]
async fn add_mock_response(
    pool: web::Data<Pool<Postgres>>,
    query: web::Path<(String, String)>,
    bytes: Bytes,
) -> actix_web::Result<impl Responder> {
    let (service_id, data_hash) = query.into_inner();
    let input_data = match String::from_utf8(bytes.to_vec()) {
        Ok(text) => text,
        Err(err) => {
            return Ok(HttpResponse::build(StatusCode::BAD_REQUEST)
                .content_type("application/json")
                .json(ApiError::new(err.to_string(), CustomStatusCode::BadRequest)));
        }
    };
    let data_to_insert = MockResponseBody {
        service_id: service_id.parse::<i32>().unwrap(),
        data_hash: data_hash.to_ascii_uppercase(),
        data: input_data,
    };
    let result = match insert_mock_response(pool.get_ref(), data_to_insert).await {
        Ok(result) => result,
        Err(err) => {
            println!("Got an error while inserting {err}");
            false
        }
    };
    Ok(HttpResponse::Created().json(json!({"success": result})))
}
