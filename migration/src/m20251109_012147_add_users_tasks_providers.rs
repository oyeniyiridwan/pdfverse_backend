use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        

        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .to_owned(),
            )
            .await?;


           
        // 3️⃣ Alter providers.user_id → UUID with default
        manager
            .alter_table(
                Table::alter()
                    .table(Providers::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Providers::UserId)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .to_owned(),
            )
            .await?;


        // 4️⃣ Alter tasks.user_id → UUID with default
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Tasks::UserId)
                            .uuid()
                            .null()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .to_owned(),
            )
            .await?;

        // 5️⃣ Recreate foreign keys
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
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_tasks_user_id")
                    .from(Tasks::Table, Tasks::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
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
