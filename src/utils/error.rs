use anyhow::{self, Error};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;



#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Expectation failed: {0}")]
    ExpectationFailed(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Bad gateway: {0}")]
    BadGateway(String),
    #[error("Internal server error: {0}")]
    Internal(Error),
     #[error("Email success: {0}")]
    EmailSuccess(String),
     #[error("Forbidden success: {0}")]
    Forbidden(String),
     #[error("Request timeout success: {0}")]
    RequestTimeout(String),
    
}

impl ApiError {
  

    pub fn internal_msg(msg: impl Into<String>) -> Self {
        Self::Internal(anyhow::anyhow!(msg.into()))
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        Self::Internal(Error::new(err))
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Self::Internal(Error::new(err))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {

        let (status, message,success) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg,false),
            ApiError::ExpectationFailed(msg) => (StatusCode::EXPECTATION_FAILED, msg,false),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg,false),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg,false),
            ApiError::BadGateway(msg) => (StatusCode::BAD_GATEWAY, msg,false),
             ApiError::EmailSuccess(msg) => (StatusCode::ACCEPTED, msg,true),
                          ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg,true),
                          ApiError::RequestTimeout(msg) => (StatusCode::REQUEST_TIMEOUT, msg,true),

            ApiError::Internal(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {err}"),false
            ),
        };
       

        let body = Json(json!({
            "error": message,
            "success": success
        }));

        (status, body).into_response()
    }
}






