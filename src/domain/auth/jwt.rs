

use std::collections::HashMap;

use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde_json::{ Value};
use uuid::Uuid;
use crate::{app::settings::Settings, domain::auth::claim::Claim, utils::error::ApiError};

pub fn create_jwt(user_id: &Uuid) -> Result<HashMap<String, Value>, ApiError> {
    let mut response = HashMap::new();
    let token_expiration = Utc::now()
        .checked_add_signed(Duration::minutes(10))
        .expect("failed")
        .timestamp() as usize;
    let token_claims = Claim {
        sub: user_id.to_owned(),
        exp: Some(token_expiration),
        is_refresh_token: false,
    };
    let refresh_claims = Claim {
        sub: user_id.to_owned(),
        exp: Some(Utc::now()
        .checked_add_signed(Duration::weeks(24*52))
        .expect("failed")
        .timestamp() as usize),
        is_refresh_token: true,
    };
    let secret = Settings::from_env().jwt_secret;
    let token = encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| ApiError::ExpectationFailed("failed to generate token".to_string()))?;

    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| ApiError::ExpectationFailed("failed to generate token".to_string()))?;
    response.insert("access_token".to_string(), Value::String(token));
    response.insert("refresh_token".to_string(), Value::String(refresh_token));
    response.insert(
        "expires_at".to_string(),
        Value::Number(token_expiration.into()),
    );
    response.insert("expires_in".to_string(), Value::Number(600.into()));

    Ok(response)
}

pub fn decode_jwt(token: &str) -> Result<Claim, ApiError> {
    println!("token is {}",token);
    let secret = Settings::from_env().jwt_secret;
    let validation = Validation::new(Algorithm::HS256);
    let decoding_key = DecodingKey::from_secret(secret.as_ref());

    match decode::<Claim>(token, &decoding_key, &validation) {
        Ok(data) => Ok(data.claims), // ✅ valid token
        Err(err) => Err(ApiError::Unauthorized(format!("❌ Invalid token: {}", err))),
    }
}
