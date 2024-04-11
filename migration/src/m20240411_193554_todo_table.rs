use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TodoTable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TodoTable::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TodoTable::Description).string().not_null())
                    .col(ColumnDef::new(TodoTable::Status).string().not_null())
                    .col(ColumnDef::new(TodoTable::CreateAt).date_time().not_null())
                    .col(ColumnDef::new(TodoTable::UpdateAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TodoTable::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TodoTable {
    Table,
    Id,
    Description,
    Status,
    CreateAt,
    UpdateAt,
}
