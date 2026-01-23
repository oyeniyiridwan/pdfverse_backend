
use axum::{Json, extract::FromRequest, http::StatusCode};
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use validator::{Validate, ValidationError};

use crate::domain::chat_message::ChatMessage;


#[derive(Serialize, Deserialize, Validate, Debug)]

pub struct RequestChatMessage {
   pub id: Option<String>,
   pub chat_id: String,
   pub text: String
}

impl<S> FromRequest<S> for RequestChatMessage
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(chat_message) = Json::<RequestChatMessage>::from_request(req, state)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("error: {e}")))?;
        if let Err(e) = chat_message.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", e)));
        }
        Ok(chat_message)
    }
}






#[skip_serializing_none]
#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct ChatMessageResponse {
     pub id: String,
    pub chat_id: Option<String>,
    pub role: String,
    pub text: String,
    pub timestamp: Option<DateTimeWithTimeZone>,
  
}


impl From<ChatMessage> for ChatMessageResponse {
    fn from(chat_message: ChatMessage) -> Self {
        Self { id:
            chat_message.id.expect("chat messages not supplied by chat info domain").to_string(),
         chat_id: Some(chat_message.chat_info_id.to_string()),
          role: chat_message.role, text: chat_message.text, timestamp: chat_message.timestamp }
    }
}


