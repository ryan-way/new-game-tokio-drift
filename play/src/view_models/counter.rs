use db::entity;
use db::repository::Repository;

pub struct CounterViewModel<'a> {
    repo: &'a mut dyn Repository<entity::counter::Model>,
}

impl<'a> CounterViewModel<'a> {
    pub fn new(repo: &'a mut dyn Repository<entity::counter::Model>) -> Self {
        Self { repo }
    }
}
