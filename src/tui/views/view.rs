use crossterm::event::KeyCode;
use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
};
use ratatui::{layout::Rect, widgets::Widget};

use ratatui::backend::Backend;

use ratatui::terminal::Frame;

pub trait View<'a, W>
where
    W: Widget,
{
    fn get_widget(&'a self) -> W;
    fn get_rect(&self, f: &Rect) -> Rect;

    fn get_title(&self) -> String;

    fn draw<B>(&'a self, f: &mut Frame<B>)
    where
        B: Backend,
    {
        let block = Block::default()
            .title(Span::styled(
                self.get_title(),
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White));
        f.render_widget(block, f.size());

        let size = self.get_rect(&f.size());
        let widget = self.get_widget();

        f.render_widget(widget, size);
    }

    fn handle_key(&mut self, key: KeyCode) -> Result<(), &str>;
}
