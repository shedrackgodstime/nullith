use serde::Serialize;
use worker::*;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u16,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(code: u16, error: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code,
            error: error.into(),
            message: message.into(),
        }
    }

    pub fn not_found(resource: &str) -> Self {
        Self::new(404, "NOT_FOUND", format!("{} not found", resource))
    }

    pub fn bad_request(reason: &str) -> Self {
        Self::new(400, "BAD_REQUEST", reason)
    }

    pub fn unauthorized(reason: &str) -> Self {
        Self::new(401, "UNAUTHORIZED", reason)
    }

    pub fn internal(reason: &str) -> Self {
        Self::new(500, "INTERNAL_ERROR", reason)
    }
}

pub fn error_response(err: crate::error::AppError) -> Response {
    match err {
        crate::error::AppError::NotFound(msg) => {
            Response::from_json(&ErrorResponse::not_found(&msg)).unwrap()
        }
        crate::error::AppError::BadRequest(msg) => {
            Response::from_json(&ErrorResponse::bad_request(&msg)).unwrap()
        }
        crate::error::AppError::Unauthorized(msg) => {
            Response::from_json(&ErrorResponse::unauthorized(&msg)).unwrap()
        }
        _ => Response::from_json(&ErrorResponse::internal("An error occurred")).unwrap(),
    }
}