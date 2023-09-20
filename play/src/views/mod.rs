use ratatui::{prelude::Backend, widgets::Paragraph, Frame};

use self::logger::LoggerView;

pub mod logger;

#[derive(Default)]
pub struct MainView {
    logger_view: LoggerView,
    show_log: bool,
}

impl MainView {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) {
        if self.show_log {
            self.logger_view.draw(f)
        } else {
            f.render_widget(Paragraph::new("Hello World!"), f.size())
        }
    }

    pub fn key_press(&mut self, c: char) {
        match c {
            'l' => self.show_log = !self.show_log,
            'i' => log::info!("Logged from key press"),
            _ => log::error!("Invalid key press"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_when_main_view_created_show_log_is_false() {
        let main_view = MainView::default();

        assert_eq!(main_view.show_log, false);
    }

    #[test]
    fn test_when_main_view_handles_l_key_show_log_toggles() {
        let mut main_view = MainView::default();

        assert!(!main_view.show_log);

        main_view.key_press('l');

        assert!(main_view.show_log);

        main_view.key_press('l');

        assert!(!main_view.show_log);
    }
}
