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
    pub jwt_secret: String,
    pub mail_password: String,
    pub linkedin_client_id: String,
    pub linkedin_secret: String,
    pub smtp_url: String,
    pub port: u16,
    pub mail_address: String,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub s3_region: String,
    pub openai_embedding_url: String,
    pub openai_messaging_url: String,

    pub openai_key: String,
    pub openai_model: String,
        pub openai_response_model: String,

}

impl Settings {
    pub fn from_env() -> Self {
        Self {
            database_url: var("DATABASE_URL").expect("DATABASE_URL not set"),
            redis_url: var("REDIS_URL").expect("REDIS_URL not set"),
            smtp_url: var("SMTP_URL").expect("SMTP_URL not set"),
            redirect_url: var("REDIRECT_URL").expect("REDIRECT_URL not set"),
            jwt_secret: var("JWT_SECRET").expect("JWT_SECRET not set"),
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
            s3_endpoint: var("S3_ENDPOINT").expect("S3_ENDPOINT not set"),
            s3_bucket: var("S3_BUCKET").expect("S3_BUCKET not set"),
            s3_access_key: var("S3_ACCESS_KEY").expect("S3_ACCESS_KEY not set"),
            s3_secret_key: var("S3_SECRET_KEY").expect("S3_SECRET_KEY not set"),
            s3_region: var("S3_REGION").expect("S3_REGION not set"),
            openai_key: var("OPENAI_KEY").expect("OPENAI_KEY not set"),
            openai_model: var("OPENAI_MODEL").expect("OPENAI_MODEL not set"),
            openai_embedding_url: var("OPENAI_EMBEDDING_URL")
                .expect("OPENAI_EMBEDDING_URL not set"),
            openai_messaging_url: var("OPENAI_MESSAGING_URL")
                .expect("OPENAI_MESSAGING_URL not set"),
                   openai_response_model: var("OPENAI_RESPONSE_MODEL")
                .expect("OPENAI_RESPONSE_MODEL not set"),

        }
    }
}
