use uuid::Uuid;
use crate::utils::error::ApiError;
use crate::{database::users::{ActiveModel, Column,Entity as Users}, domain::user::User};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter
};

// #[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>,ApiError>;
        async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>,ApiError>;

    async fn create_user(&self, user: &User) -> Result<(),ApiError>;
        async fn update_user_by_id(&self, id: &Uuid,
        password: Option<String>,
        first_name: Option<String>,
                email_verified: Option<bool>,

         last_name: Option<String>,) -> Result<User,ApiError>;
           async fn update_user_by_email(&self, email: &str,
        password: Option<String>,
        first_name: Option<String>,
                email_verified: Option<bool>,

         last_name: Option<String>,) -> Result<User,ApiError>;

}






pub struct UserRepositoryImpl {
    db: DatabaseConnection,
}

impl UserRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

// #[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>,ApiError> {
        let user = Users::find()
            .filter(Column::Email.eq(email.to_lowercase()))
            .one(&self.db)
            .await
.map_err(|e|
    ApiError::internal_msg(format!("error: {e}"))
    )?;

        Ok(user.map(User::from))
    }


            async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>,ApiError>{
                let user = Users::find()
            .filter(Column::Id.eq(id.to_owned()))
            .one(&self.db)
            .await.map_err(|e|
    ApiError::internal_msg(format!("error: {e}"))
    )?;

        Ok(user.map(User::from)) 
            }


    async fn create_user(&self, user: &User) -> Result<(),ApiError> {
        let mut active_user = ActiveModel {
            id: Set(user.id.clone()),
            // email: Set(user.email.clone()),
            password: Set(user.password.clone()),
            email_verified:Set(user.email_verified),
            ..Default::default()
        };
if let Some(email) = user.email.clone(){
    active_user.email = Set(Some(email.to_lowercase()));
};

        active_user.insert(&self.db).await.map_err(|e|
    ApiError::internal_msg(format!("error: {e}"))
    )?;
        Ok(())
    }

     async fn update_user_by_id(&self, id: &Uuid,
        password: Option<String>,
        first_name: Option<String>,
                email_verified: Option<bool>,

         last_name: Option<String>,) -> Result<User,ApiError> {
                let user = Users::find()
            .filter(Column::Id.eq(id.to_owned()))
            .one(&self.db)
            .await.map_err(|e|
    ApiError::internal_msg(format!("error: {e}"))
    )?;
        
        let mut active_user = match user{
            Some(user) => user,
            None => {
          return Err(ApiError::NotFound(
                    "No record of user".to_string(),
                ));       
            },
        }.into_active_model();
       if let Some(ref _name) = first_name{
active_user.first_name = Set(first_name)
       }
       if let Some(ref _name) = last_name{
active_user.last_name = Set(last_name)
       }
       if let Some(ref _pass) = password{
active_user.password = Set(password)
       }
           if let Some( _email_verified) = email_verified{
active_user.email_verified = Set(_email_verified)
       }

 let user_model=  active_user.update(&self.db).await.map_err(|e|
    ApiError::internal_msg(format!("error: {e}"))
    )?;
        Ok(User::from(user_model))
    }







       async fn update_user_by_email(&self, email: &str,
        password: Option<String>,
        first_name: Option<String>,
                email_verified: Option<bool>,

         last_name: Option<String>,) -> Result<User,ApiError>
         {
                let user = Users::find()
            .filter(Column::Email.eq(email.to_lowercase().to_owned()))
            .one(&self.db)
            .await.map_err(|e|
    ApiError::internal_msg(format!("error: {e}"))
    )?;
        
        let mut active_user = match user{
            Some(user) => user,
            None => {
          return Err(ApiError::NotFound(
                    "No record of user".to_string(),
                ));       
            },
        }.into_active_model();
       if let Some(ref _name) = first_name{
active_user.first_name = Set(first_name)
       }
       if let Some(ref _name) = last_name{
active_user.last_name = Set(last_name)
       }
       if let Some(ref _pass) = password{
active_user.password = Set(password)
       }
        if let Some( _email_verified) = email_verified{
active_user.email_verified = Set(_email_verified)
       }

 let user_model=  active_user.update(&self.db).await.map_err(|e|
    ApiError::internal_msg(format!("error: {e}"))
    )?;
        Ok(User::from(user_model))
    }


}
