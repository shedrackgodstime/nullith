use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

#[allow(dead_code)]
pub type AppResult<T> = Result<T, AppError>;

#[allow(dead_code)]
pub fn not_found(message: impl Into<String>) -> AppError {
    AppError::NotFound(message.into())
}

#[allow(dead_code)]
pub fn bad_request(message: impl Into<String>) -> AppError {
    AppError::BadRequest(message.into())
}

#[allow(dead_code)]
pub fn internal(message: impl Into<String>) -> AppError {
    AppError::Internal(message.into())
}

#[allow(dead_code)]
pub fn unauthorized(message: impl Into<String>) -> AppError {
    AppError::Unauthorized(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_not_found() {
        let err = not_found("item not found");
        match err {
            AppError::NotFound(msg) => assert_eq!(msg, "item not found"),
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn test_error_bad_request() {
        let err = bad_request("invalid input");
        match err {
            AppError::BadRequest(msg) => assert_eq!(msg, "invalid input"),
            _ => panic!("Expected BadRequest error"),
        }
    }

    #[test]
    fn test_error_unauthorized() {
        let err = unauthorized("invalid token");
        match err {
            AppError::Unauthorized(msg) => assert_eq!(msg, "invalid token"),
            _ => panic!("Expected Unauthorized error"),
        }
    }

    #[test]
    fn test_error_serialization() {
        let err = not_found("test");
        let json = serde_json::to_string(&err).unwrap();
        assert!(json.contains("NotFound"));
        assert!(json.contains("test"));
    }

    #[test]
    fn test_result_handling() {
        let result: AppResult<i32> = Ok(42);
        assert!(result.is_ok());

        let err_result: AppResult<i32> = Err(not_found("test"));
        assert!(err_result.is_err());
    }
}
