use crate::{database::providers::{ActiveModel, Column,Entity as Providers}, domain::{provider::Provider}};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection,
    EntityTrait, QueryFilter,
};
use crate::utils::error::ApiError;
pub trait ProviderRepository: Send + Sync {
    async fn find_by_external_id(&self, sub: &str) -> Result<Option<Provider>,ApiError>;
    async fn create_provider(&self, provider: &Provider) -> Result<(),ApiError>;
}






pub struct ProviderRepositoryImpl {
    db: DatabaseConnection,
}

impl ProviderRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl ProviderRepository for ProviderRepositoryImpl {
    async fn find_by_external_id(&self, sub: &str) -> Result<Option<Provider>,ApiError>{
        let provider = Providers::find()
            .filter(Column::ExternalId.eq(sub))
            .one(&self.db)
            .await.map_err(|e|
    ApiError::internal_msg(format!("error: {e}"))
    )?;

        Ok(provider.map(Provider::from))
    }

    async fn create_provider(&self, provider: &Provider) -> Result<(),ApiError>{
        let mut active_provider: ActiveModel = ActiveModel { 
             provider_name:Set(provider.provider_name.to_owned()), 
             external_id:Set(provider.external_id.to_owned()),
            //   email: Set(provider.email.to_owned()),
               user_id: Set(provider.user_id.to_owned()),
               
               ..Default::default()
             };
             if let Some(email) = provider.email.clone(){
    active_provider.email = Set(Some(email.to_lowercase()));
};

        active_provider.insert(&self.db).await.map_err(|e|
    ApiError::internal_msg(format!("error: {e}"))
    )?;
        Ok(())

    }
}
