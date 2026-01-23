use crate::domain::auth::verify_password;
use crate::domain::auth::provider::Provider;
use crate::domain::auth::utils::decide_redirect_link;
use crate::utils::functions::basic::extract_names;
use crate::utils::{error::ApiError};
use crate::{
    app::{cache::AppCache, settings::Settings},
    domain::{ provider::Provider as TableProvider, user::User},
    dtos::auth_dto::AuthClaims,
    repository::{auth_repository::AuthRepository, provider_repository::ProviderRepository},
    services::{infrastructure_services::EmailService, user_service::UserService},
    utils::{config::OAUTHConfig},
};
use axum::response::Redirect;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

pub trait AuthService: Send + Sync {
    async fn get_authorization_url(&self, provider: Provider) -> Result<Redirect, ApiError>;
    async fn handle_auth_callback(
        &self,
        map: HashMap<String, String>,
        provider: Provider,
    ) -> Result<Redirect, ApiError>;
    async fn create_or_get_user_via_token(
        &self,
        provider: Provider,
        token: Option<String>,
    ) -> Result<User, ApiError>;
    async fn magic_link(&self, email: String) -> Result<String, ApiError>;
    async fn login_with_email_password(
        &self,
        email: String,
        password: String,
    ) -> Result<User, ApiError>;
    async fn signup_with_email_password(
        &self,
        email: String,
        password: String,
    ) -> Result<String, ApiError>;

    async fn email_verification(&self, token: Option<String>) -> Result<String, ApiError>;
}

pub struct AuthServiceImpl<P, U, A>
where
    A: AuthRepository + Send + Sync,
    P: ProviderRepository + Send + Sync,
    U: UserService + Send + Sync,
{
    pub provider_repo: Arc<P>,
    pub email_service: EmailService,
    pub user_service: Arc<U>,
    pub auth_repo: Arc<A>,
    pub app_cache: AppCache,
}

impl<P, U, A> AuthServiceImpl<P, U, A>
where
    P: ProviderRepository + Send + Sync,
    U: UserService + Send + Sync,
    A: AuthRepository + Send + Sync,
{
    async fn _exchange_code_token(
        &self,
        code: &str,
        provider: &Provider,
    ) -> Result<String, ApiError> {
        let oauth_config = OAUTHConfig::for_provider(&provider)?;
        let client = reqwest::Client::new();
        let params = [
            ("code", code),
            ("client_id", &oauth_config.client_id),
            ("client_secret", &oauth_config.client_secret),
            ("redirect_uri", &oauth_config.redirect_uri),
            ("grant_type", "authorization_code"),
        ];
        let response = client
            .post(oauth_config.token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| ApiError::RequestTimeout(format!("Error sending request: {}", e)))?;

        if !response.status().is_success() {
            let _ = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(ApiError::ExpectationFailed(format!(
                " token exchange failed: {}",
                body
            )));
        }

        let json: Value = response.json().await.map_err(|e| {
            ApiError::internal_msg(format!("Failed to parse Google token JSON: {e}"))
        })?;

        println!("data {:?}", &json);
        let id_token = json
            .get("id_token")
            .and_then(|v| v.as_str())
            .ok_or(ApiError::NotFound("Missing id_token".to_string()))?;

        Ok(id_token.to_string())
    }

    async fn _extract_auth_claims_from_token(
        &self,
        app_cache: AppCache,
        token: &str,
        provider: &Provider,
    ) -> Result<AuthClaims, ApiError> {
        let shared_data = match provider {
            Provider::Google => &app_cache.google,
            Provider::LinkedIn => &app_cache.linkedin,
            _ => {
                return Err(ApiError::Forbidden(
                    "provider/path does not exist".to_string(),
                ));
            }
        };
        let cached = shared_data.read().await;

        let (cached_value, should_refresh) = match &*cached {
            Some((a, b)) => (Some(a.clone()), b.elapsed() > Duration::from_secs(5)),
            None => (None, true),
        };

        let keys = if should_refresh {
            drop(cached);

            let client = reqwest::Client::new();
            let url = OAUTHConfig::jwks_url(provider)?;
            let response = client.get(url).send().await?;
            if !response.status().is_success() {
                return Err(ApiError::ExpectationFailed(
                    "unsuccessful cert fetch".to_string(),
                ));
            }
            let jwks: Value = response.json().await?;

            tokio::spawn({
                let data = Arc::clone(shared_data);
                let copy_jwks = jwks.clone();
                async move {
                    let mut cached = data.write().await;
                  
                    *cached = Some((copy_jwks, Instant::now() + Duration::from_secs(5 * 60 * 60)));
                }
            });
            jwks
        } else {
            cached_value.ok_or(ApiError::internal_msg("cached value not".to_string()))?
        };

        let header = decode_header(token)?;

        let kid = header
            .kid
            .ok_or("")
            .map_err(|e| ApiError::ExpectationFailed(format!("error: {e}")))?;
        let key = keys["keys"]
            .as_array()
            .ok_or(ApiError::ExpectationFailed(format!("failed")))?
            .iter()
            .find(|f| f["kid"] == kid)
            .ok_or(ApiError::ExpectationFailed("no valid keys".to_string()))?;

        let n = key["n"]
            .as_str()
            .ok_or("Missing n")
            .map_err(|e| ApiError::ExpectationFailed(format!("error: {e}")))?;

        let e = key["e"]
            .as_str()
            .ok_or("Missing e")
            .map_err(|e| ApiError::ExpectationFailed(format!("error: {e}")))?;
        let settings = Settings::from_env();
        let client_id = match *provider {
            Provider::Google => settings.google_client_id,
            Provider::LinkedIn => settings.linkedin_client_id,
            _ => {
                return Err(ApiError::Forbidden(
                    "provider/path does not exist".to_string(),
                ));
            }
        };
        let decoding_key = DecodingKey::from_rsa_components(n, e)
            .map_err(|e| ApiError::internal_msg(format!("error decoding: {e}")))?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[client_id]);

        let token_data = decode::<AuthClaims>(token, &decoding_key, &validation)
            .map_err(|e| ApiError::internal_msg(format!("error decoding: {e}")))?;

        Ok(token_data.claims)
    }

    async fn _email_from_token_normal_auth(
        &self,
        provider: &Provider,
        token: &str,
    ) -> Result<String, ApiError> {
        match provider {
            // Provider::Signup | Provider::Login |
            Provider::MagicLink => {}
            _ => {
                return Err(ApiError::Forbidden(
                    "provider/path does not exist".to_string(),
                ));
            }
        }
        let email = self
            .auth_repo
            .read_email_from_redis(provider, token)
            .await?;
        Ok(email)
    }
}

