use ratatui::style::{Color, Style};
use ratatui::{backend::CrosstermBackend, Terminal};

use std::io::Stdout;

use tui_logger::TuiLoggerWidget;

#[derive(Default)]
pub struct View;

impl View {
    pub fn draw(&self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        terminal
            .draw(|f| {
                let logger = TuiLoggerWidget::default()
                    .output_separator(' ')
                    .output_timestamp(Some(String::from("%F %T")))
                    .style_warn(Style::default().fg(Color::Yellow))
                    .style_error(Style::default().fg(Color::Red))
                    .style_trace(Style::default().fg(Color::Blue));

                f.render_widget(logger, f.size());
            })
            .unwrap();
    }
}
