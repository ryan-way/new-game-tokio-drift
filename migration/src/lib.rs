pub use sea_orm_migration::prelude::*;

mod m20230918_210522_create_counter;
mod m20231016_214912_tic_tac_toe;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230918_210522_create_counter::Migration),
            Box::new(m20231016_214912_tic_tac_toe::Migration),
        ]
    }
}
