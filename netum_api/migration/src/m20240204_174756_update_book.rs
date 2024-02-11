use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.alter_table(
            Table::alter()
                .table(Book::Table)
                .modify_column(
                    ColumnDef::new(Book::Genre)
                        .array(ColumnType::String(None))
                        .not_null()
                        .default(Value::Array(ArrayType::String, Some(Box::new(vec![]))))
                )
                .rename_column(Book::Genre, Book::Categories)
                .modify_column(ColumnDef::new(Book::Location).string())
                .add_column(ColumnDef::new(Book::LentTo).string())
                .add_column(ColumnDef::new(Book::CreatedAt).timestamp_with_time_zone().not_null())
                .add_column(ColumnDef::new(Book::UpdatedAt).timestamp_with_time_zone().not_null())
                .add_column(ColumnDef::new(Book::DeletedAt).timestamp_with_time_zone())
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
                .table(Book::Table)
                .modify_column(ColumnDef::new(Book::Categories).string().not_null())
                .rename_column(Book::Categories, Book::Genre)
                .drop_column(Book::LentTo)
                .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum Book {
    Table,
    Genre,
    Categories,
    Location,
    LentTo,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
