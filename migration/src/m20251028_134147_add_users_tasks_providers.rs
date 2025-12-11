use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       // ========================
        // USERS TABLE
        // ========================
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::FirstName).string_len(64).null())
                    .col(ColumnDef::new(Users::LastName).string_len(64).null())
                    .col(
                        ColumnDef::new(Users::Email)
                            .string_len(128)
                            .null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::Password).string_len(64).null())
                    .col(ColumnDef::new(Users::DeletedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Users::Token).text().null())
                    .to_owned(),
            )
            .await?;


        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_users_email")
                    .table(Users::Table)
                    .col(Users::Email)
                    .to_owned(),
            )
            .await?;


         // ========================
        // PROVIDERS TABLE
        // ========================
        manager
            .create_table(
                Table::create()
                    .table(Providers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Providers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Providers::ProviderName).string_len(32).not_null())
                    .col(ColumnDef::new(Providers::ExternalId).string_len(128).not_null())
                    .col(ColumnDef::new(Providers::AccessToken).text().null())
                    .col(ColumnDef::new(Providers::RefreshToken).text().null())
                    .col(ColumnDef::new(Providers::ExpiresAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Providers::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(Providers::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Providers::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

         manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_providers_user_id")
                    .from(Providers::Table, Providers::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
    .create_index(
        Index::create()
            .if_not_exists()
            .name("idx_providers_user_id_provider_name")
            .table(Providers::Table)
            .col(Providers::ExternalId)
            .col(Providers::ProviderName) // add second column
            .to_owned(),
    )
    .await?;






                // ========================
        // TASKS TABLE
        // ========================
        manager
            .create_table(
                Table::create()
                    .table(Tasks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tasks::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tasks::Priority).string_len(4).null())
                    .col(ColumnDef::new(Tasks::Title).string_len(255).not_null())
                    .col(ColumnDef::new(Tasks::CompletedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Tasks::Description).text().null())
                    .col(ColumnDef::new(Tasks::DeletedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Tasks::UserId).integer().null())
                    .col(
                        ColumnDef::new(Tasks::IsDefault)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;



          manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_tasks_user_id")
                    .from(Tasks::Table, Tasks::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        // Index for tasks.user_id
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_tasks_user_id")
                    .table(Tasks::Table)
                    .col(Tasks::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

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
    UpdatedAt
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