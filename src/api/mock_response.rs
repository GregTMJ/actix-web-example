use actix_web::{
    HttpResponse, Responder,
    http::StatusCode,
    post,
    web::{Bytes, Data, Path},
};
use log::{info, warn};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::{
    api::schemas::MockResponseBody,
    database::insert_mock_response,
    helpers::errors::{ApiError, StatusCode as CustomStatusCode},
};

#[post("/mock_response/{service_id}/{data_hash}")]
async fn add_mock_response(
    pool: Data<Pool<Postgres>>,
    query: Path<(String, String)>,
    bytes: Bytes,
) -> actix_web::Result<impl Responder> {
    let (service_id, data_hash) = query.into_inner();
    let input_data = match String::from_utf8(bytes.to_vec()) {
        Ok(text) => text,
        Err(err) => {
            return Ok(HttpResponse::build(StatusCode::BAD_REQUEST)
                .content_type("application/json")
                .json(ApiError {
                    message: err.to_string(),
                    code: CustomStatusCode::BadRequest,
                }));
        }
    };
    let service_id = match service_id.parse::<i32>() {
        Ok(val) => val,
        Err(e) => {
            return Ok(HttpResponse::build(StatusCode::BAD_REQUEST)
                .content_type("application/json")
                .json(ApiError {
                    message: e.to_string(),
                    code: CustomStatusCode::BadRequest,
                }));
        }
    };
    info!("Inserting mock data to service {service_id}");
    let data_to_insert = MockResponseBody {
        service_id,
        data_hash: data_hash.to_ascii_uppercase(),
        data: input_data,
    };
    let result = match insert_mock_response(pool.get_ref(), data_to_insert).await {
        Ok(result) => result,
        Err(err) => {
            warn!("mock response insertion error: {err}");
            false
        }
    };
    Ok(HttpResponse::Created().json(json!({"success": result})))
}
