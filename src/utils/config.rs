use crate::{
    app::settings::Settings,
    domain::auth::provider::Provider,
    utils::{
        constant::{
            GOOGLE_JWKS_URL, GOOGLE_REDIRECT_BASE_URL, LINKEDIN_JWKS_URL,
            LINKEDIN_REDIRECT_BASE_URL, LINKEDIN_URL, OAUTH_URL,
        },
        enums::StorageProvider,
        error::ApiError,
    },
};

pub struct OAUTHConfig {
    pub client_id: String,
    pub client_secret: String,
    pub token_url: String,
    pub auth_base_url: String,
    pub redirect_uri: String,
}

impl OAUTHConfig {
    pub fn for_provider(provider: &Provider) -> Result<Self, ApiError> {
        let settings: Settings = Settings::from_env();
        let redirect_uri_base = settings.redirect_url;
        match provider {
            Provider::Google => Ok(Self {
                client_id: settings.google_client_id,
                client_secret: settings.google_secret,
                token_url: OAUTH_URL.to_string(),
                auth_base_url: GOOGLE_REDIRECT_BASE_URL.to_string(),
                redirect_uri: format!(
                    "{}/auth/{}/callback",
                    redirect_uri_base,
                    provider.to_string()
                ),
            }),
            Provider::LinkedIn => Ok(Self {
                client_id: settings.linkedin_client_id,
                client_secret: settings.linkedin_secret,
                token_url: LINKEDIN_URL.to_string(),
                auth_base_url: LINKEDIN_REDIRECT_BASE_URL.to_string(),
                redirect_uri: format!(
                    "{}/auth/{}/callback",
                    redirect_uri_base,
                    provider.to_string()
                ),
            }),
            _ => {
                return Err(ApiError::NotFound("provider not available".to_string()));
            }
        }
    }

    pub fn jwks_url(provider: &Provider) -> Result<String, ApiError> {
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

pub struct StorageConfig {
    pub endpoint: String,
    pub bucket: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
    pub provider: StorageProvider,
}

impl StorageConfig {
    pub fn new() -> Self {
        let settings: Settings = Settings::from_env();

        Self {
            endpoint: settings.s3_endpoint,
            bucket: settings.s3_bucket,
            region: settings.s3_region,
            access_key: settings.s3_access_key,
            secret_key: settings.s3_secret_key,
            provider: StorageProvider::from_env(),
        }
    }
}
