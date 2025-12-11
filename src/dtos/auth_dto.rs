use axum::{Json, {extract::FromRequest, http::StatusCode}};
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthClaims {
    pub sub: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub aud: String,
    pub iss: String,
    pub exp: usize,
}




#[derive(Serialize,Deserialize,Validate)]
pub struct  AuthToken {
    pub token:Option<String>,
    pub refresh_token: Option<String>
}



impl<S> FromRequest<S> for AuthToken
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(auth_token) = Json::<AuthToken>::from_request(req, state)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("error: {e}")))?;
        if let Err(e) = auth_token.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", e)));
        }
        Ok(auth_token)
    }
}