use crate::models::tictactoe::IModel;

pub use crate::models::tictactoe::{Board, Token};

pub struct ViewModel<'a> {
    model: &'a mut dyn IModel,
}

impl<'a> ViewModel<'a> {
    pub fn default(model: &'a mut dyn IModel) -> Self {
        ViewModel { model }
    }
}

impl<'a> IViewModel for ViewModel<'a> {
    fn reset(&mut self) {
        self.model.reset();
    }

    fn turn(&mut self, x: usize, y: usize) {
        if x > 2 {
            println!("X out of bounds")
        } else if y > 2 {
            println!("Y out of bounds")
        } else {
            self.model.turn(x + y * 3);
        }
    }

    fn board(&self) -> &Board {
        &self.model.board()
    }
}

pub trait IViewModel {
    fn reset(&mut self);
    fn turn(&mut self, x: usize, y: usize);
    fn board(&self) -> &Board;
}
