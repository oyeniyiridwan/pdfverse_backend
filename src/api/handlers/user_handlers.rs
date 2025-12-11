
use axum::{Extension, Json, extract::State, http::HeaderMap};

use crate::{ app::state::ConcreteAppState, custom_middleware::Claim, dtos::user_dto::{RequestMagicLinkUser, RequestUser, UserResponse}, services::{auth_service::AuthService, user_service::UserService}, utils::{error::ApiError, helper_function::{create_jwt_tokens, extract_platform_from_header, session_and_cookie_header}}};


pub async fn login_with_email_password( 
        headers:HeaderMap,

     State(state):State<ConcreteAppState>,

 user: RequestUser)-> Result<(HeaderMap, Json<UserResponse>),ApiError>{
let log_user = 
state.auth_service.login_with_email_password(user.email, user.password).await?;
 let platform: crate::utils::enums::Platform = extract_platform_from_header(headers)?;

  let (headers_out,session)= session_and_cookie_header(&platform,&log_user.id)?;
  let mut user_response = UserResponse::from(log_user);
user_response.session =Some(session );
Ok((headers_out, Json(user_response)))

}




pub async fn magic_link(
     State(state):State<ConcreteAppState>,
request_magic_link_user:RequestMagicLinkUser)->Result<String,ApiError>{
     state.auth_service.magic_link(request_magic_link_user.email).await
    
}












pub async fn signup_with_email_password(
     State(state):State<ConcreteAppState>,
    user: RequestUser,
) -> Result<String, ApiError> {
//     let log_user = 
state.auth_service.signup_with_email_password(user.email, user.password).await
// ?;
//  let token = create_jwt(&log_user.id)?;
//   let mut user_response = UserResponse::from(log_user);
// user_response.token =Some(token );
// Ok(Json(user_response))

}


pub async fn get_user(
          State(state):State<ConcreteAppState>,

        Extension(claim):Extension<Claim>,
) -> Result<Json<UserResponse>, ApiError> {
let user = state.auth_service.user_service.get_user_by_id(&claim.sub).await?;
    Ok(Json(UserResponse::from(user)))
}