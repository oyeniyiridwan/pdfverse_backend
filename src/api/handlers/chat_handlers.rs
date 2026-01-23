use crate::{
    app::state::ConcreteAppState,
    domain::auth::claim::Claim,
    dtos::{
        chat_info::ChatInfoResponse,
        chat_message::{ChatMessageResponse, RequestChatMessage},
    },
    services::chat_service::ChatService,
    utils::error::ApiError,
};
use axum::{
    Extension, Json,
    extract::{Multipart, Path, State},
};

pub async fn upload_file(
    Extension(claim): Extension<Claim>,
    State(state): State<ConcreteAppState>,
    multipart: Multipart,
) -> Result<Json<ChatInfoResponse>, ApiError> {
    let user_id = claim.sub;
    println!("got here uploading");
    let chat_info = state.chat_service.upload_file(user_id, multipart).await?;
    Ok(Json(ChatInfoResponse::from(chat_info)))
}

pub async fn all_chat_info(
    Extension(claim): Extension<Claim>,
    State(state): State<ConcreteAppState>,
) -> Result<Json<Vec<ChatInfoResponse>>, ApiError> {
    let user_id = claim.sub;
    println!("got here fetching all chat");
    let all_chats = state
        .chat_service
        .get_all_chat_infos(user_id)
        .await?
        .into_iter()
        .map(|k| ChatInfoResponse::from(k))
        .collect::<Vec<ChatInfoResponse>>();
    Ok(Json(all_chats))
}

pub async fn all_chat_messages_by_chat_info_id(
    Path(chat_info_id): Path<String>,
    State(state): State<ConcreteAppState>,
) -> Result<Json<Vec<ChatMessageResponse>>, ApiError> {
    println!("got here fetching all chat messages");
    let all_messages = state
        .chat_service
        .get_all_messages_by_chat_info_id(chat_info_id)
        .await?
        .into_iter()
        .map(|k| ChatMessageResponse::from(k))
        .collect::<Vec<ChatMessageResponse>>();
    Ok(Json(all_messages))
}

pub async fn ask_questions(
    State(state): State<ConcreteAppState>,
    chat: RequestChatMessage,
) -> Result<Json<ChatMessageResponse>, ApiError> {
    let chat_response = state
        .chat_service
        .prompt_and_get_message(chat.chat_id, &chat.text)
        .await?;
    Ok(Json(ChatMessageResponse::from(chat_response)))
}
