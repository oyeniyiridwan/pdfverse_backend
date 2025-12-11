use redis::{AsyncTypedCommands,aio::MultiplexedConnection};
use uuid::Uuid;
use crate::{app::settings::Settings, domain::auth::Provider, utils::error::ApiError};


pub trait AuthRepository: Send + Sync{
    async fn read_email_from_redis(&self, provider: &Provider,token:&str) ->Result<String,ApiError>;
        async fn verify_token_and_create_new_redis(&self, provider: &Provider,token:&str) ->Result<String,ApiError>;

    async fn write_to_redis_and_get_token(&self, provider:&Provider, email: &str) ->Result<String,ApiError>;


}

pub  struct AuthRepositoryImpl{
 redis_db: MultiplexedConnection   
}

impl AuthRepositoryImpl {
    pub fn new(redis_db: MultiplexedConnection) -> Self {
        Self { redis_db }
    }
}


impl AuthRepository for AuthRepositoryImpl {
      async fn read_email_from_redis(&self, provider: &Provider,token:&str) ->Result<String,ApiError>{
     let key = format!("{}:{}", provider.to_string(), token);
          let mut con = self.redis_db.clone();

    let value: Option<String> = con
        .get(&key).await
        .map_err(|e| 
           ApiError::internal_msg(format!("error: {e}")))?;
    let email: String = match value {
        Some(json) =>json,
        None => {
            return Err(
               ApiError::Unauthorized("Invalid token".to_string()));
        }
    };
   con.del(&key).await.map_err(|e|
ApiError::internal_msg(format!("error: {e}"))
)?; 
   Ok(email)
      
      }


 async fn verify_token_and_create_new_redis(&self, provider: &Provider,token:&str) ->Result<String,ApiError>{
  let email =    self.read_email_from_redis(provider,token).await?;
            let mut con = self.redis_db.clone();


let token = Uuid::new_v4() ;
    let key = format!("{}:{}", provider.to_string(), &token);
    let _: () = con
        .set_ex(&key, &email, 600).await
        .map_err(|e| 
   ApiError::internal_msg(format!("error: {e}")))?;
   Ok(token.to_string())
      
      }



    async fn write_to_redis_and_get_token(&self, provider:&Provider, email: &str) ->Result<String,ApiError>{
      let mut con = self.redis_db.clone();
    let token = Uuid::new_v4() ;
    let key = format!("{}:{}", provider.to_string(), &token);
    let _: () = con
        .set_ex(&key, &email, 600).await
        .map_err(|e| 
   ApiError::internal_msg(format!("error: {e}")))?;

// let link: String = format!("{}/{}?token={}", settings.redirect_url,provider.to_string(),token);

Ok(token.to_string())
    }

}

