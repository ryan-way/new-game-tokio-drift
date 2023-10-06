use crossterm::event::KeyCode;
use std::convert::From;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Route {
    #[default]
    MainMenu,
    Counter,
    Logger,
}

#[derive(Debug)]
pub enum Command {
    Load,
    Save,
    Quit,
    Add,
    Reset,
    Minus,
    Logger,
    Up,
    Down,
    Left,
    Right,
    Select,
    Unselect,
    Invalid,
}

impl From<KeyCode> for Command {
    fn from(key: KeyCode) -> Self {
        match key {
            KeyCode::Char(c) => match c {
                'l' => Command::Load,
                's' => Command::Save,
                'o' => Command::Logger,
                'a' => Command::Add,
                'm' => Command::Minus,
                'q' => Command::Quit,
                'r' => Command::Reset,
                _ => Command::Invalid,
            },
            KeyCode::Up => Command::Up,
            KeyCode::Down => Command::Down,
            KeyCode::Left => Command::Left,
            KeyCode::Right => Command::Right,
            KeyCode::Enter => Command::Select,
            KeyCode::Backspace => Command::Unselect,
            _ => Command::Invalid,
        }
    }
}
