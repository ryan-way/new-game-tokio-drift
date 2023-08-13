use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Cell, Row, Table},
};

use crate::viewmodels::{TicTacToeViewModel, Token};
use crate::views::View;

static O: &str = "     
 ███ 
 █ █ 
 ███ 
     ";

static X: &str = "     
 █ █ 
  █  
 █ █ 
     ";

static EMPTY: &str = "";

pub struct TicTacToeView {
    view_model: TicTacToeViewModel,
}

impl TicTacToeView {
    pub fn default() -> TicTacToeView {
        TicTacToeView {
            view_model: TicTacToeViewModel::default(),
        }
    }

    fn get_cell_style(&self, idx: usize) -> Style {
        let (fg, bg) = if idx % 2 == 0 {
            (Color::Black, Color::Green)
        } else {
            (Color::Green, Color::Black)
        };
        Style::default().fg(fg).bg(bg)
    }

    fn token_to_cell(&self, token: &Token, idx: usize) -> Cell {
        Cell::from(match token {
            Token::O => O,
            Token::X => X,
            Token::Empty => EMPTY,
        })
        .style(self.get_cell_style(idx))
    }

    fn tokens_to_row(&self, tokens: &[Token; 3], row_idx: usize) -> Row {
        Row::new(
            tokens
                .iter()
                .enumerate()
                .map(|(cell_idx, token)| self.token_to_cell(token, cell_idx + row_idx))
                .collect::<Vec<Cell>>(),
        )
        .height(5)
    }
}

impl<'a> View<'a, Table<'a>> for TicTacToeView {
    fn get_widget(&'a self) -> Table<'a> {
        Table::new(
            self.view_model
                .board()
                .iter()
                .enumerate()
                .map(|(idx, row)| self.tokens_to_row(row, idx))
                .collect::<Vec<Row>>(),
        )
        .column_spacing(0)
        .widths(&[
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
        ])
    }

    fn get_rect(&self, f: &Rect) -> Rect {
        let width = 5 + 5 + 5;
        let height = 5 + 5 + 5;
        Rect {
            x: f.width / 2 - width / 2,
            y: f.height / 2 - height / 2,
            width,
            height,
        }
    }

    fn get_title(&self) -> String {
        String::from("Tic Tac Toe")
    }

    fn handle_key(&mut self, key: KeyCode) -> Result<(), &str> {
        match key {
            KeyCode::Char(k) => match k {
                '1'..='9' => {
                    let num = k.to_string().parse::<usize>().or(Err("Cannot Parse"))? - 1;
                    self.view_model.turn(num % 3, num / 3)?;
                    Ok(())
                }
                'r' => {
                    self.view_model.reset();
                    Ok(())
                }
                _ => Err("Unaccpeted character"),
            },
            _ => Err("Unaccepted command"),
        }
    }
}
