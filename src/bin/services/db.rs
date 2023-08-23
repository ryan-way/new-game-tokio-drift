use log;
use new_game_tokio_drift::entities::prelude::*;
use sea_orm::{Database, DbErr, EntityTrait};

const DB_URL: &str = "sqlite://sqlite.db";

pub async fn initialize_db() -> Result<(), DbErr> {
    let db = Database::connect(DB_URL).await?;
    let hello_world = HelloWorld::find_by_id(1)
        .one(&db)
        .await?
        .ok_or(DbErr::RecordNotFound(String::from("Hello World")))?;
    log::info!("{}", hello_world.message);
    log::trace!("{}", hello_world.something_else);
    db.close().await?;
    Ok(())
}
