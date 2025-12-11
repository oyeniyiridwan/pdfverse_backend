use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1️⃣ Drop foreign keys
        manager
            .drop_foreign_key(ForeignKey::drop().name("fk_providers_user_id").table(Providers::Table).to_owned())
            .await?;
        manager
            .drop_foreign_key(ForeignKey::drop().name("fk_tasks_user_id").table(Tasks::Table).to_owned())
            .await?;

        // 2️⃣ Drop old users.id and recreate as UUID
      
          manager
            .alter_table(Table::alter().table(Providers::Table).drop_column(Providers::UserId).to_owned())
            .await?;

         manager
            .alter_table(Table::alter().table(Tasks::Table).drop_column(Tasks::UserId).to_owned())
            .await?;
          manager
            .alter_table(Table::alter().table(Users::Table).drop_column(Users::Id).to_owned())
            .await?;

       

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Optional: revert UUIDs back to integers if needed
        Ok(())
    }
}


#[derive(DeriveIden)]
enum Users {
     Table,
    Id,
    FirstName,
    LastName,
    Email,
    Password,
    DeletedAt,
    Token,
    EmailVerified
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

#[derive(Iden)]
enum Tasks {
    Table,
    Id,
    Priority,
    Title,
    CompletedAt,
    Description,
    DeletedAt,
    UserId,
    IsDefault,
}
