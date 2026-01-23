use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect
};
use uuid::Uuid;

use crate::{
    database::chat_message::{ActiveModel, Column, Entity},
    domain::chat_message::ChatMessage,
    utils::error::ApiError,
};

pub trait ChatMessageRepository: Send + Sync {
    async fn create_chat_message(        &self,
chat_message: ChatMessage) -> Result<ChatMessage, ApiError>;
    async fn find_chat_message_by_id_and_chat_info_id(
        &self,
        chat_info_id: &Uuid,
        id: &Uuid,
    ) -> Result<Option<ChatMessage>, ApiError>;

     async fn find_all_chat_messages_by_chat_info_id(
        &self,
        chat_info_id: &Uuid,
            limit: Option<u64>,

    ) -> Result<Vec<ChatMessage>, ApiError>;
     async fn insert_multiple_messages(
        &self,
        message_list: Vec<ChatMessage>,
    ) -> Result<Vec<ChatMessage>, ApiError>;
}

pub struct ChatMessageRepositoryImpl {
    db: DatabaseConnection,
}

impl ChatMessageRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
impl ChatMessageRepository for ChatMessageRepositoryImpl {
    async fn create_chat_message(        &self,
chat_message: ChatMessage) -> Result<ChatMessage, ApiError> {
       let active_chat_message = ActiveModel{
        chat_info_id: Set(chat_message.chat_info_id),
        role: Set(chat_message.role),
        text: Set(chat_message.text),
        ..Default::default()
    };
    let chat_message_model =active_chat_message.insert(&self.db).await.map_err(|e| 
        ApiError::internal_msg(format!("ChatMessageRepository: {e}")))?;
        Ok(ChatMessage::from(chat_message_model))
    }

    async fn find_chat_message_by_id_and_chat_info_id(
        &self,
        chat_info_id: &Uuid,
        id: &Uuid,
    ) -> Result<Option<ChatMessage>, ApiError> {
        let condition = Condition::all()
        .add(Column::ChatInfoId.eq(*chat_info_id))
        .add(Column::Id.eq(*id));
    let possible_chat_message_model = Entity::find().
    filter(condition).one(&self.db).await.map_err(|e| 
        ApiError::internal_msg(format!("find_chat_message: {e}")))?;
       Ok(possible_chat_message_model.map(ChatMessage::from))
    }
    
    async fn find_all_chat_messages_by_chat_info_id(
    &self,
    chat_info_id: &Uuid,
    limit: Option<u64>,
) -> Result<Vec<ChatMessage>, ApiError> {

    let mut query = Entity::find()
        .filter(Column::ChatInfoId.eq(*chat_info_id));

    if let Some(lim) = limit {
        query = query.order_by_desc(Column::Timestamp)
.limit(lim);
    }

    let messages = query
        .all(&self.db)
        .await
        .map_err(|e| {
            ApiError::internal_msg(format!("find_chat_message: {e}"))
        })?
        .into_iter()
        .map(ChatMessage::from)
        .collect::<Vec<ChatMessage>>();

    Ok(messages)
}


    async fn insert_multiple_messages(
        &self,
        message_list: Vec<ChatMessage>,
    ) -> Result<Vec<ChatMessage>, ApiError> {
        let list_of_active_messages = message_list
            .into_iter()
            .map(| message| ActiveModel {
                chat_info_id:Set(message.chat_info_id),
                role: Set(message.role),
                text: Set(message.text),
                ..Default::default()
            })
            .collect::<Vec<ActiveModel>>();
        let  messages = Entity::insert_many(list_of_active_messages)
            .exec_with_returning_many(&self.db)
            .await
            .map_err(|e| ApiError::internal_msg(format!("ChatMessageRepository-insert messages: {e}")))?
            .into_iter().map(|k| ChatMessage::from(k)).collect::<Vec<ChatMessage>>();

       
        Ok(messages)
    }
}
