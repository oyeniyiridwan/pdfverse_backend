use std::{borrow::Cow, collections::HashMap};

use axum::{Json, extract::FromRequest, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use validator::{Validate, ValidationError};

use crate::{domain::user::User};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct RequestUser {
    #[validate(email())]
    pub email: String,
    #[validate(length(min = 8, message="must at least be 8 characters"),custom(function =validate_password))]
    pub password: String,
}

impl<S> FromRequest<S> for RequestUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = Json::<RequestUser>::from_request(req, state)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("error: {e}")))?;
        if let Err(e) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", e)));
        }
        Ok(user)
    }
}

fn validate_password(a: &str) -> Result<(), ValidationError> {
    let forbidden = &['.', '!', ',', ';'];
    let mut response: Vec<&str> = Vec::new();

    if !a.chars().any(|c| c.is_ascii_digit()) {
        response.push("must contain digit(s)");
    }
    if !a.chars().any(|c| c.is_uppercase()) {
        response.push("must contain uppercase character(s)");
    }
    if !a.chars().any(|c| c.is_lowercase()) {
        response.push("must contain lowercase character(s)");
    }
    if !a
        .chars()
        .any(|c| !c.is_alphanumeric() && !forbidden.contains(&c))
    {
        response.push("must contain special character(s), excluding (.,!;)");
    }
    if response.is_empty() {
        Ok(())
    } else {
        let mut err = ValidationError::new("password");
        let msg = format!("Password {}", response.join(", "));
        err.message = Some(Cow::Owned(msg));
        Err(err)
    }
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct RequestMagicLinkUser {
    #[validate(email())]
    pub email: String,
}

impl<S> FromRequest<S> for RequestMagicLinkUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = Json::<RequestMagicLinkUser>::from_request(req, state)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("error: {e}")))?;
        if let Err(e) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", e)));
        }
        Ok(user)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct UserResponse {
    pub id: String,
    pub email: Option<String>,
    pub session: Option<HashMap<String,Value>>,
    pub first_name:Option<String>,
    pub last_name:Option<String>
}


impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email,
            session: None,
            first_name:user.first_name,
            last_name: user.last_name,
        }
    }
}
