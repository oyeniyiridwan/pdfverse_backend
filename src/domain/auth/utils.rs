use std::{collections::HashMap, str::FromStr};

use crate::app::settings::Settings;
use crate::domain::auth::{create_jwt, decode_jwt};
use crate::domain::auth::provider::Provider;
use crate::{
    utils::{enums::Platform, error::ApiError},
};

use axum::http::HeaderValue;
use axum::{Json, http::HeaderMap};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use reqwest::header;
use serde_json::{ Value};
use time::Duration as timeDuration;
use uuid::Uuid;



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
    let mut session = create_jwt(user_id)?;
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
                .map_err(|e| ApiError::internal_msg(format!("session_and_cookie_header: {}", e)))?,
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
    println!("decoded claim {:?}", &claim);
    if !claim.is_refresh_token {
        return Err(ApiError::Forbidden("Invalid token type".to_string()));
    }
    // let now = Utc::now().timestamp() as usize;
    let (headers_out, session) = session_and_cookie_header(platform, &claim.sub)?;

    Ok((headers_out, Json(session)))
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