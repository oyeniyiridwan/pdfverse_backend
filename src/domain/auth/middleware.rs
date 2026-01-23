use axum::{extract::Request, middleware::Next, response::Response};
use chrono::Utc;
use crate::{domain::auth::{claim::Claim, decode_jwt}, utils::error::ApiError};


pub async fn auth_middleware(mut request: Request, next: Next) -> Result<Response, ApiError> {
    let headers = request.headers();

    let token_header = headers
        .get("Authorization")
        .ok_or_else(|| ApiError::Unauthorized("Please supply a token".to_string()))?;

    let token = token_header
        .to_str()
        .map_err(|_| ApiError::Unauthorized("Invalid token format".to_string()))?;

    let token = token.strip_prefix("Bearer ").unwrap_or(token);

    let claim:Claim = decode_jwt(token)
        .map_err(|_| ApiError::Unauthorized("Invalid token".to_string()))?;
    if claim.is_refresh_token{
        return Err(ApiError::Unauthorized("Invalid token type".to_string()));

    }
    let now = Utc::now().timestamp() as usize;
    match claim.exp{
        Some(value) => {
if now > value{
        return Err(ApiError::Unauthorized("Expired token".to_string()));

    }
        },
        None => {
             return Err(ApiError::Unauthorized("Expired token".to_string()));
        },
    }

     

    request.extensions_mut().insert(claim);
    Ok(next.run(request).await)
}
