
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
mod custom_middleware;
use migration:: MigratorTrait;
use tokio::sync::RwLock;
mod custom_filter;
use std::{ error::Error, sync::Arc};

use crate::{api::routes::create_routes, app::{cache::AppCache, settings::Settings, state::{AppState, ConcreteAppState}}, infrastructure::{db::database_connection, email_client::EmailClient, redis::redis_database_connection}, repository::{auth_repository::AuthRepositoryImpl, provider_repository::ProviderRepositoryImpl, task_repository::TaskRepositoryImpl, user_repository::UserRepositoryImpl}, services::{auth_service::AuthServiceImpl, email_service::EmailService, task_service::TaskServiceImpl, user_service::UserServiceImpl}};





pub async fn run() -> Result<(), Box<dyn Error>> {
    let settings: Settings = Settings::from_env();

let database = database_connection(&settings.database_url).await?;
println!("here adebara {}", &settings.database_url);
let  redis_database =redis_database_connection(&settings.redis_url).await?;
let email_client = EmailClient::new()?;
let provider_repo = Arc::new(
    ProviderRepositoryImpl::new(database.clone()));

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



let app_state:ConcreteAppState = Arc::new(AppState::new(auth_service, task_service));
    


    migration::Migrator::up(&database, None).await?;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:3001")).await?;
    let app = create_routes(app_state);
    serve(listener, app).await?;

    Ok(())
}














