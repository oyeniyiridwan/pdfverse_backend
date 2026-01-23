use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

use crate::database::chat_message;

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub id: Option<Uuid>,
    pub chat_info_id: Uuid,
    pub role: String,
    pub text: String,
    pub timestamp: Option<DateTimeWithTimeZone>,
}

impl ChatMessage {
    pub fn new(
        id: Option<Uuid>,
        chat_info_id: Uuid,
        is_user: bool,
        text: String,
        timestamp: Option<DateTimeWithTimeZone>,
    ) -> Self {
        let role = match is_user{
            true => "user".to_string(),
            false => "assistant".to_string(),
        };
        Self {
            id,
            chat_info_id,
            role,
            text,
            timestamp,
        }
    }
}

impl From<chat_message::Model> for ChatMessage {
    fn from(value: chat_message::Model) -> Self {
        Self {
            id: Some(value.id),
            chat_info_id: value.chat_info_id,
            role: value.role,
            text: value.text,
            timestamp: value.timestamp,
        }
    }
}
