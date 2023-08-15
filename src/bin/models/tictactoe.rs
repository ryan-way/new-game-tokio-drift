use new_game_tokio_drift::game::tictactoe;

pub use tictactoe::Board;
pub use tictactoe::Token;

use tictactoe::Game;

pub struct Model {
    game: Game,
}

impl Model {
    pub fn default() -> Self {
        Self {
            game: Game::default(),
        }
    }
}

impl IModel for Model {
    fn reset(&mut self) {
        self.game.reset();
    }

    fn turn(&mut self, idx: usize) {
        self.game.turn(idx);
    }

    fn board(&self) -> &tictactoe::Board {
        &self.game.board
    }
}

pub trait IModel {
    fn reset(&mut self);
    fn turn(&mut self, idx: usize);
    fn board(&self) -> &tictactoe::Board;
}
