use async_trait::async_trait;
use dotenv::dotenv;
use sea_orm::DatabaseConnection;
use std::env;
use std::error::Error;

#[async_trait]
pub trait Repository<M>: Sync {
    async fn db(&self) -> Result<DatabaseConnection, Box<dyn Error>> {
        dotenv()?;
        let var = env::var("DATABASE_URL")?;
        let db = sea_orm::Database::connect(var).await?;
        Ok(db)
    }

    async fn first(&self) -> Result<M, Box<dyn Error>>;
    async fn update(&self, model: &M) -> Result<(), Box<dyn Error>>;
}
