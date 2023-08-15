use crate::models::tictactoe;
pub struct MainModel {
    tictactoe: tictactoe::Model,
}

impl MainModel {
    pub fn default() -> Self {
        MainModel {
            tictactoe: tictactoe::Model::default(),
        }
    }
}

impl IMainModel for MainModel {
    fn tictactoe(&mut self) -> &mut dyn tictactoe::IModel {
        &mut self.tictactoe
    }
}

pub trait IMainModel {
    fn tictactoe(&mut self) -> &mut dyn tictactoe::IModel;
}
