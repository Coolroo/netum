use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Location::Table)
                    .col(
                        ColumnDef::new(Location::Location)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Location::Color)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .col(
                        ColumnDef::new(Book::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Book::Title).string().not_null())
                    .col(ColumnDef::new(Book::SubTitle).string())
                    .col(
                        ColumnDef::new(Book::Authors)
                            .array(ColumnType::String(None))
                            .not_null()
                            .default(Value::Array(ArrayType::String, Some(Box::new(vec![])))),
                    )
                    .col(ColumnDef::new(Book::Genre).string())
                    .col(ColumnDef::new(Book::Location).string())
                    .col(ColumnDef::new(Book::Read).boolean())
                    .col(ColumnDef::new(Book::Description).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_book_to_location")
                            .from(Book::Table, Book::Location)
                            .to(Location::Table, Location::Location)
                            .on_update(ForeignKeyAction::SetDefault),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Location::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Location {
    Table,
    Location,
    Color,
}

#[derive(DeriveIden)]
enum Book {
    Table,
    Id,
    Title,
    SubTitle,
    Authors,
    Genre,
    Location,
    Read,
    Description,
}
