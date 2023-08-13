use std::result::Result;

#[derive(PartialEq, Copy, Clone)]
pub enum Token {
    X,
    O,
    Empty,
}

pub type TicTacToeBoard = [[Token; 3]; 3];

pub struct TicTacToeGame {
    board: TicTacToeBoard,
    current_token: Token,
}

impl TicTacToeGame {
    pub fn default() -> Self {
        Self {
            board: [
                [Token::Empty, Token::Empty, Token::Empty],
                [Token::Empty, Token::Empty, Token::Empty],
                [Token::Empty, Token::Empty, Token::Empty],
            ],
            current_token: Token::X,
        }
    }

    pub fn reset(&mut self) {
        for y in 0..3 {
            for x in 0..3 {
                self.board[y][x] = Token::Empty;
            }
        }
        self.current_token = Token::X;
    }

    pub fn turn(&mut self, idx: usize) -> Result<(), &str> {
        if idx > 8 {
            Err("Out of bounds")
        } else if self.board[idx / 3][idx % 3] != Token::Empty {
            Ok(())
        } else {
            self.board[idx / 3][idx % 3] = self.current_token;
            self.toggle_token();
            Ok(())
        }
    }

    pub fn toggle_token(&mut self) {
        self.current_token = if self.current_token == Token::X {
            Token::O
        } else {
            Token::X
        }
    }
}

pub struct TicTacToeViewModel {
    game: TicTacToeGame,
}

impl TicTacToeViewModel {
    pub fn default() -> Self {
        TicTacToeViewModel {
            game: TicTacToeGame::default(),
        }
    }

    pub fn reset(&mut self) {
        self.game.reset();
    }

    pub fn turn(&mut self, x: usize, y: usize) -> Result<(), &str> {
        if x > 2 {
            Err("X out of bounds")
        } else if y > 2 {
            Err("Y out of bounds")
        } else {
            self.game.turn(x + y * 3)?;
            Ok(())
        }
    }

    pub fn board(&self) -> &TicTacToeBoard {
        &self.game.board
    }
}
