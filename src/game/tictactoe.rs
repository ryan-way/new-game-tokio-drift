#[derive(PartialEq, Copy, Clone)]
pub enum Token {
    X,
    O,
    Empty,
}

pub type Board = [[Token; 3]; 3];

pub struct Game {
    pub board: Board,
    pub current_token: Token,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: [
                [Token::Empty, Token::Empty, Token::Empty],
                [Token::Empty, Token::Empty, Token::Empty],
                [Token::Empty, Token::Empty, Token::Empty],
            ],
            current_token: Token::X,
        }
    }
}

impl Game {
    pub fn reset(&mut self) {
        for y in 0..3 {
            for x in 0..3 {
                self.board[y][x] = Token::Empty;
            }
        }
        self.current_token = Token::X;
    }

    pub fn turn(&mut self, idx: usize) {
        if idx > 8 {
            println!("Out of bounds")
        } else if self.board[idx / 3][idx % 3] != Token::Empty {
            println!("Non empty move")
        } else {
            self.board[idx / 3][idx % 3] = self.current_token;
            self.toggle_token();
        }
    }

    fn toggle_token(&mut self) {
        self.current_token = if self.current_token == Token::X {
            Token::O
        } else {
            Token::X
        }
    }
}
