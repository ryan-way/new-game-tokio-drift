use super::base::Repository;
use crate::entity::counter::ActiveModel;
use crate::entity::counter::Model;
use crate::entity::prelude::*;
use log;
use sea_orm::ActiveModelTrait;
use sea_orm::Set;
use sea_orm::{DbErr, EntityTrait};
use std::error::Error;

pub struct CounterRespoitory;

#[async_trait::async_trait]
impl Repository<Model> for CounterRespoitory {
    async fn first(&self) -> Result<Model, Box<dyn Error>> {
        log::info!("Fetching first Counter");
        let db = self.db().await?;
        let counter = Counter::find()
            .one(&db)
            .await?
            .ok_or(DbErr::Custom("Couldn't find Counter".to_owned()))?;
        log::info!("Fetched counter with id: {}", counter.id);
        Ok(counter)
    }

    async fn update(&self, model: &Model) -> Result<(), Box<dyn Error>> {
        log::info!("Updating counter with id: {}", model.id);
        let db = self.db().await?;
        let mut active: ActiveModel = model.clone().into();
        active.name = Set(model.name.clone());
        active.count = Set(model.count);
        log::info!("Active Model: {:?}", active);
        active.update(&db).await?;
        Ok(())
    }
}
