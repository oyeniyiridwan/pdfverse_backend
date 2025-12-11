use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add the column
        manager
            .alter_table(
                Table::alter()
                    .table(Providers::Table)
                    .add_column(
                        ColumnDef::new(Providers::Email)
                            .string_len(128)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Add unique index on the email column
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_providers_email_unique")
                    .table(Providers::Table)
                    .col(Providers::Email)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop index first
        manager
            .drop_index(
                Index::drop()
                    .name("idx_providers_email_unique")
                    .table(Providers::Table)
                    .to_owned(),
            )
            .await?;

        // Drop the column
        manager
            .alter_table(
                Table::alter()
                    .table(Providers::Table)
                    .drop_column(Providers::Email)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

// Add to Providers enum
#[derive(Iden)]
enum Providers {
    Table,
    Id,
    ProviderName,
    ExternalId,
    AccessToken,
    RefreshToken,
    ExpiresAt,
    UserId,
    CreatedAt,
    UpdatedAt,
    Email, // ← new column
}
