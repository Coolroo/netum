pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240204_174756_update_book;
mod m20240218_175044_add_identifier_and_isbn_to_book;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240204_174756_update_book::Migration),
            Box::new(m20240218_175044_add_identifier_and_isbn_to_book::Migration)
        ]
    }
}
