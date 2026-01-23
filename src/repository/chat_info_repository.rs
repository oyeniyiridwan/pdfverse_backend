use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection,
    DatabaseTransaction, EntityTrait, QueryFilter,
};
use uuid::Uuid;

use crate::{
    database::chat_info::{ActiveModel, Column, Entity},
    domain::chat_info::ChatInfo,
    utils::error::ApiError,
};

pub trait ChatInfoRepository: Send + Sync {
    async fn create_chat_info(&self, chat_info: ChatInfo) -> Result<ChatInfo, ApiError>;
    fn create_chat_info_via_db_transaction(
        &self,
        db: &DatabaseTransaction,
        chat_info: ChatInfo,
    ) -> impl Future<Output = Result<ChatInfo, ApiError>> + Send;
    async fn find_chat_info_by_id_and_user_id(
        &self,
        user_id: &Uuid,
        id: &Uuid,
    ) -> Result<Option<ChatInfo>, ApiError>;

    async fn find_all_chat_info_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ChatInfo>, ApiError>;
}

pub struct ChatInfoRepositoryImpl {
    db: DatabaseConnection,
}

impl ChatInfoRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl ChatInfoRepository for ChatInfoRepositoryImpl {
    async fn create_chat_info(&self, chat_info: ChatInfo) -> Result<ChatInfo, ApiError> {
        let active_chat_info = ActiveModel {
            user_id: Set(chat_info.user_id),
            title: Set(chat_info.title),
            path: Set(chat_info.path),
            ..Default::default()
        };
        let chat_info_model = active_chat_info
            .insert(&self.db)
            .await
            .map_err(|e| ApiError::internal_msg(format!("ChatInfoRepository: {e}")))?;
        Ok(ChatInfo::from(chat_info_model))
    }

    async fn find_chat_info_by_id_and_user_id(
        &self,
        user_id: &Uuid,
        id: &Uuid,
    ) -> Result<Option<ChatInfo>, ApiError> {
        let condition = Condition::all()
            .add(Column::UserId.eq(*user_id))
            .add(Column::Id.eq(*id));
        let possible_chat_info_model = Entity::find()
            .filter(condition)
            .one(&self.db)
            .await
            .map_err(|e| ApiError::internal_msg(format!("find_chat: {e}")))?;
        Ok(possible_chat_info_model.map(ChatInfo::from))
    }

    async fn create_chat_info_via_db_transaction(
        &self,
        db: &DatabaseTransaction,
        chat_info: ChatInfo,
    ) -> Result<ChatInfo, ApiError> {
        let active_chat_info = ActiveModel {
            user_id: Set(chat_info.user_id),
            title: Set(chat_info.title),
            path: Set(chat_info.path),
            ..Default::default()
        };
        let chat_info_model = active_chat_info
            .insert(db)
            .await
            .map_err(|e| ApiError::internal_msg(format!("create_chat: {e}")))?;
        Ok(ChatInfo::from(chat_info_model))
    }

    async fn find_all_chat_info_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ChatInfo>, ApiError> {
        let chat_infos = Entity::find()
            .filter(Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| ApiError::internal_msg(format!("find chat user: {e}")))?
            .into_iter()
            .map(|e| ChatInfo::from(e))
            .collect::<Vec<ChatInfo>>();
        Ok(chat_infos)
    }
}
