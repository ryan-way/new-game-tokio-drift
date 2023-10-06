mod counter;

pub use counter::*;
use db::entity;
use db::repository::Repository;

use mockall::*;

pub struct MainViewModel<'a> {
    counter_vm: CounterViewModel<'a>,
}

impl<'a> MainViewModel<'a> {
    pub fn new(counter_repo: &'a dyn Repository<entity::counter::Model>) -> Self {
        Self {
            counter_vm: CounterViewModel::new(counter_repo),
        }
    }
}

impl<'a> MainVm for MainViewModel<'a> {
    fn counter(&mut self) -> &mut dyn CounterVm {
        &mut self.counter_vm
    }
}

#[automock]
pub trait MainVm {
    fn counter(&mut self) -> &mut dyn CounterVm;
}
