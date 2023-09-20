use ratatui::prelude::Backend;
use ratatui::style::{Color, Style};
use ratatui::Frame;
use tui_logger::TuiLoggerWidget;

#[derive(Default)]
pub struct LoggerView;

impl LoggerView {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) {
        let logger = TuiLoggerWidget::default()
            .output_separator(' ')
            .output_timestamp(Some(String::from("%F %T")))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_error(Style::default().fg(Color::Red))
            .style_trace(Style::default().fg(Color::Blue));

        f.render_widget(logger, f.size());
    }
}
