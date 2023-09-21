use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
// use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::backend::CrosstermBackend;
use std::error::Error;
use std::io;
use std::io::Stdout;
use std::time::Duration;

use crate::views::MainView;

pub struct Terminal {
    terminal: ratatui::Terminal<CrosstermBackend<Stdout>>,
}

impl Terminal {
    pub fn new() -> Result<Terminal, Box<dyn Error>> {
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;
        let terminal = ratatui::Terminal::new(CrosstermBackend::new(stdout))?;
        Ok(Self { terminal })
    }

    pub fn run(&mut self, main_view: &mut MainView) -> Result<(), Box<dyn Error>> {
        loop {
            self.terminal.draw(|frame| {
                main_view.draw(frame);
            })?;
            if event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    main_view.handle_key(key.code);
                }
            }
            if main_view.should_quit() {
                break;
            }
            main_view.route();
        }
        Ok(())
    }

    pub fn restore(&mut self) -> Result<(), Box<dyn Error>> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen,)?;
        Ok(self.terminal.show_cursor()?)
    }
}

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;
