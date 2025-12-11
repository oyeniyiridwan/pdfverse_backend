use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

      manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS citext;")
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .modify_column(
                        ColumnDef::new(Users::Email)
                            .custom("CITEXT") // This enables case-insensitive email
                            .null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Providers::Table)
                    .modify_column(
                        ColumnDef::new(Providers::Email)
                           .custom("CITEXT") // This enables case-insensitive email
                            .null()
                            
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .modify_column(ColumnDef::new(Users::Email).string_len(128).null())
                    .to_owned(),
            )
            .await?;
          manager
            .alter_table(
                Table::alter()
                    .table(Providers::Table)
                    .modify_column(
                        ColumnDef::new(Providers::Email)
                            .string_len(128)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .get_connection()
            .execute_unprepared("DROP EXTENSION IF EXISTS citext;")
            .await?;

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
    EmailVerified,
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