// #[async_trait]
impl<P, U, A> AuthService for AuthServiceImpl<P, U, A>
where
    P: ProviderRepository + Send + Sync,
    U: UserService + Send + Sync,
    A: AuthRepository + Send + Sync,
{
    async fn get_authorization_url(&self, provider: Provider) -> Result<Redirect, ApiError> {
        let (scope, state) = match provider {
            Provider::Google => ("email profile", "random_state"),
            Provider::LinkedIn => ("openid profile email", "random_state"),
            _ => {
                return Err(ApiError::Forbidden(
                    "provider/path does not exist".to_string(),
                ));
            }
        };
        let oauth_config = OAUTHConfig::for_provider(&provider)?;
        let auth_url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}",
            oauth_config.auth_base_url,
            oauth_config.client_id,
            oauth_config.redirect_uri,
            scope,
            state
        );

        println!("Redirect URL: {}", &auth_url);
        Ok(Redirect::to(&auth_url))
    }

    async fn handle_auth_callback(
        &self,
        map: HashMap<String, String>,
        provider: Provider,
    ) -> Result<Redirect, ApiError> {
        let code = map
            .get("code")
            .ok_or(ApiError::BadRequest("code not found".to_string()))?;
        let token = self._exchange_code_token(code, &provider).await?;
        let app_url = Settings::from_env().app_url;
        let app_url = format!(
            "{}?provider={}&token={}",
            app_url,
            provider.to_string(),
            token
        );
        Ok(Redirect::to(&app_url))
    }

    async fn create_or_get_user_via_token(
        &self,
        provider: Provider,
        token: Option<String>,
    ) -> Result<User, ApiError> {
        let token = match token {
            Some(token) => token,
            None => {
                return Err(ApiError::Forbidden("Invalid/Expired token".to_string()));
            }
        };
        let (sub_id, email, first_name, last_name) = match provider {
            Provider::Google | Provider::LinkedIn => {
                let claim = self
                    ._extract_auth_claims_from_token(self.app_cache.clone(), &token, &provider)
                    .await?;
                let names = extract_names(&claim.name);
                (Some(claim.sub), claim.email, names.0, names.1)
            }
            _ => {
                let email = self
                    ._email_from_token_normal_auth(&provider, &token)
                    .await?;
                (None, Some(email), None, None)
            }
        };

        let (user_related, provider_related) = match (&email, &sub_id) {
            (None, None) => {
                return Err(ApiError::ExpectationFailed("Failed auth".to_string()));
            }
            (None, Some(_)) => (false, true),
            (Some(_), None) => (true, false),
            (Some(_), Some(_)) => (true, true),
        };
        let existing_provider = match provider_related {
            true => {
                self.provider_repo
                    .find_by_external_id(&sub_id.clone().unwrap())
                    .await?
            }
            false => None,
        };
        let user = match user_related {
            true => {
                self.user_service
                    .get_user_by_email(email.clone().unwrap())
                    .await?
            }
            false => match &existing_provider {
                Some(p) => Some(self.user_service.get_user_by_id(&p.user_id).await?),
                None => None,
            },
        };

        let create_provider = match &existing_provider {
            Some(_) => false,
            None => provider_related && true,
        };
        let main_user = match user {
            Some(user) => user,
            None => {
                self.user_service
                    .register_user(email.clone(), None, first_name, last_name, &provider)
                    .await?
            }
        };
        if create_provider {
            let main_provider =
                TableProvider::new(sub_id.unwrap(), email, &main_user.id, provider.to_string());
            self.provider_repo.create_provider(&main_provider).await?;
        }

        Ok(main_user)
    }

    async fn magic_link(&self, email: String) -> Result<String, ApiError> {
        let provider = &Provider::MagicLink;
        let (user, new_user) = match self.user_service.get_user_by_email(email.clone()).await? {
            Some(user) => (user, false),
            None => (
                self.user_service
                    .register_user(Some(email.clone()), None, None, None, provider)
                    .await?,
                true,
            ),
        };
        let token = self
            .auth_repo
            .write_to_redis_and_get_token(provider, &email)
            .await?;

        let url_in_mail: String = decide_redirect_link(&token, provider)?;

        self.email_service.send_verification_or_access_link(
            &email,
            new_user,
            user.first_name,
            url_in_mail,
        );
        Ok("Kindly check your mail for the access Link ".to_string())
    }

    async fn login_with_email_password(
        &self,
        email: String,
        password: String,
    ) -> Result<User, ApiError> {
        let user = match self.user_service.get_user_by_email(email.clone()).await? {
            Some(user) => user,
            None => {
                return Err(ApiError::Unauthorized(
                    "Invalid email or password".to_string(),
                ));
            }
        };
        match &user.password {
            Some(main_password) => {
                if !verify_password(main_password, &password)? {
                    return Err(ApiError::Unauthorized(
                        "Invalid email or password".to_string(),
                    ));
                }
            }
            None => {
                return Err(ApiError::Unauthorized(
                    "Invalid email or password".to_string(),
                ));
            }
        };
        if !user.email_verified {
            let provider = Provider::Signup;
            let token = self
                .auth_repo
                .write_to_redis_and_get_token(&provider, &email)
                .await?;

            let url_in_mail: String = decide_redirect_link(&token, &provider)?;

            self.email_service.send_verification_or_access_link(
                &email,
                false,
                user.first_name.clone(),
                url_in_mail,
            );
            return Err(ApiError::EmailSuccess(
                "Email verification to access app has been sent to your email".to_string(),
            ));
        }
        Ok(user)
    }

    async fn signup_with_email_password(
        &self,
        email: String,
        password: String,
    ) -> Result<String, ApiError> {
        let possible_user = self.user_service.get_user_by_email(email.clone()).await?;
        let provider = &Provider::Signup;
        match possible_user {
            Some(_) => {
                return Err(ApiError::Forbidden(
                    "user already exist, proceed to login".to_string(),
                ));
            }
            None => {
                let new_user = self
                    .user_service
                    .register_user(Some(email.clone()), Some(password), None, None, provider)
                    .await?;
                println!("verified:{}", &new_user.email_verified);
                let token = self
                    .auth_repo
                    .write_to_redis_and_get_token(provider, &email)
                    .await?;

                let url_in_mail: String = decide_redirect_link(&token, provider)?;

                self.email_service.send_verification_or_access_link(
                    &email,
                    true,
                    new_user.first_name,
                    url_in_mail,
                );
                return Ok(
                    "Email verification to access app has been sent to your email".to_string(),
                );
            }
        };
    }

    async fn email_verification(&self, token: Option<String>) -> Result<String, ApiError> {
        match token {
            Some(token) => {
                let provider = &Provider::Signup;
                let email = match self.auth_repo.read_email_from_redis(provider, &token).await {
                    Ok(email) => email,
                    Err(_) => {
                        return Err(ApiError::ExpectationFailed(
                            "Invalid/Expired Token".to_string(),
                        ));
                    }
                };
                let _ = &self
                    .user_service
                    .update_user_by_email(&email, None, None, Some(true), None)
                    .await?;
                Ok("Email verified Successfully".to_string())
            }
            None => Err(ApiError::Forbidden("Invalid/Expired token".to_string())),
        }
    }
}
