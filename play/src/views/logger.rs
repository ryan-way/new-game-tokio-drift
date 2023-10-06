use ratatui::style::{Color, Style};
use tui_logger::TuiLoggerWidget;

use super::traits::View;
use crate::services::Router;
use crate::terminal::Frame;
use crate::utils::Command;

pub struct LoggerView;

impl LoggerView {
    pub fn new() -> Self {
        Self
    }
}

impl View for LoggerView {
    fn draw(&mut self, f: &mut Frame) {
        let logger = TuiLoggerWidget::default()
            .output_separator(' ')
            .output_timestamp(Some(String::from("%F %T")))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_error(Style::default().fg(Color::Red))
            .style_trace(Style::default().fg(Color::Blue));

        f.render_widget(logger, f.size());
    }
    fn handle_key(&mut self, _command: Command, _router: &mut Router) {}
}
