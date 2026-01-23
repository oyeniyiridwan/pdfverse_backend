use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

use crate::{database::chat_info};

#[derive(Debug, Clone,)]
pub struct ChatInfo{
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub title: String,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub path: String,
}

impl From<chat_info::Model> for ChatInfo {
    fn from(chat_info: chat_info::Model) -> Self {
        Self {
            id: Some(chat_info.id),
            user_id: chat_info.user_id,
            title: chat_info.title,
            created_at: Some(chat_info.created_at),
            path: chat_info.path,
        }
    }
}

impl ChatInfo {
    pub fn new(
        user_id: &Uuid,
        title: &str,
        path: &str,
    ) -> Self {
        Self {
           id: None,
            user_id: user_id.to_owned(),
            title: title.to_string(),
           created_at: None,
            path: path.to_string(),
        }
    }
}
