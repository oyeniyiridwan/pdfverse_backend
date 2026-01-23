
use axum::serve;
mod database;
mod api;
mod utils;
mod app;
mod infrastructure;
mod services;
mod dtos;
mod domain;
mod repository;
use migration:: MigratorTrait;
use reqwest::Client;
use tokio::sync::RwLock;
mod adapters;
use std::{ error::Error, sync::Arc};

use crate::{adapters::storage_adapter::StorageAdapterImpl, api::routes::create_routes, app::{cache::AppCache, settings::Settings, state::{AppState, ConcreteAppState}}, infrastructure::{db::database_connection, email_client::EmailClient, redis::redis_database_connection, storage::get_s3_client}, repository::{auth_repository::AuthRepositoryImpl, chat_info_repository::ChatInfoRepositoryImpl, chat_message_repository::ChatMessageRepositoryImpl, document_chunk_repository::DocumentChunkRepositoryImpl, provider_repository::ProviderRepositoryImpl, task_repository::TaskRepositoryImpl, user_repository::UserRepositoryImpl}, services::{auth_service::AuthServiceImpl, chat_service::ChatServiceImpl, openai_service::{self, OpenAIConfig, OpenAIServiceImpl}, infrastructure_services::{EmailService, StorageServiceImpl}, task_service::TaskServiceImpl, user_service::UserServiceImpl}};





pub async fn run() -> Result<(), Box<dyn Error>> {
    let settings: Settings = Settings::from_env();

let database = database_connection(&settings.database_url).await?;
let  redis_database =redis_database_connection(&settings.redis_url).await?;
let email_client = EmailClient::new()?;
let provider_repo = Arc::new(
    ProviderRepositoryImpl::new(database.clone()));
let s3_client = get_s3_client();
    let email_service = EmailService::new(email_client);
let user_service = Arc::new(UserServiceImpl::new(
    Arc::new(UserRepositoryImpl::new(database.clone())
)));
let app_cache = AppCache::new(Arc::new(RwLock::new(None)),Arc::new(RwLock::new(None)));
let auth_repo = Arc::new(AuthRepositoryImpl::new(redis_database.clone()));
let auth_service = AuthServiceImpl{
    provider_repo: provider_repo,
    email_service: email_service,
    user_service: user_service,
    auth_repo: auth_repo,
    app_cache: app_cache,
};

let task_repo = TaskRepositoryImpl::new(database.clone());
let task_service = TaskServiceImpl::new(Arc::new(task_repo));


let storage_service = StorageServiceImpl::new(Arc::new(StorageAdapterImpl::new(s3_client)));
let chat_info_repo = ChatInfoRepositoryImpl::new(database.clone());
let chat_message_repo = ChatMessageRepositoryImpl::new(database.clone());
let document_chunk_repo = DocumentChunkRepositoryImpl::new(database.clone());
let client = Client::new();
let config = OpenAIConfig::default();
let openai_service = OpenAIServiceImpl::new(Arc::new(document_chunk_repo.clone()),client , config);
 let chat_service = ChatServiceImpl::new(Arc::new(chat_info_repo),
  Arc::new(chat_message_repo), 
  Arc::new(document_chunk_repo), Arc::new(storage_service),
  Arc::new(openai_service),
  database.clone());

let app_state:ConcreteAppState = Arc::new(AppState::new(auth_service, task_service,chat_service));
    


    migration::Migrator::up(&database, None).await?;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:3001")).await?;
    let app = create_routes(app_state);
    serve(listener, app).await?;

    Ok(())
}














