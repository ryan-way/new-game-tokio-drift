use crate::models::IMainModel;
use crate::viewmodels::tictactoe::{IViewModel, ViewModel};

pub struct MainViewModel<'a> {
    tictactoe: ViewModel<'a>,
}

impl<'a> MainViewModel<'a> {
    pub fn default(model: &'a mut dyn IMainModel) -> Self {
        MainViewModel {
            tictactoe: ViewModel::default(model.tictactoe()),
        }
    }
}

impl<'a> IMainViewModel for MainViewModel<'a> {
    fn tictactoe(&mut self) -> &mut dyn IViewModel {
        &mut self.tictactoe
    }
}

pub trait IMainViewModel {
    fn tictactoe(&mut self) -> &mut dyn IViewModel;
}
