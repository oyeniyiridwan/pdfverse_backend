use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       manager
            .alter_table(Table::alter().table(Users::Table).drop_column(Users::Token).to_owned())
            .await?;  

      
        

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
manager
    .alter_table(
        Table::alter()
            .table(Users::Table)
            .add_column_if_not_exists(ColumnDef::new(Users::Token).text().null())
            .to_owned(),
    )
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
    EmailVerified
}


