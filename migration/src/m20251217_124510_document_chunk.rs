use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // --- Enable pgvector extension ---
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS vector;")
            .await?;

        // --- document_chunk table ---
        manager
            .create_table(
                Table::create()
                    .table(DocumentChunk::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DocumentChunk::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(DocumentChunk::ChatId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DocumentChunk::ChunkIndex)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DocumentChunk::Content)
                            .text()
                            .not_null(),
                    )
                    // pgvector column (custom SQL)
                    .col(
                        ColumnDef::new(DocumentChunk::Embedding)
                            .custom(Alias::new("vector(1536)")),
                    )
                    .col(
                        ColumnDef::new(DocumentChunk::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_document_chunk_chat_id")
                            .from(DocumentChunk::Table, DocumentChunk::ChatId)
                            .to(ChatInfo::Table, ChatInfo::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // --- Indexes ---

        // Fast lookup by chat
        manager
            .create_index(
                Index::create()
                    .name("idx_document_chunk_chat_id")
                    .table(DocumentChunk::Table)
                    .col(DocumentChunk::ChatId)
                    .to_owned(),
            )
            .await?;

        // Ensure unique chunk ordering per chat
        manager
            .create_index(
                Index::create()
                    .name("idx_document_chunk_chat_chunk")
                    .table(DocumentChunk::Table)
                    .col(DocumentChunk::ChatId)
                    .col(DocumentChunk::ChunkIndex)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_document_chunk_chat_chunk")
                    .table(DocumentChunk::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_document_chunk_chat_id")
                    .table(DocumentChunk::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(DocumentChunk::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}



#[derive(Iden)]
enum DocumentChunk {
    Table,
    Id,
    ChatId,
    ChunkIndex,
    Content,
    Embedding,
    CreatedAt,
}


#[derive(DeriveIden)]
enum ChatInfo {
    Table,
    Id,
    UserId,
    Title,
    Path,
    CreatedAt,
}