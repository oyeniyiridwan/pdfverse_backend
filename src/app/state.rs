
use std::sync::Arc;

use crate::{repository::{auth_repository::AuthRepositoryImpl, provider_repository::ProviderRepositoryImpl, task_repository::TaskRepositoryImpl, user_repository::UserRepositoryImpl}, services::{auth_service::{AuthService, AuthServiceImpl}, task_service::{TaskService, TaskServiceImpl}, user_service::UserServiceImpl}};


pub type ConcreteAppState = Arc<AppState<AuthServiceImpl<ProviderRepositoryImpl, UserServiceImpl<UserRepositoryImpl>, AuthRepositoryImpl>, TaskServiceImpl<TaskRepositoryImpl>>>;

#[derive(Clone,Copy)]
pub struct AppState<A,T> 
where 
A:AuthService,
T:TaskService {
    pub auth_service:A,
    pub task_service: T,
}

impl<A, T> AppState<A, T> 
where 
A: AuthService, 
T: TaskService {
    pub fn new(auth_service: A, task_service: T) -> Self {
        Self { auth_service, task_service }
    }
}
