extern crate game;

use db::entity::counter::Model;
use db::repository::Repository;
use std::error::Error;

pub struct CounterViewModel<'a> {
    repo: &'a dyn Repository<Model>,
    model: Model,
}

impl<'a> CounterViewModel<'a> {
    pub fn new(repo: &'a dyn Repository<Model>) -> Self {
        Self {
            repo,
            model: Model::default(),
        }
    }
}

#[async_trait::async_trait]
impl<'a> CounterVm for CounterViewModel<'a> {
    async fn load(&mut self) -> Result<(), Box<dyn Error>> {
        log::info!("CounterVm: Loading data");
        self.model = self.repo.first().await?;
        log::debug!("{:?}", self.model);
        log::info!("CounterVm: Loaded");
        Ok(())
    }

    async fn save(&mut self) -> Result<(), Box<dyn Error>> {
        log::info!("CounterVm: Saving data");
        self.repo.update(&self.model).await?;
        log::debug!("{:?}", self.model);
        log::info!("CounterVm: Saved");
        Ok(())
    }

    fn add(&mut self) {
        log::info!("CounterVm: Adding to {}", self.model.count);
        self.model.count = game::count(self.model.count);
        log::debug!("{:?}", self.model);
        log::info!("CounterVm: Result {}", self.model.count);
    }

    fn minus(&mut self) {
        log::info!("CounterVm: Minusing to {}", self.model.count);
        self.model.count -= 1;
        log::debug!("{:?}", self.model);
        log::info!("CounterVm: Result {}", self.model.count);
    }

    fn reset(&mut self) {
        log::info!("CounterVm: Reseting");
        self.model = Model::default();
        log::debug!("{:?}", self.model);
    }

    fn data(&self) -> &Model {
        &self.model
    }
}

#[async_trait::async_trait]
pub trait CounterVm {
    async fn load(&mut self) -> Result<(), Box<dyn Error>>;
    async fn save(&mut self) -> Result<(), Box<dyn Error>>;

    fn add(&mut self);
    fn minus(&mut self);
    fn reset(&mut self);
    fn data(&self) -> &Model;
}
