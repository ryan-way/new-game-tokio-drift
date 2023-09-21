mod counter;

pub use counter::*;
use db::entity;
use db::repository::Repository;

pub struct MainViewModel<'a> {
    counter_repo: &'a mut dyn Repository<entity::counter::Model>,
}

impl<'a> MainViewModel<'a> {
    pub fn counter(&self) -> CounterViewModel<'a> {
        CounterViewModel::new(self.counter_repo)
    }
}
