use std::sync::Arc;

use reqwest::{Client, header::AUTHORIZATION};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::{
    app::settings::Settings,
    domain::{chat_message::ChatMessage, document::DocumentChunk},
    repository::document_chunk_repository::DocumentChunkRepository,
    utils::{constant::BATCH_NO, error::ApiError, functions::basic::cosine_similarity},
};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AIMessageBody {
    pub role: Role,
    pub content: String,
}

impl AIMessageBody {
    pub fn new(role: Role, content: String) -> Self {
        Self { role, content }
    }

    fn default_messaging_body(prompt: String) -> Vec<Self> {
        vec![
            AIMessageBody::new(Role::System, "You are a helpful assistant.".to_string()),
            AIMessageBody::new(Role::User, prompt),
        ]
    }
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<AIMessageBody>,
    pub temperature: f32,
}

impl ChatCompletionRequest {
    pub fn new(context: String, model: &str) -> Self {
        let messages = AIMessageBody::default_messaging_body(context);
        Self {
            model: model.to_string(),
            messages,
            temperature: 0.0,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Clone)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub embedding_url: String,
    pub messaging_url: String,
    pub model: String,
    pub response_model: String,
    pub batch_size: usize,
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        let settings = Settings::from_env();
        Self {
            api_key: settings.openai_key,
            embedding_url: settings.openai_embedding_url,
            messaging_url: settings.openai_messaging_url,
            model: settings.openai_model,
            batch_size: BATCH_NO,
            response_model: settings.openai_response_model,
        }
    }
}

pub trait OpenAIService: Send + Sync {
    fn update_chunks_with_embeddings(&self, chunks: Vec<DocumentChunk>);
    async fn embeddings_for_question(&self, message: &str) -> Result<Vec<f32>, ApiError>;
    fn build_openai_context(
        &self,
        chunks: Vec<DocumentChunk>,
        question_embeddings: Vec<f32>,
    ) -> String;
    async fn retrieve_openai_llm_response_using_context(
        &self,
        context: String,
        id: Uuid,
    ) -> Result<ChatMessage, ApiError>;
}

pub struct OpenAIServiceImpl<D>
where
    D: DocumentChunkRepository + Send + Sync + 'static,
{
    pub document_chunk_repo: Arc<D>,
    pub client: Client,
    pub config: OpenAIConfig,
}

impl<D> OpenAIServiceImpl<D>
where
    D: DocumentChunkRepository + Send + Sync + 'static,
{
    pub fn new(document_chunk_repo: Arc<D>, client: Client, config: OpenAIConfig) -> Self {
        Self {
            document_chunk_repo,
            client,
            config,
        }
    }
}

impl<D> OpenAIService for OpenAIServiceImpl<D>
where
    D: DocumentChunkRepository + Send + Sync + 'static,
{
    fn update_chunks_with_embeddings(&self, mut chunks: Vec<DocumentChunk>) {
        let new_document_chunk_repo: Arc<D> = Arc::clone(&self.document_chunk_repo);
        let client = self.client.clone();
        let config = self.config.clone();
        tokio::spawn(async move {
            match openai_embeddings_for_chunks_v2(&mut chunks, client, config).await {
                Ok(_) => {
                    if let Err(e) = new_document_chunk_repo
                        .batch_update_of_list_chunks(chunks)
                        .await
                    {
                        eprintln!("error batch updating db with embeddings :{e} ");
                    }
                }
                Err(e) => {
                    eprintln!("open ai embeddings fetching fails {e}");
                }
            }
        });
    }

    

    async fn embeddings_for_question(&self, message: &str) -> Result<Vec<f32>, ApiError> {
        let body = json!({
               "model":&self.config.model,
               "input":message
        });
        let resp = &self
            .client
            .post(&self.config.embedding_url)
            .header(AUTHORIZATION, format!("Bearer {}", &self.config.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| ApiError::internal_msg(format!("error: {e}")))?
            .json::<Value>()
            .await?;

        let embedding = resp
            .get("data")
            .and_then(|f| f.get(0))
            .and_then(|w| w.get("embedding"))
            .and_then(|e| e.as_array())
            .expect("Invalid Embedding response for_message")
            .iter()
            .map(|v| v.as_f64().unwrap() as f32)
            .collect::<Vec<f32>>();

        Ok(embedding)
    }

    fn build_openai_context(
        &self,
        chunks: Vec<DocumentChunk>,
        question_embeddings: Vec<f32>,
    ) -> String {
        let mut scored_chunks = chunks
            .iter()
            .map(|chunk| {
                let score =
                    cosine_similarity(&question_embeddings, &chunk.clone().embedding.unwrap());
                (score, chunk.clone())
            })
            .collect::<Vec<(f32, DocumentChunk)>>();
        scored_chunks.sort_by(|a, b| b.0.total_cmp(&a.0));

        // let top_chunks = scored_chunks.into_iter().take(5).collect::<Vec<_>>();
        // let context = top_chunks
        //     .into_iter().enumerate()
        //     .map(|(i, c)| format!("Source {}:\n{}", i + 1.0, c.content))
        //     .collect::<Vec<_>>()
        //     .join("\n\n---\n\n");
        let top_chunks = scored_chunks.into_iter().take(5).collect::<Vec<_>>();
let context = top_chunks
    .into_iter()
    .enumerate()                          // ← use enumerate for a real index
    .map(|(i, (score, c))| format!("Source {}:\n{}", i + 1, c.content))
    .collect::<Vec<_>>()
    .join("\n\n---\n\n");

        context
    }

    async fn retrieve_openai_llm_response_using_context(
        &self,
        context: String,
        id: Uuid,
    ) -> Result<ChatMessage, ApiError> {
        let body = ChatCompletionRequest::new(context, &self.config.response_model);
        let resp = &self
            .client
            .post(&self.config.messaging_url)
            .header(AUTHORIZATION, format!("Bearer {}", &self.config.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| ApiError::internal_msg(format!("error: {e}")))?
            .json::<Value>()
            .await?;
        let message = resp
            .get("choices")
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("message"))
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_str())
            .ok_or_else(|| ApiError::internal_msg("failed to extract message"))?;
        // eprint!("open ai response {}", resp);
        let chat: ChatMessage = ChatMessage::new(None, id, false, message.to_string(), None);
        Ok(chat)
    }
}

async fn openai_embeddings_for_chunks_v2(
    chunks: &mut Vec<DocumentChunk>,
    client: Client,
    config: OpenAIConfig,
) -> Result<(), ApiError> {
    for batch in chunks.chunks_mut(config.batch_size) {
        let text = batch.iter().map(|e| e.content.as_str()).collect::<Vec<_>>(); // "Hello world";

        let body = json!({
               "model":config.model,
               "input":text
        });
        let resp = client
            .post(&config.embedding_url)
            .header(AUTHORIZATION, format!("Bearer {}", config.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| ApiError::internal_msg(format!("error: {e}")))?
            .json::<Value>()
            .await?;

        // println!("response for bacth is : {:?}", &resp);
        for (chunks, data) in batch.iter_mut().zip(
            resp.get("data")
                .and_then(|v| v.as_array())
                .ok_or_else(|| ApiError::internal_msg("Invalid response for_chunks".to_string()))?,
        ) {
            let embedding = data
                .get("embedding")
                .and_then(|v| v.as_array())
                .ok_or_else(|| {
                    ApiError::internal_msg("Invalid embedding response for_chunks".to_string())
                })?
                .iter()
                .map(|f| f.as_f64().unwrap() as f32)
                .collect::<Vec<f32>>();
            chunks.update_embedding(embedding);
        }
    }

    Ok(())
}
