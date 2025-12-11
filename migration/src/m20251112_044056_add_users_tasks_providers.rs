use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
          manager
            .drop_foreign_key(ForeignKey::drop().name("fk_tasks_user_id").table(Tasks::Table).to_owned())
            .await?;

     manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .modify_column(
                        ColumnDef::new(Tasks::UserId)
                            .uuid()
                            .not_null()

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
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

           manager
            .drop_foreign_key(ForeignKey::drop().name("fk_tasks_user_id").table(Tasks::Table).to_owned())
            .await?;
manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .modify_column(
                        ColumnDef::new(Tasks::UserId)
                            .uuid()
                            .null()
                          
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
              Ok(())
    }
}


#[derive(DeriveIden)]
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
