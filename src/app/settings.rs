use std::env::var;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database_url: String,
    pub redis_url: String,
    pub redirect_url: String,
    pub app_url: String,
    pub google_client_id: String,
    pub google_secret: String,
    pub secret: String,

    pub mail_password: String,
    pub linkedin_client_id: String,
    pub linkedin_secret: String,
    pub smtp_url: String,
    pub port: u16,
    pub mail_address: String,
}

impl Settings {
    pub fn from_env() -> Self {
        if cfg!(debug_assertions) {
            dotenvy::from_filename(".env").ok();
        } else {
            dotenvy::from_filename(".env_prod").ok();
        }
        Self {
            database_url: var("DATABASE_URL").expect("DATABASE_URL not set"),
            redis_url: var("REDIS_URL").expect("REDIS_URL not set"),
            smtp_url: var("SMTP_URL").expect("SMTP_URL not set"),
            redirect_url: var("REDIRECT_URL").expect("REDIRECT_URL not set"),
            secret: var("JWT_SECRET").unwrap_or("iya_mi".to_string()),
            app_url: var("APP_URL").expect("APP_URL not set"),
            google_client_id: var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set"),
            google_secret: var("GOOGLE_SECRET").expect("GOOGLE_SECRET not set"),
            linkedin_client_id: var("LINKEDIN_CLIENT_ID").expect("LINKEDIN_CLIENT_ID not set"),
            linkedin_secret: var("LINKEDIN_SECRET").expect("LINKEDIN_SECRET not set"),
            mail_password: var("MAIL_PASSWORD").expect("MAIL_PASSWORD not set"),
            mail_address: var("MAIL_ADDRESS").expect("MAIL_ADDRESS not set"),
            port: var("PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()
                .expect("Invalid PORT"),
        }
    }
}
