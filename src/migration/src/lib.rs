pub use sea_orm_migration::prelude::*;

mod m20230828_060531_test;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230828_060531_test::Migration)]
    }
}
