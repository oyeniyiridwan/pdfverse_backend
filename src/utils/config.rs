use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{app::settings::Settings, domain::auth::Provider, utils::{constant::{GOOGLE_JWKS_URL, GOOGLE_REDIRECT_BASE_URL, LINKEDIN_JWKS_URL, LINKEDIN_REDIRECT_BASE_URL, LINKEDIN_URL, OAUTH_URL}, error::ApiError}};









pub struct OAUTHConfig {
  pub  client_id: String,
   pub client_secret: String,
   pub token_url: String,
  pub  auth_base_url: String,
  pub  redirect_uri: String,
}

impl OAUTHConfig {
  pub  fn for_provider(provider: &Provider) -> Result<Self, ApiError> {
        let settings = Settings::from_env();
        let redirect_uri_base = settings.redirect_url;
        match provider {
            Provider::Google => Ok(Self {
                client_id: settings.google_client_id,
                client_secret: settings.google_secret,
                token_url: OAUTH_URL.to_string(),
                auth_base_url: GOOGLE_REDIRECT_BASE_URL.to_string(),
                redirect_uri: format!("{}/auth/{}/callback", redirect_uri_base,provider.to_string()),
            }),
            Provider::LinkedIn => Ok(Self {
                                client_id:settings.linkedin_client_id,
                client_secret: settings.linkedin_secret,
                token_url: LINKEDIN_URL.to_string(),
                auth_base_url:LINKEDIN_REDIRECT_BASE_URL.to_string(),
                redirect_uri: format!("{}/auth/{}/callback", redirect_uri_base, provider.to_string()),
            }),
            _ => {
                return Err(ApiError::NotFound("provider not available".to_string()));
            }
        }
    }

pub fn jwks_url(provider: &Provider)->Result<String,ApiError>{
   let url = match *provider {
            Provider::Google => GOOGLE_JWKS_URL,
            Provider::LinkedIn => LINKEDIN_JWKS_URL,
            _ => {
                return Err(ApiError::NotFound("provider not available".to_string()));
            }
        };
        Ok(url.to_owned())
}
}



