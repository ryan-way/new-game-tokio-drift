use crate::{terminal::Frame, utils::Command};
use ratatui::{
    prelude::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::services::Router;
use crate::utils::Route;

use super::traits::View;

const NUM_ITEMS: usize = 3;
pub struct MainMenuView {
    state: ListState,
    list: [&'static str; NUM_ITEMS],
}

impl MainMenuView {
    pub fn new() -> Self {
        let list = ["Counter", "Temp", "Something"];
        Self {
            state: ListState::default(),
            list,
        }
    }

    fn next(&mut self) {
        let next = match self.state.selected() {
            Some(i) => {
                if i < NUM_ITEMS - 1 {
                    i + 1
                } else {
                    0
                }
            }
            None => 0,
        };

        self.state.select(Some(next));
    }

    fn previous(&mut self) {
        let previous = match self.state.selected() {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    NUM_ITEMS - 1
                }
            }
            None => NUM_ITEMS - 1,
        };

        self.state.select(Some(previous));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }

    fn select(&mut self, router: &mut Router) {
        match self.state.selected() {
            Some(i) => match i {
                0 => router.set_route(Route::Counter),
                _ => log::warn!("Ignoring selection: {}", i),
            },
            None => log::warn!("Nothing Selected!"),
        }
    }

    fn size(&self, f: Rect) -> Rect {
        let width = 5 + 5 + 5;
        let height = 5 + 5 + 5;
        Rect {
            x: f.width / 2 - width / 2,
            y: f.height / 2 - height / 2,
            width,
            height,
        }
    }
}

impl View for MainMenuView {
    fn draw(&mut self, f: &mut Frame) {
        let list = List::new(self.list.map(ListItem::new))
            .block(Block::default().title("Main Menu").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">>");

        f.render_stateful_widget(list, self.size(f.size()), &mut self.state)
    }

    fn handle_key(&mut self, command: Command, router: &mut Router) {
        match command {
            Command::Up => self.previous(),
            Command::Down => self.next(),
            Command::Select => self.select(router),
            Command::Unselect => self.unselect(),
            _ => log::warn!("Invalid keycode"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod when_created {
        use super::*;

        #[test]
        fn nothing_selected() {
            let main_menu = MainMenuView::new();

            assert_eq!(main_menu.state.selected(), None);
        }
    }

    mod when_nothing_selected {
        use super::*;

        #[test]
        fn next_selects_first() {
            let mut main_menu = MainMenuView::new();
            assert_eq!(main_menu.state.selected(), None);

            main_menu.next();
            assert_eq!(main_menu.state.selected(), Some(0));
        }

        #[test]
        fn previous_selects_last() {
            let mut main_menu = MainMenuView::new();
            assert_eq!(main_menu.state.selected(), None);

            main_menu.previous();
            assert_eq!(main_menu.state.selected(), Some(NUM_ITEMS - 1));
        }

        #[test]
        fn select_does_not_change_route() {
            let mut main_menu = MainMenuView::new();
            let mut router = Router::default();
            assert_eq!(main_menu.state.selected(), None);

            main_menu.handle_key(Command::Select, &mut router);
            assert_eq!(router.route(), Route::MainMenu);
        }
    }

    mod when_first_selected {
        use super::*;

        #[test]
        fn next_selects_second() {
            let mut main_menu = MainMenuView::new();
            main_menu.state.select(Some(0));

            main_menu.next();
            assert_eq!(main_menu.state.selected(), Some(1));
        }

        #[test]
        fn previous_selects_last() {
            let mut main_menu = MainMenuView::new();
            main_menu.state.select(Some(0));

            main_menu.previous();
            assert_eq!(main_menu.state.selected(), Some(NUM_ITEMS - 1));
        }

        #[test]
        fn unselect_selects_nothing() {
            let mut main_menu = MainMenuView::new();
            main_menu.state.select(Some(0));

            main_menu.unselect();
            assert_eq!(main_menu.state.selected(), None);
        }

        #[test]
        fn select_routes_to_counter() {
            let mut main_menu = MainMenuView::new();
            let mut router = Router::default();
            main_menu.state.select(Some(0));

            main_menu.handle_key(Command::Select, &mut router);
            assert_eq!(router.route(), Route::Counter);
        }
    }

    mod when_last_selected {
        use super::*;

        #[test]
        fn next_selects_first() {
            let mut main_menu = MainMenuView::new();
            main_menu.state.select(Some(NUM_ITEMS - 1));

            main_menu.next();
            assert_eq!(main_menu.state.selected(), Some(0));
        }

        #[test]
        fn previous_selects_second_to_last() {
            let mut main_menu = MainMenuView::new();
            main_menu.state.select(Some(NUM_ITEMS - 1));

            main_menu.previous();
            assert_eq!(main_menu.state.selected(), Some(NUM_ITEMS - 2));
        }

        #[test]
        fn unselect_selects_nothing() {
            let mut main_menu = MainMenuView::new();
            main_menu.state.select(Some(NUM_ITEMS - 1));

            main_menu.unselect();
            assert_eq!(main_menu.state.selected(), None);
        }
    }
}
