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
            .modify_column(ColumnDef::new(Users::Email).string_len(128).not_null())
            .modify_column(ColumnDef::new(Users::Token).text().not_null())
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
            .modify_column(ColumnDef::new(Users::Token).text().null())
            .to_owned(),
    )
    .await?;        Ok(())
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


