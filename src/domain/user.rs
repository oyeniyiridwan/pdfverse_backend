use uuid::Uuid;

use crate::{database::users, domain::auth::provider::Provider};

#[derive(Debug, Clone)]
pub struct User{
    pub id:Uuid,
    pub email: Option<String>,
    pub password:Option<String>,
     pub first_name:Option<String>,
    pub last_name:Option<String>,
    pub email_verified: bool
}



impl From<users::Model> for User {
    fn from(user: users::Model) -> Self {
     Self {email_verified:user.email_verified,
         id: user.id, email: user.email, password: user.password,  first_name: user.first_name, last_name: user.last_name }
    }
}

impl User {
    pub fn new(email:Option<String>,password_hash: Option<String>,  first_name: Option<String>,
                last_name: Option<String>,provider: &Provider)->Self{
            let        email_verified = match provider{
                Provider::Google => true,
                Provider::LinkedIn => true,
                Provider::Signup => false,
                 Provider::Login => false,
                Provider::MagicLink => false,
            };
        Self{
            id: Uuid::new_v4(),
            email: email,
            password: password_hash,
            first_name: first_name,
            last_name: last_name,
            email_verified:email_verified
        }
    }
}