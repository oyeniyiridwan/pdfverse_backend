use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the old unique index on email
        manager
            .drop_index(
                Index::drop()
                    .name("idx_providers_email_unique")
                    .table(Providers::Table)
                    .to_owned(),
            )
            .await?;

        // Create a new unique index on provider_user_id
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_providers_provider_user_id_unique")
                    .table(Providers::Table)
                    .col(Providers::ExternalId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the unique index on provider_user_id
        manager
            .drop_index(
                Index::drop()
                    .name("idx_providers_provider_user_id_unique")
                    .table(Providers::Table)
                    .to_owned(),
            )
            .await?;

        // Recreate the unique index on email
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
}

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
    Email,
}
