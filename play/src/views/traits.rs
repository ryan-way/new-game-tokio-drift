use crate::terminal::Frame;
use crossterm::event::KeyCode;

use crate::services::Router;

pub trait View {
    fn draw(&mut self, f: &mut Frame);
    fn handle_key(&mut self, code: KeyCode, router: &mut Router);
}
