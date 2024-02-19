use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub mod m20240218_000001_create_client_table;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240218_000001_create_client_table::Migration)]
    }
}
