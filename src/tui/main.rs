use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
    Terminal,
};
use std::{
    io::{self, Stdout},
    time::Duration,
};

enum Token {
    X,
    O,
    Empty,
}

static O: &'static str = "     
 ███ 
 █ █ 
 ███ 
     ";

static X: &'static str = "     
 █ █ 
  █  
 █ █ 
     ";

static EMPTY: &'static str = "";

fn get_cell_style(idx: u8) -> Style {
    let (fg, bg) = if idx % 2 == 0 {
        (Color::Black, Color::Green)
    } else {
        (Color::Green, Color::Black)
    };
    Style::default().fg(fg).bg(bg)
}

fn token_to_cell(token: &Token, idx: u8) -> Cell {
    Cell::from(match token {
        Token::O => O,
        Token::X => X,
        Token::Empty => EMPTY,
    })
    .style(get_cell_style(idx))
}

fn tokens_to_row(tokens: &[Token; 3], idx: u8) -> Row {
    Row::new(vec![
        token_to_cell(&tokens[0], 0 + idx),
        token_to_cell(&tokens[1], 1 + idx),
        token_to_cell(&tokens[2], 2 + idx),
    ])
    .height(5)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    Ok(loop {
        terminal.draw(|f| {
            let block = Block::default()
                .title(Span::styled(
                    "Tic Tac Toe",
                    Style::default().add_modifier(Modifier::BOLD),
                ))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::White));

            f.render_widget(block, f.size());
            let width = 5 + 5 + 5 + 2;
            let height = 5 + 5 + 5 + 2;
            let size = ratatui::layout::Rect {
                x: f.size().width / 2 - width / 2,
                y: f.size().height / 2 - height / 2,
                width,
                height,
            };
            let table = Table::new(vec![
                tokens_to_row(&[Token::X, Token::O, Token::Empty], 0),
                tokens_to_row(&[Token::O, Token::Empty, Token::X], 1),
                tokens_to_row(&[Token::Empty, Token::O, Token::X], 2),
            ])
            .column_spacing(0)
            .widths(&[
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
            ]);

            f.render_widget(table, size);
        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    })
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
    let mut terminal = setup_terminal().context("setup failed")?;
    run(&mut terminal).context("app loop failed")?;
    restore_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}
