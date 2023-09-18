extern crate log;

mod logger;
mod views;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::Paragraph, Terminal};
use std::{error::Error, io::Stdout};
use std::{io, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal).await?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

async fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    logger::init_logger()?;
    let mut show_logger = false;
    let logger = views::logger::View::default();

    db::db_test().await?;

    log::info!("Testing game logic: {}", game::count(1));

    loop {
        if show_logger {
            logger.draw(terminal);
        } else {
            terminal.draw(|frame| {
                let greeting = Paragraph::new("Hello World!");
                frame.render_widget(greeting, frame.size());
            })?;
        }
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => match c {
                        'q' => break,
                        'l' => show_logger = !show_logger,
                        'i' => log::info!("Logged from key press"),
                        _ => log::error!("Invalid key press"),
                    },
                    _ => log::error!("Invalide Key Code type"),
                }
            }
        }
    }
    Ok(())
}
