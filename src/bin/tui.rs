extern crate log;
extern crate tui_logger;

mod models;
mod services;
mod viewmodels;
mod views;

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use models::MainModel;
use ratatui::{backend::CrosstermBackend, Terminal};
use services::log::intialize_logger;
use std::{
    io::{self, Stdout},
    time::Duration,
};
use viewmodels::MainViewModel;
use views::MainView;

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), &'static str> {
    let mut show_logs = false;
    let logger_view = views::logger::View::default();
    let mut main_model = MainModel::default();
    let mut main_viewmodel = MainViewModel::default(&mut main_model);
    let mut main_view = MainView::default(&mut main_viewmodel);
    loop {
        if show_logs {
            logger_view.draw(terminal);
        } else {
            main_view.draw(terminal);
        }

        if event::poll(Duration::from_millis(250)).or(Err("Couldn't poll"))? {
            if let Event::Key(key) = event::read().or(Err("Couldn't read key"))? {
                if KeyCode::Char('l') == key.code {
                    show_logs = !show_logs;
                } else if KeyCode::Char('k') == key.code {
                    log::info!("Some Info");
                    log::error!("An Error");
                    log::trace!("tracing...");
                    log::debug!("Gonna debug");
                    log::warn!("Final warning");
                } else {
                    main_view.handle_key(key.code);
                }
            }
        }

        if main_view.should_quit() {
            break;
        }
    }
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}

fn main() -> Result<()> {
    intialize_logger();
    let mut terminal = setup_terminal().context("setup failed")?;
    run(&mut terminal).unwrap();
    restore_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}
