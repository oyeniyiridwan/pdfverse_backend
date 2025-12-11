use std::{collections::HashMap, str::FromStr};

use crate::{app::settings::Settings, custom_middleware::Claim};
use crate::{
    domain::auth::Provider,
    utils::{enums::Platform, error::ApiError},
};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::http::HeaderValue;
use axum::{Json, http::HeaderMap};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use reqwest::header;
use serde_json::{ Value};
use time::Duration as timeDuration;
use uuid::Uuid;

pub fn create_jwt_tokens(user_id: &Uuid) -> Result<HashMap<String, Value>, ApiError> {
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
    let secret = Settings::from_env().secret;
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
    let secret = Settings::from_env().secret;
    // .map_err(|_| ApiError::internal_msg("failed to generate token".to_string()))?;
    let validation = Validation::new(Algorithm::HS256);
    let decoding_key = DecodingKey::from_secret(secret.as_ref());

    match decode::<Claim>(token, &decoding_key, &validation) {
        Ok(data) => Ok(data.claims), // ✅ valid token
        Err(err) => Err(ApiError::Unauthorized(format!("❌ Invalid token: {}", err))),
    }
}

pub fn extract_names(names: &Option<String>) -> (Option<String>, Option<String>) {
    match names {
        Some(names) => {
            let parts: Vec<&str> = names.split_whitespace().collect();
            if parts.is_empty() {
                (None, None)
            } else {
                if parts.len() > 1 {
                    (Some((parts[0]).to_string()), Some((parts[1]).to_string()))
                } else {
                    (Some((parts[0]).to_string()), None)
                }
            }
        }
        None => (None, None),
    }
}

pub fn hash_password(password: String) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let password = password.as_bytes();
    let password_argon = Argon2::default();
    match password_argon.hash_password(&password, &salt) {
        Ok(password) => Ok(password.to_string()),
        Err(e) => Err(ApiError::internal_msg(format!("encryption error; {e}"))),
    }
}

pub fn verify_password(hash_password: &str, password: &str) -> Result<bool, ApiError> {
    let password_argon = Argon2::default();

    let parsed_hash = PasswordHash::new(&hash_password)
        .map_err(|e| ApiError::internal_msg(format!("decryption error: {e}")))?;
    let password = password.as_bytes();
    Ok(password_argon
        .verify_password(password, &parsed_hash)
        .is_ok())
}

pub fn decide_redirect_link(token: &str, provider: &Provider) -> Result<String, ApiError> {
    let settings: Settings = Settings::from_env();

    let link = match provider {
        Provider::Signup | Provider::Login => {
            format!("{}/verify_email?token={}", settings.app_url, token)
        }
        Provider::MagicLink => format!(
            "{}/?provider={}&token={}",
            settings.app_url,
            provider.to_string(),
            token
        ),
        _ => return Err(ApiError::Forbidden("Invalid provider Path".to_string())),
    };
    Ok(link)
}



pub fn extract_token_and_platform(
    jar: CookieJar,
    headers: HeaderMap,
    value: Value,
) -> Result<(Option<String>, Platform), ApiError> {
    // Extract platform from headers
    let platform = extract_platform_from_header(headers)?;

    // Extract token based on platform
    let token = match platform {
        Platform::Web => jar.get("refresh_token").map(|v| v.value().to_string()),
        Platform::Mobile => value
            .get("refresh_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
    };

    Ok((token, platform))
}


pub fn extract_platform_from_header(headers: HeaderMap) -> Result<Platform, ApiError> {
    let platform_str = headers
        .get("x-platform")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ApiError::BadRequest("Platform header not supplied".to_string()))?;

    let platform = Platform::from_str(platform_str)?;
    Ok(platform)
}

pub fn session_and_cookie_header(
    platform: &Platform,
    user_id: &Uuid,
) -> Result<(HeaderMap, HashMap<String, Value>), ApiError> {
    let mut session = create_jwt_tokens(user_id)?;
    let mut headers_out = HeaderMap::new();
    if *platform == Platform::Web {
        let refresh_token = session.remove("refresh_token").unwrap().to_string();
let cookie_str = Cookie::build(("refresh_token", refresh_token.clone()))
    .http_only(true)
    .secure(true)
    .same_site(SameSite::Strict)
    .path("/")
    .max_age(timeDuration::days(7))
    // .domain("shevyverse.com")
    .to_string()
    .replace("\"", ""); // Remove quotes after building

headers_out.insert(
    header::SET_COOKIE,
    HeaderValue::from_str(&cookie_str)
        .map_err(|e| ApiError::internal_msg(format!("error: {}", e)))?,
);
    }
    Ok((headers_out, session))
}




pub async fn refresh_token_helper(
    token: Option<String>,
    platform: &Platform,
) -> Result<(HeaderMap, Json<HashMap<String, Value>>), ApiError> {
    let token = match token {
        Some(value) => value,
        None => {
            return Err(ApiError::BadRequest("Kindly supply token".to_string()));
        }
    };
    let claim = decode_jwt(&token)?;
    println!("decoded claim {:?}",&claim);
    if !claim.is_refresh_token {
        return Err(ApiError::Forbidden("Invalid token type".to_string()));
    }
    // let now = Utc::now().timestamp() as usize;
    let (headers_out, session) = session_and_cookie_header(platform, &claim.sub)?;

    Ok((headers_out, Json(session)))
}