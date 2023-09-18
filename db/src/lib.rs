extern crate dotenv;
extern crate log;

mod entity;

use dotenv::dotenv;

use std::env;

pub use entity::prelude::*;
use sea_orm::{DbErr, EntityTrait};

pub async fn db_test() -> Result<(), DbErr> {
    dotenv().ok();
    log::info!("Testing db");
    let db_var = env::var("DATABASE_URL").ok().ok_or(DbErr::Custom(
        "\"DATABASE_URL\" variable not defined".to_owned(),
    ))?;
    let db = sea_orm::Database::connect(db_var).await?;
    let counter = Counter::find()
        .one(&db)
        .await?
        .ok_or(DbErr::RecordNotFound("Counter not found".to_owned()))?;
    log::info!("Counter: count {}; name {}", counter.count, counter.name);
    log::info!("Testing complete");
    Ok(())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
