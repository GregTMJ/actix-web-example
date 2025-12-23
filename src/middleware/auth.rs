use std::env;

use actix_web::{
    Error, HttpResponse,
    body::{BoxBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    http::StatusCode,
    middleware::Next,
};

use crate::helpers::errors::{ApiError, StatusCode as CustomStatusCode};

pub async fn extract_auth_key(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let system_api_key: String = match env::var("X_API_KEY") {
        Ok(value) => value,
        Err(msg) => {
            return Ok(req.into_response(
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .content_type("application/json")
                    .json(ApiError {
                        message: msg.to_string(),
                        code: CustomStatusCode::InternalError,
                    }),
            ));
        }
    };
    let api_error: ApiError;
    match req.headers().get("X-Api-Key") {
        Some(val) => {
            if *val == system_api_key {
                return next.call(req).await;
            } else {
                api_error = ApiError {
                    message: String::from("Api key did not match"),
                    code: CustomStatusCode::Unauthorized,
                }
            }
        }
        None => {
            api_error = ApiError {
                message: String::from("Api key is missing"),
                code: CustomStatusCode::Unauthorized,
            }
        }
    }
    Ok(req.into_response(
        HttpResponse::build(StatusCode::UNAUTHORIZED)
            .content_type("application/json")
            .json(api_error),
    ))
}
