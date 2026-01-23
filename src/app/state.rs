use std::sync::Arc;

use crate::{
    adapters::storage_adapter::StorageAdapterImpl,
    repository::{
        auth_repository::AuthRepositoryImpl, chat_info_repository::ChatInfoRepositoryImpl, chat_message_repository::ChatMessageRepositoryImpl, document_chunk_repository::DocumentChunkRepositoryImpl, provider_repository::ProviderRepositoryImpl, task_repository::TaskRepositoryImpl, user_repository::UserRepositoryImpl
    },
    services::{
        auth_service::{AuthService, AuthServiceImpl}, chat_service::{ChatService, ChatServiceImpl}, openai_service::OpenAIServiceImpl, infrastructure_services::{StorageService, StorageServiceImpl}, task_service::{TaskService, TaskServiceImpl}, user_service::UserServiceImpl
    },
};

pub type ConcreteAppState = Arc<
    AppState<
        AuthServiceImpl<
            ProviderRepositoryImpl,
            UserServiceImpl<UserRepositoryImpl>,
            AuthRepositoryImpl,
        >,
        TaskServiceImpl<TaskRepositoryImpl>,
        ChatServiceImpl<
            ChatInfoRepositoryImpl,
            ChatMessageRepositoryImpl,
            DocumentChunkRepositoryImpl,
            StorageServiceImpl<StorageAdapterImpl>,
            OpenAIServiceImpl<DocumentChunkRepositoryImpl>
        >,
    >,
>;

#[derive(Clone, Copy)]
pub struct AppState<A, T, C>
where
    A: AuthService,
    T: TaskService,
    C: ChatService,
{
    pub auth_service: A,
    pub task_service: T,
    pub chat_service: C,
}

impl<A, T, C> AppState<A, T, C>
where
    A: AuthService,
    T: TaskService,
    C: ChatService,
{
    pub fn new(auth_service: A, task_service: T, chat_service: C) -> Self {
        Self {
            auth_service,
            task_service,
            chat_service,
        }
    }
}
