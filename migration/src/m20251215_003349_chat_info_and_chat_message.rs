use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ---------- chat_info table ----------
        manager
            .create_table(
                Table::create()
                    .table(ChatInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ChatInfo::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(ChatInfo::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ChatInfo::Title)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ChatInfo::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chat_info_user_id")
                            .from(ChatInfo::Table, ChatInfo::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Add index on user_id for fast user chat queries
        manager
            .create_index(
                Index::create()
                    .name("idx_chat_info_user_id")
                    .table(ChatInfo::Table)
                    .col(ChatInfo::UserId)
                    .to_owned(),
            )
            .await?;

        // ---------- chat_message table ----------
        manager
            .create_table(
                Table::create()
                    .table(ChatMessage::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ChatMessage::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(ChatMessage::ChatInfoId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ChatMessage::Role)
                            .string_len(16)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ChatMessage::Text)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ChatMessage::Timestamp)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chat_message_chat_info_id")
                            .from(ChatMessage::Table, ChatMessage::ChatInfoId)
                            .to(ChatInfo::Table, ChatInfo::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Add composite index on (chat_info_id, timestamp) for fast message retrieval
        manager
            .create_index(
                Index::create()
                    .name("idx_chat_message_chat_info_id_timestamp")
                    .table(ChatMessage::Table)
                    .col(ChatMessage::ChatInfoId)
                    .col(ChatMessage::Timestamp)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_chat_message_chat_info_id_timestamp")
                    .table(ChatMessage::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_chat_info_user_id")
                    .table(ChatInfo::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ChatMessage::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ChatInfo::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum ChatInfo {
    Table,
    Id,
    UserId,
    Title,
    CreatedAt,
}

#[derive(Iden)]
enum ChatMessage {
    Table,
    Id,
    ChatInfoId,
    Role,
    Text,
    Timestamp,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    Password,
    DeletedAt,
    Token,
    EmailVerified,
}