//! Error type for error handling

use crate::types::ErrorInfo;
use thiserror::Error as ThisError;

/// Define all possible errors
#[derive(ThisError, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// 400
    #[error("Bad Request")]
    BadRequest,

    /// 401
    #[error("{0}")]
    Unauthorized(String),

    /// 403
    #[error("{0}")]
    Forbidden(String),

    /// 404
    #[error("Not Found")]
    NotFound,

    /// 409
    #[error("{0}")]
    Conflict(String),

    /// 422
    #[error("Unprocessable Entity: {0:?}")]
    UnprocessableEntity(ErrorInfo),

    /// 500
    #[error("Internal Server Error: {0:?}")]
    InternalServerError(String),

    /// serde deserialize error
    #[error("Deserialize Error")]
    DeserializeError,

    /// request error
    #[error("Http Request Error")]
    RequestError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_request() {
        let error = Error::BadRequest;
        assert_eq!(format!("{error}"), "Bad Request")
    }

    #[test]
    fn unauthorized() {
        let error = Error::Unauthorized("Test".to_string());
        assert_eq!(format!("{error}"), "Test")
    }

    #[test]
    fn forbidden() {
        let error = Error::Forbidden("Test".to_string());
        assert_eq!(format!("{error}"), "Test")
    }

    #[test]
    fn not_found() {
        let error = Error::NotFound;
        assert_eq!(format!("{error}"), "Not Found")
    }
}
