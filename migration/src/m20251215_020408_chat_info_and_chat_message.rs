use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
          manager
            .alter_table(
                Table::alter()
                    .table(ChatInfo::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(ChatInfo::Path)
                            .text()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
            manager.alter_table(
                Table::alter()
                    .table(ChatInfo::Table)
                    .drop_column(ChatInfo::Path)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ChatInfo {
    Table,
    Id,
    UserId,
    Title,
    Path,
    CreatedAt,
}
