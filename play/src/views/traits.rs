use crate::{terminal::Frame, utils::Command};

use crate::services::Router;

pub trait View {
    fn draw(&mut self, f: &mut Frame);
    fn handle_key(&mut self, command: Command, router: &mut Router);
}
