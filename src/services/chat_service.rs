use std::sync::Arc;

use axum::extract::Multipart;
use reqwest::{Client, header::AUTHORIZATION};
use sea_orm::{DatabaseConnection, DbErr, TransactionTrait};
use uuid::Uuid;

use crate::{
    domain::{
        chat_info::ChatInfo,
        chat_message::ChatMessage,
        document::{DocumentChunk, pdf::process_pdf},
    },
    repository::{
        chat_info_repository::ChatInfoRepository, chat_message_repository::ChatMessageRepository,
        document_chunk_repository::DocumentChunkRepository,
    },
    services::{infrastructure_services::StorageService, openai_service::OpenAIService},
    utils::{error::ApiError, functions::basic::cosine_similarity},
};

pub trait ChatService: Send + Sync {
    async fn upload_file(&self, user_id: Uuid, multipart: Multipart) -> Result<ChatInfo, ApiError>;
    async fn get_all_chat_infos(&self, user_id: Uuid) -> Result<Vec<ChatInfo>, ApiError>;
    async fn get_all_messages_by_chat_info_id(
        &self,
        chat_info_id: String,
    ) -> Result<Vec<ChatMessage>, ApiError>;
    async fn prompt_and_get_message(
        &self,
        chat_info_id: String,
        message: &str,
    ) -> Result<ChatMessage, ApiError>;
}

pub struct ChatServiceImpl<C, M, D, S, O>
where
    C: ChatInfoRepository + Send + Sync + 'static,
    M: ChatMessageRepository + Send + Sync + 'static,
    D: DocumentChunkRepository + Send + Sync + 'static,
    S: StorageService + Send + Sync,
    O: OpenAIService + Send + Sync,
{
    pub chat_info_repo: Arc<C>,
    pub chat_message_repo: Arc<M>,
    pub document_chunk_repo: Arc<D>,
    pub storage_service: Arc<S>,
    pub openai_service: Arc<O>,
    db: DatabaseConnection,
}

impl<C, M, D, S, O> ChatService for ChatServiceImpl<C, M, D, S, O>
where
    C: ChatInfoRepository + Send + Sync,
    M: ChatMessageRepository + Send + Sync,
    D: DocumentChunkRepository + Send + Sync,
    S: StorageService + Send + Sync,
    O: OpenAIService + Send + Sync,
{
    async fn upload_file(&self, user_id: Uuid, multipart: Multipart) -> Result<ChatInfo, ApiError> {
        let storage_service = Arc::clone(&self.storage_service);
        let chat_info_repo = Arc::clone(&self.chat_info_repo);
        let document_chunk_repo: Arc<D> = Arc::clone(&self.document_chunk_repo);
        let (file_name, file_path, file_bytes) = storage_service.upload_file(multipart).await?;

        let (chat_info, saved_chunks) = self
            .db
            .transaction::<_, (ChatInfo, Vec<DocumentChunk>), ApiError>(|txn| {
                Box::pin(async move {
                    let domain_chat_info = ChatInfo::new(&user_id, &file_name, &file_path);
                    let domain_chat_info = chat_info_repo
                        .create_chat_info_via_db_transaction(txn, domain_chat_info)
                        .await?;
                    let chunks = process_pdf(file_bytes).map_err(|e| {
                        ApiError::internal_msg(format!("failed to chunk pdf error: {e}"))
                    })?;

                    let list_document_chunks = chunks
                        .iter()
                        .map(|e| DocumentChunk::new(domain_chat_info.id.unwrap(), e.clone(), None))
                        .collect::<Vec<DocumentChunk>>();

                    let saved_chunks = document_chunk_repo
                        .insert_document_chunk_list_via_db_transaction(txn, list_document_chunks)
                        .await?;

                    Ok((domain_chat_info, saved_chunks))
                })
            })
            .await
            .map_err(|e| ApiError::internal_msg(format!("Transaction error: {e}")))?;
        self.openai_service
            .update_chunks_with_embeddings(saved_chunks);

        Ok(chat_info)
    }

    async fn get_all_chat_infos(&self, user_id: Uuid) -> Result<Vec<ChatInfo>, ApiError> {
        let result = self
            .chat_info_repo
            .find_all_chat_info_by_user_id(user_id)
            .await?;
        Ok(result)
    }

    async fn get_all_messages_by_chat_info_id(
        &self,
        chat_info_id: String,
    ) -> Result<Vec<ChatMessage>, ApiError> {
        let id = Uuid::parse_str(&chat_info_id)
            .map_err(|e| ApiError::BadRequest(format!("Invalid  request: {e}")))?;
        let result = self
            .chat_message_repo
            .find_all_chat_messages_by_chat_info_id(&id, None)
            .await?;
        Ok(result)
    }

    async fn prompt_and_get_message(
        &self,
        chat_info_id: String,
        question: &str,
    ) -> Result<ChatMessage, ApiError> {
        let id = Uuid::parse_str(&chat_info_id)
            .map_err(|e| ApiError::BadRequest(format!("Invalid  request: {e}")))?;
        let user_chat = ChatMessage::new(None, id, true, question.to_string(), None);

        let question_embeddings = self
            .openai_service
            .embeddings_for_question(question)
            .await?;
        let previous_messages = self
            .chat_message_repo
            .find_all_chat_messages_by_chat_info_id(&id, Some(7))
            .await?
            .into_iter()
            .map(|e| e.text)
            .collect::<Vec<_>>();

        let chunks = self
            .document_chunk_repo
            .find_all_document_chunks_by_chat_id(&id)
            .await?;
        let context = self
            .openai_service
            .build_openai_context(chunks, question_embeddings);

        let prompt = if previous_messages.is_empty() {
            format!(
                "Answer using only the context below.\n\nContext:\n{}\n\nQuestion:\n{}",
                context, question
            )
        } else {
            let chat_history = previous_messages.join("\n");
            format!(
                "Provide a factual answer based solely on the context below and the previous conversation. \
If the answer is not contained in them, say that it is not available.\n\nContext:\n{}\n\nPrevious conversation:\n{}\n\nQuestion:\n{}",
                context, chat_history, question
            )
        };

        let chat = self
            .openai_service
            .retrieve_openai_llm_response_using_context(prompt, id)
            .await?;
        let result = self
            .chat_message_repo
            .insert_multiple_messages(vec![user_chat, chat])
            .await?;
        let response_chat = result
            .into_iter()
            .nth(1)
            .ok_or_else(|| ApiError::internal_msg("error retrieving"))?;
        Ok(response_chat)
    }
}

impl<C, M, D, S, O> ChatServiceImpl<C, M, D, S, O>
where
    C: ChatInfoRepository + Send + Sync,
    M: ChatMessageRepository + Send + Sync,
    D: DocumentChunkRepository + Send + Sync,
    S: StorageService + Send + Sync,
    O: OpenAIService + Send + Sync,
{
    pub fn new(
        chat_info_repo: Arc<C>,
        chat_message_repo: Arc<M>,
        document_chunk_repo: Arc<D>,
        storage_service: Arc<S>,
        embedding_service: Arc<O>,
        db: DatabaseConnection,
    ) -> Self {
        Self {
            chat_info_repo,
            chat_message_repo,
            document_chunk_repo,
            storage_service,
            openai_service: embedding_service,
            db,
        }
    }
}
