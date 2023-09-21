use crossterm::event::KeyCode;

use crate::services::Frame;

mod counter;
mod logger;
mod main_menu;
mod traits;

use crate::services::{Route, Router};
// use crate::view_models::MainViewModel;

use self::counter::CounterView;
use self::logger::LoggerView;
use self::main_menu::MainMenuView;
use self::traits::View;

pub struct MainView {
    router: Router,
    current_view: Box<dyn View>,
    quit: bool,
}

impl MainView {
    pub fn default() -> Self {
        Self {
            router: Router::default(),
            current_view: Box::from(MainMenuView::new()),
            quit: false,
        }
    }

    pub fn draw(&mut self, f: &mut Frame) {
        self.current_view.draw(f);
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') => match self.router.route() {
                Route::MainMenu => self.quit = true,
                _ => self.router.set_route(Route::MainMenu),
            },
            KeyCode::Char('l') => match self.router.route() {
                Route::Logger => self.router.previous(),
                _ => self.router.set_route(Route::Logger),
            },
            _ => {
                self.current_view.handle_key(code, &mut self.router);
            }
        }
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    pub fn route(&mut self) {
        if self.router.should_reroute() {
            self.current_view = match self.router.route() {
                Route::Counter => Box::from(CounterView::new()),
                Route::MainMenu => Box::from(MainMenuView::new()),
                Route::Logger => Box::from(LoggerView::default()),
            };
            self.router.routed();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod when_created {
        use super::*;

        #[test]
        fn route_is_main_menu() {
            let main_view = MainView::default();

            assert_eq!(main_view.router.route(), Route::MainMenu);
        }
    }

    mod when_l_is_pressed {
        use super::*;

        #[test]
        fn route_toggles_logger() {
            let mut main_view = MainView::default();

            assert_eq!(main_view.router.route(), Route::MainMenu);

            main_view.handle_key(KeyCode::Char('l'));

            assert_eq!(main_view.router.route(), Route::Logger);

            main_view.handle_key(KeyCode::Char('l'));

            assert_eq!(main_view.router.route(), Route::MainMenu);
        }
    }

    mod when_q_is_pressed {
        use super::*;

        #[test]
        fn from_main_menu_then_quit() {
            let mut main_view = MainView::default();

            assert_eq!(main_view.quit, false);

            main_view.handle_key(KeyCode::Char('q'));

            assert_eq!(main_view.quit, true);

            main_view.handle_key(KeyCode::Char('q'));

            assert_eq!(main_view.quit, true);
        }

        #[test]
        fn from_not_main_menu_return_then_quit() {
            let mut main_view = MainView::default();

            assert_eq!(main_view.quit, false);
            assert_eq!(main_view.router.route(), Route::MainMenu);

            main_view.handle_key(KeyCode::Char('l'));

            assert_eq!(main_view.quit, false);
            assert_eq!(main_view.router.route(), Route::Logger);

            main_view.handle_key(KeyCode::Char('q'));

            assert_eq!(main_view.quit, false);
            assert_eq!(main_view.router.route(), Route::MainMenu);

            main_view.handle_key(KeyCode::Char('q'));

            assert_eq!(main_view.quit, true);
        }
    }
}
