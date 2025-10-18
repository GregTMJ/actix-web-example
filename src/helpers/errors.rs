use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum StatusCode {
    BadRequest,
    DatabaseError,
    ErrorConflict,
    InternalError,
    Unauthorized,
    Forbidden,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
    pub code: StatusCode,
}

impl ApiError {
    pub fn new(
        message: &str,
        code: StatusCode,
    ) -> Self {
        Self {
            message: message.to_owned(),
            code,
        }
    }
}
