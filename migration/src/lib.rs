pub use sea_orm_migration::prelude::*;

mod m20230918_210522_create_counter;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230918_210522_create_counter::Migration),
        ]
    }
}
