use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1️⃣ Drop index
        manager
            .drop_index(
                Index::drop()
                    .name("idx_chat_message_chat_info_id_timestamp")
                    .table(ChatMessage::Table)
                    .to_owned(),
            )
            .await?;

        // 2️⃣ Alter timestamp: nullable + auto-generated
        manager
            .alter_table(
                Table::alter()
                    .table(ChatMessage::Table)
                    .modify_column(
                        ColumnDef::new(ChatMessage::Timestamp)
                            .timestamp_with_time_zone()
                            .null()
                            .default(Expr::current_timestamp()), // ✅ auto-generated
                    )
                    .to_owned(),
            )
            .await?;

        // 3️⃣ Recreate index
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
        // 1️⃣ Drop index
        manager
            .drop_index(
                Index::drop()
                    .name("idx_chat_message_chat_info_id_timestamp")
                    .table(ChatMessage::Table)
                    .to_owned(),
            )
            .await?;

        // 2️⃣ Backfill NULLs before reverting
        manager
            .get_connection()
            .execute_unprepared(
                "UPDATE chat_message SET timestamp = NOW() WHERE timestamp IS NULL",
            )
            .await?;

        // 3️⃣ Remove default + restore NOT NULL
        manager
            .alter_table(
                Table::alter()
                    .table(ChatMessage::Table)
                    .modify_column(
                        ColumnDef::new(ChatMessage::Timestamp)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 4️⃣ Recreate index
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
}

#[derive(Iden)]
enum ChatMessage {
    Table,
    ChatInfoId,
    Timestamp,
}
