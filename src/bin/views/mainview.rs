use crate::viewmodels::IMainViewModel;
use crate::views::tictactoe;
use crossterm::event::KeyCode;
use ratatui::{backend::CrosstermBackend, Terminal};

use std::io::Stdout;

pub struct MainView<'a> {
    tictactoe: tictactoe::View<'a>,
}

impl<'a> MainView<'a> {
    pub fn default(viewmodel: &'a mut dyn IMainViewModel) -> Self {
        Self {
            tictactoe: tictactoe::View::default(viewmodel.tictactoe()),
        }
    }
    pub fn handle_key(&mut self, key: KeyCode) {
        self.tictactoe.handle_key(key);
    }
    pub fn draw(&self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        self.tictactoe.draw(terminal);
    }

    pub fn should_quit(&self) -> bool {
        self.tictactoe.should_quit()
    }
}
