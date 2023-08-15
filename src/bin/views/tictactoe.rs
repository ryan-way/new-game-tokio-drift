use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Cell, Row, Table},
};

use ratatui::{backend::CrosstermBackend, Terminal};

use ratatui::{
    layout::Alignment,
    style::Modifier,
    text::Span,
    widgets::{Block, BorderType, Borders},
};
use std::io::Stdout;

use crate::viewmodels::tictactoe::{IViewModel, Token};

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

pub struct View<'a> {
    quit: bool,
    view_model: &'a mut dyn IViewModel,
}

impl<'a> View<'a> {
    pub fn default(view_model: &'a mut dyn IViewModel) -> Self {
        Self {
            quit: false,
            view_model,
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

    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(k) => match k {
                '1'..='9' => {
                    let num = k.to_string().parse::<usize>().unwrap() - 1;
                    self.view_model.turn(num % 3, num / 3);
                }
                'r' => {
                    self.view_model.reset();
                }
                'q' => {
                    self.quit = true;
                }
                _ => println!("Unaccepted character"),
            },
            _ => println!("Unaccepted command"),
        }
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    pub fn draw(&self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        terminal.draw(|f| {
            let block = Block::default()
                .title(Span::styled(
                    self.get_title(),
                    Style::default().add_modifier(Modifier::BOLD),
                ))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::White));
            f.render_widget(block, f.size());

            let size = self.get_rect(&f.size());
            let table = Table::new(
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
            ]);

            f.render_widget(table, size);
        });
    }
}
