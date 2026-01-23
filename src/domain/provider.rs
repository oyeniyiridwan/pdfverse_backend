use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

use crate::{database::providers};

pub struct Provider {
    pub provider_name: String,
    pub external_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTimeWithTimeZone>,
    pub email: Option<String>,
    pub user_id: Uuid,
}

impl Provider {
  pub  fn new(sub_id: String,email: Option<String>, user_id: &Uuid,provider_name:String ) -> Self {
       Self { provider_name: provider_name, external_id: sub_id, access_token: None, refresh_token:None, expires_at: None, email: email, user_id: user_id.to_owned() }
    }
}

impl From<providers::Model> for Provider {
    fn from(provider: providers::Model) -> Self {
        Self { provider_name: provider.provider_name, external_id: provider.external_id, access_token: provider.access_token, refresh_token: provider.refresh_token, expires_at: provider.expires_at, email:provider.email, user_id: provider.user_id }
    }
}