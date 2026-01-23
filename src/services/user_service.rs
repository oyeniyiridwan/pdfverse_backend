use crate::domain::auth::hash_password;
use crate::domain::auth::provider::Provider;
use crate::utils::error::ApiError;
use crate::{
    domain::{user::User},
    repository::user_repository::UserRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub trait UserService {
    async fn register_user(
        &self,

        email: Option<String>,
        password: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        provider: &Provider,
    ) -> Result<User, ApiError>;
    async fn update_user_by_id(
        &self,
        id: &Uuid,
        password: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        email_verified: Option<bool>,
    ) -> Result<User, ApiError>;
    async fn update_user_by_email(
        &self,
        email: &str,
        password: Option<String>,
        first_name: Option<String>,
                email_verified: Option<bool>,
        last_name: Option<String>,
    ) -> Result<User, ApiError>;
    async fn get_user_by_email(&self, email: String) -> Result<Option<User>, ApiError>;
    async fn get_user_by_id(&self, id: &Uuid) -> Result<User, ApiError>;
}

pub struct UserServiceImpl<U: UserRepository + Send + Sync> {
    pub user_repo: Arc<U>,
}

impl<U: UserRepository + Send + Sync> UserServiceImpl<U> {
    pub fn new(user_repo: Arc<U>) -> Self {
        Self { user_repo }
    }
}

// #[async_trait]
impl<U> UserService for UserServiceImpl<U>
where
    U: UserRepository + Send + Sync,
{
    async fn register_user(
        &self,
        email: Option<String>,
        password: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        provider: &Provider,
    ) -> Result<User, ApiError> {
        let password_hash = match password {
            Some(a) => Some(hash_password(a)?),
            None => None,
        };
        let user = User::new(email, password_hash, first_name, last_name, provider);
        self.user_repo.create_user(&user).await?;
        Ok(user)
    }

    async fn get_user_by_email(&self, email: String) -> Result<Option<User>, ApiError> {
        let user = self.user_repo.find_by_email(&email).await?;
        println!("response {}",user.is_none());
        Ok(user)
    }

    async fn get_user_by_id(&self, id: &Uuid) -> Result<User, ApiError> {
        let user = self.user_repo.find_by_id(id).await?.ok_or_else(|| ApiError::NotFound("User does not exist".to_string()))?;
        Ok(user)
    }

    async fn update_user_by_id(
        &self,
        id: &Uuid,
        password: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        email_verified: Option<bool>,
    ) -> Result<User, ApiError> {
        let password_hash = match password {
            Some(a) => Some(hash_password(a)?),
            None => None,
        };
        self.user_repo
            .update_user_by_id(id, password_hash, first_name, email_verified, last_name)
            .await
    }

    async fn update_user_by_email(
        &self,
        email: &str,
        password: Option<String>,
        first_name: Option<String>,
        email_verified: Option<bool>,
                last_name: Option<String>,

    ) -> Result<User, ApiError> {
        let password_hash = match password {
            Some(a) => Some(hash_password(a)?),
            None => None,
        };
        self.user_repo
            .update_user_by_email(email, password_hash, first_name, email_verified, last_name)
            .await
    }
}
