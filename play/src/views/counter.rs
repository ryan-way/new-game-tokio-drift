use crossterm::event::KeyCode;
use ratatui::widgets::Paragraph;

use super::traits::View;
use crate::services::Frame;
use crate::services::Router;

// use crate::view_models::CounterViewModel;

pub struct CounterView;

impl CounterView {
    pub fn new() -> Self {
        Self
    }
}

impl View for CounterView {
    fn draw(&mut self, f: &mut Frame) {
        f.render_widget(Paragraph::new("Hello from Counter"), f.size())
    }

    fn handle_key(&mut self, _code: KeyCode, _router: &mut Router) {}
}
