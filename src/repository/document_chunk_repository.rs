use lettre::transport::smtp::commands::Data;
use migration::CaseStatement;
// use sea_orm::sea_query::{Expr, Query, Alias};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection,
    DatabaseTransaction, EntityTrait, QueryFilter,
    sea_query::expr::Expr
};

use uuid::Uuid;

use crate::{
    database::document_chunk::{ActiveModel, Column, Entity},
    domain::document::DocumentChunk,
    utils::{error::ApiError, functions::basic::option_string_from_option_pg_vector},
};

pub trait DocumentChunkRepository: Send + Sync {
    async fn create_document_chunk(
        &self,
        document_chunk: DocumentChunk,
    ) -> Result<DocumentChunk, ApiError>;
    async fn find_document_chunk_by_id_and_chat_id(
        &self,
        chat_id: &Uuid,
        id: &Uuid,
    ) -> Result<Option<DocumentChunk>, ApiError>;
    async fn find_all_document_chunks_by_chat_id(
        &self,
        chat_id: &Uuid,
    ) -> Result<Vec<DocumentChunk>, ApiError>;
    async fn insert_document_chunk_list(
        &self,
        list_document_chunks: Vec<DocumentChunk>,
    ) -> Result<(), ApiError>;

     fn insert_document_chunk_list_via_db_transaction(
        &self,
        db: &DatabaseTransaction,
        list_document_chunks: Vec<DocumentChunk>,
    )     -> impl Future<Output = Result<Vec<DocumentChunk>, ApiError>> + Send; // Add + Send here

    fn batch_update_of_list_chunks(
        &self,
        list_document_chunks: Vec<DocumentChunk>,
    )     -> impl Future<Output = Result<(), ApiError>> + Send;

}

#[derive(Clone)]
pub struct DocumentChunkRepositoryImpl {
    db: DatabaseConnection,
}

impl DocumentChunkRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
impl DocumentChunkRepository for DocumentChunkRepositoryImpl {
    async fn create_document_chunk(
        &self,
        document_chunk: DocumentChunk,
    ) -> Result<DocumentChunk, ApiError> {
        let active_document_chunk = ActiveModel {
            chat_id: Set(document_chunk.chat_id),
            content: Set(document_chunk.content),
            embedding: Set(
                // option_string_from_option_pg_vector(
                document_chunk.embedding,
            // )
        ),
            ..Default::default()
        };
        let document_chunk_model = active_document_chunk
            .insert(&self.db)
            .await
            .map_err(|e| ApiError::internal_msg(format!("DocumentChunkRepository-create_document: {e}")))?;
        Ok(DocumentChunk::from(document_chunk_model))
    }

    async fn find_document_chunk_by_id_and_chat_id(
        &self,
        chat_id: &Uuid,
        id: &Uuid,
    ) -> Result<Option<DocumentChunk>, ApiError> {
        let condition = Condition::all()
            .add(Column::ChatId.eq(*chat_id))
            .add(Column::Id.eq(*id));
        let possible_document_model = Entity::find()
            .filter(condition)
            .one(&self.db)
            .await
            .map_err(|e| ApiError::internal_msg(format!("DocumentChunkRepository-find_document: {e}")))?;
        Ok(possible_document_model.map(DocumentChunk::from))
    }

    async fn insert_document_chunk_list(
        &self,
        list_document_chunks: Vec<DocumentChunk>,
    ) -> Result<(), ApiError> {
        let list_of_active_document_chunks = list_document_chunks
            .iter()
            .enumerate()
            .map(|(index, document_chunk)| ActiveModel {
                chat_id: Set(document_chunk.chat_id),
                content: Set(document_chunk.content.clone()),
                embedding: Set(
                    // option_string_from_option_pg_vector(
                    document_chunk.embedding.clone(),
                // )
            ),
                chunk_index: Set(index as i32),
                ..Default::default()
            })
            .collect::<Vec<ActiveModel>>();
        let _ = Entity::insert_many(list_of_active_document_chunks)
            // .exec_with_returning_many
            .exec(&self.db)
            .await
            .map_err(|e| ApiError::internal_msg(format!("DocumentChunkRepository-insert_document-db_transaction: {e}")))?;

       
        Ok(())
    }

    async fn insert_document_chunk_list_via_db_transaction(
        &self,
        db: &DatabaseTransaction,

        list_document_chunks: Vec<DocumentChunk>,
    ) -> Result<Vec<DocumentChunk>, ApiError> {
        let list_of_active_document_chunks = list_document_chunks
            .iter()
            .enumerate()
            .map(|(index, document_chunk)| ActiveModel {
                chat_id: Set(document_chunk.chat_id),
                content: Set(document_chunk.content.clone()),
                embedding: Set(
                    // option_string_from_option_pg_vector(
                    document_chunk.embedding.clone(),
                // )
            ),
                chunk_index: Set(index as i32),
                ..Default::default()
            })
            .collect::<Vec<ActiveModel>>();
        let result = Entity::insert_many(list_of_active_document_chunks)
            .exec_with_returning_many(db)
            // .exec(db)
            .await
            .map_err(|e| ApiError::internal_msg(format!("DocumentChunkRepository-insert_document: {e}")))?
            .into_iter().map(|k|
                 DocumentChunk::from(k)).collect::<Vec<DocumentChunk>>();

        // let models =     document_chunk_models.iter().map(|k| DocumentChunk::from(k.clone()) ).collect::<Vec<DocumentChunk>>();
        //     todo!()
        Ok(result)
    }
    





    async fn batch_update_of_list_chunks(
    &self,
    list_document_chunks: Vec<DocumentChunk>,
) -> Result<(), ApiError> {
    if list_document_chunks.is_empty() {
        return Ok(());
    }

let mut case_expr =CaseStatement::new();
    for chunk in list_document_chunks.iter() {
        if let Some(id) = chunk.id {
            case_expr = case_expr.case(
                Column::Id.eq(id),  // WHEN id = ?
                Expr::value(chunk.embedding.clone())  // THEN embedding
            );
        }
    }
    let  case_expr = case_expr.into();

    let ids = list_document_chunks.iter()
        .filter_map(|chunk| chunk.id)
        .collect::<Vec<_>>();

    Entity::update_many()
        .col_expr(Column::Embedding, case_expr)
        .filter(Column::Id.is_in(ids))
        .exec(&self.db)
        .await
        .map_err(|e| ApiError::internal_msg(format!("batch update failed: {e}")))?;
    
    Ok(())
}

    async fn find_all_document_chunks_by_chat_id(
        &self,
        chat_id: &Uuid,
    ) -> Result<Vec<DocumentChunk>, ApiError> {
         let all_chat_document_models = Entity::find()
            .filter(Column::ChatId.eq(*chat_id))
            .all(&self.db)
            .await
            .map_err(|e| ApiError::internal_msg(format!("DocumentChunkRepository-finding all document: {e}")))?;
        Ok(all_chat_document_models.into_iter().map(|e|DocumentChunk::from(e)).collect())
      
    }


  



}


