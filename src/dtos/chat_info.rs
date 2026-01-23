
use axum::{Json, extract::FromRequest, http::StatusCode};
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use validator::Validate;

use crate::domain::chat_info::ChatInfo;

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct RequestChatInfo {
   pub id: String,
}

impl<S> FromRequest<S> for RequestChatInfo
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(chat_info) = Json::<RequestChatInfo>::from_request(req, state)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("error: {e}")))?;
        if let Err(e) = chat_info.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", e)));
        }
        Ok(chat_info)
    }
}



#[skip_serializing_none]
#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct ChatInfoResponse {
   pub id: String,
    pub user_id: Option<String>,
    pub title: String,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub path: String,
}


impl From<ChatInfo> for ChatInfoResponse {
    fn from(chat_info: ChatInfo) -> Self {
        Self { id: chat_info.id.expect("chat info id not supplied by chat info domain").to_string(), user_id: None, title: chat_info.title, created_at: None, path: chat_info.path }
    }
}
