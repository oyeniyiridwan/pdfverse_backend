











use std::{collections::HashMap};

use axum::{ Json, extract::{Path, Query, State}, http::HeaderMap, response::Redirect};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use serde_json::Value;
use crate::{app::state:: ConcreteAppState, domain::auth::Provider, dtos::{auth_dto::AuthToken, user_dto::UserResponse}, services::auth_service::AuthService, utils::{enums::Platform, error::ApiError, helper_function::{create_jwt_tokens, decode_jwt, extract_platform_from_header, extract_token_and_platform, refresh_token_helper, session_and_cookie_header}}};


pub async fn auth(
 Path(provider): Path<Provider>,
 State(state):State<ConcreteAppState>
) -> Result<Redirect,ApiError>
 {
   state.auth_service.get_authorization_url(provider).await
   
}


pub async fn handle_auth_callback(
    Query(map): Query<HashMap<String, String>>,
    Path(provider): Path<Provider>,
 State(state):State<ConcreteAppState>
) -> Result<Redirect,ApiError> {
state.auth_service.handle_auth_callback(map, provider).await
}
    


pub async fn auth_user(
   headers:HeaderMap,
 State(state):State<ConcreteAppState>,
    Path(provider): Path<Provider>,
   Query(auth_token) : Query<AuthToken>,
) -> Result<(HeaderMap, Json<UserResponse>),ApiError> {

  let user = 
  state.auth_service.create_or_get_user_via_token(provider, auth_token.token).await?;
   let platform = extract_platform_from_header(headers)?;

  let (headers_out,session)= session_and_cookie_header(&platform,&user.id)?;
  let mut user_response = UserResponse::from(user);
  println!("session is: {:?}",&session);
user_response.session =Some(session );
  Ok((headers_out, Json(user_response)))

}






pub async fn email_verification(
   State(state):State<ConcreteAppState>,
   Query(auth_token) : Query<AuthToken>,
)->Result<String,ApiError>{
  state.auth_service.email_verification(auth_token.token).await
}







pub async fn refresh_token(
      jar: CookieJar,
   headers:HeaderMap,
   Json(value):Json<Value>,
) -> Result<(HeaderMap,Json<HashMap<String,Value>>),ApiError> {
  let (token,platform) = extract_token_and_platform(jar, headers, value)?;
 
 refresh_token_helper(token, &platform).await


}


// let token = auth_token.refresh_token.ok_or_else(||{ApiError::BadRequest("Please supply refresh token".to_string())})?;
  
  
//   let claim = decode_jwt(&token)?;
//   if !claim.is_refresh_token {
//     return Err(ApiError::Forbidden("Invalid token type".to_string()));
//   }
//   let now = Utc::now().timestamp() as usize;
 

//   let session = create_jwt_tokens(&claim.sub)?;
//   Ok(Json(session))





