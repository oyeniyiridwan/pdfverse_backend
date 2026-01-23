use super::handlers::{
    task_handlers::{combine_atomic_soft_delete_task, create_task, get_all_task, get_one_task},
    user_handlers::{login_with_email_password, magic_link},
};
use crate::{
    api::handlers::{
        auth_handlers::{auth, auth_user, email_verification, handle_auth_callback, refresh_token},
        chat_handlers::{all_chat_info, all_chat_messages_by_chat_info_id, ask_questions, upload_file},
        task_handlers::{atomic_update, partial_update},
        user_handlers::{get_user, signup_with_email_password},
    },
    app::{settings::Settings, state::ConcreteAppState},
    domain::auth::auth_middleware, utils::constant::MAX_PDF_SIZE,
};
use axum::{
    Router, extract::DefaultBodyLimit, http::{HeaderName, Method}, middleware, routing::{delete, get, patch, post, put}
};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use tower_http::cors::CorsLayer;

pub fn create_routes(app_state: ConcreteAppState) -> Router {
    let settings = Settings::from_env();

    let cors = CorsLayer::new()
        .allow_origin(settings.app_url.parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            CONTENT_TYPE,
            ACCEPT,
            AUTHORIZATION,
            HeaderName::from_static("x-platform"),
        ])
        .allow_credentials(true);

    Router::new()
        /************************************* Task Starts *********************************/
        .route("/create_task", post(create_task))
        .route("/tasks/{id}", get(get_one_task))
        .route("/tasks", get(get_all_task))
        .route("/update_task/{id}", put(atomic_update))
        .route("/partial_update_task/{id}", patch(partial_update))
        .route(
            "/combine_atomic_soft_delete_task/{id}",
            delete(combine_atomic_soft_delete_task),
        )
        /************************************* Task Ends *********************************/
        
        /************************************* AI Assistant Starts *********************************/
        .route("/upload_file", post(upload_file))
        .route("/all_chats", get(all_chat_info))
        .route("/messages/{chat_info_id}", get(all_chat_messages_by_chat_info_id))
        .route("/ask_question", post(ask_questions))

        /************************************* AI Assistant Ends *********************************/
        
        /************************************* Authenticated User Starts *********************************/

                .route("/user", get(get_user))
        /************************************* Authenticated User Ends *********************************/

        .route_layer(middleware::from_fn(auth_middleware))
        .route("/users/signup", post(signup_with_email_password))
        .route("/users/login", post(login_with_email_password))
        .route("/auth/magiclink", post(magic_link))
        .route("/auth/{provider}", get(auth))
        .route("/auth/{provider}/callback", get(handle_auth_callback))
        .route("/auth/{provider}/user", get(auth_user))
        .route("/verify_email", get(email_verification))
        .route("/refresh_token", post(refresh_token))
        .layer(cors)
        .layer(DefaultBodyLimit::max( MAX_PDF_SIZE))

        .with_state(app_state)
}
