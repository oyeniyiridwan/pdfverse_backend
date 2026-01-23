use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

use crate::{database::document_chunk, utils::functions::basic::option_pg_vector_from_option_string};

#[derive(Debug, Clone)]

pub struct DocumentChunk {
    pub id: Option<Uuid>,
    pub chat_id: Uuid,
    pub chunk_index: Option<i32>,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
    pub created_at: Option<DateTimeWithTimeZone>,
}

impl DocumentChunk {
    pub fn update_embedding(&mut self, embedding: Vec<f32>) {
        self.embedding = Some(embedding);
    }
}


impl DocumentChunk {
    pub fn new(
        // id: Option<Uuid>,
        chat_id: Uuid,
        // chunk_index: Option<i32>,
        content: String,
        embedding: Option<Vec<f32>>,
        // created_at: Option<DateTimeWithTimeZone>,
    ) -> Self {
        Self {
            id: None,
            chat_id,
            chunk_index: None,
            content,
            embedding,
            created_at: None,
        }
    }
}

impl From<document_chunk::Model> for DocumentChunk {
    fn from(value: document_chunk::Model) -> Self {
        Self {
            id: Some(value.id),
            chat_id: value.chat_id,
            chunk_index: Some(value.chunk_index),
            content: value.content,
            embedding: value.embedding,//.map(|pg_vec| pg_vec.to_vec()),
            created_at: Some(value.created_at),
        }
    }
}
