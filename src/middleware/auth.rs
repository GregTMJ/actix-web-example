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
                    .json(ApiError::new(
                        msg.to_string(),
                        CustomStatusCode::InternalError,
                    )),
            ));
        }
    };
    match req.headers().get("X-Api-Key") {
        Some(val) => {
            if val.to_owned() == system_api_key {
                next.call(req).await
            } else {
                Ok(req.into_response(
                    HttpResponse::build(StatusCode::UNAUTHORIZED)
                        .content_type("application/json")
                        .json(ApiError::new(
                            "Api key did not match".to_string(),
                            CustomStatusCode::Unauthorized,
                        )),
                ))
            }
        }
        None => Ok(req.into_response(
            HttpResponse::build(StatusCode::UNAUTHORIZED)
                .content_type("application/json")
                .json(ApiError::new(
                    "Api key missing".to_string(),
                    CustomStatusCode::Unauthorized,
                )),
        )),
    }
}
