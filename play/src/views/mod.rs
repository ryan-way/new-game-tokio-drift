use crate::terminal::Frame;
use crate::utils::Command;

mod counter;
mod logger;
mod main_menu;

use crate::services::Router;
use crate::utils::Route;
use crate::view_models::MainVm;

use self::counter::CounterView;
use self::logger::LoggerView;
use self::main_menu::MainMenuView;

pub struct MainView {
    router: Router,
    counter: CounterView,
    main_menu: MainMenuView,
    logger: LoggerView,
    quit: bool,
}

impl MainView {
    pub fn new() -> Self {
        Self {
            router: Router::default(),
            counter: CounterView::new(),
            main_menu: MainMenuView::new(),
            logger: LoggerView::new(),
            quit: false,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, main_vm: &mut dyn MainVm) {
        match self.router.route() {
            Route::Counter => {
                let counter_vm = main_vm.counter();
                self.counter.draw(frame, counter_vm)
            }
            Route::MainMenu => self.main_menu.draw(frame),
            Route::Logger => self.logger.draw(frame),
        }
    }

    pub async fn handle_key(&mut self, command: Command, main_vm: &mut dyn MainVm) {
        match command {
            Command::Quit => match self.router.route() {
                Route::MainMenu => self.quit = true,
                _ => self.router.set_route(Route::MainMenu),
            },
            Command::Logger => match self.router.route() {
                Route::Logger => self.router.previous(),
                _ => self.router.set_route(Route::Logger),
            },
            _ => self.routed_handle_key(command, main_vm).await,
        }
    }

    async fn routed_handle_key(&mut self, command: Command, main_vm: &mut dyn MainVm) {
        match self.router.route() {
            Route::Counter => {
                let counter_vm = main_vm.counter();
                self.counter
                    .handle_key(command, counter_vm)
                    .await
                    .expect("Counter failed to handled key");
            }
            Route::MainMenu => self.main_menu.handle_key(command, &mut self.router),
            Route::Logger => self.logger.handle_key(command, &mut self.router),
        }
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::view_models::MockMainVm;

    mod when_created {

        use super::*;

        #[test]
        fn route_is_main_menu() {
            let main_view = MainView::new();

            assert_eq!(main_view.router.route(), Route::MainMenu);
        }
    }

    mod when_o_is_pressed {
        use super::*;

        #[tokio::test]
        async fn route_toggles_logger() {
            let mut mock = MockMainVm::new();
            let mut main_view = MainView::new();

            assert_eq!(main_view.router.route(), Route::MainMenu);

            main_view.handle_key(Command::Logger, &mut mock).await;

            assert_eq!(main_view.router.route(), Route::Logger);

            main_view.handle_key(Command::Logger, &mut mock).await;

            assert_eq!(main_view.router.route(), Route::MainMenu);
        }
    }

    mod when_q_is_pressed {
        use super::*;

        #[tokio::test]
        async fn from_main_menu_then_quit() {
            let mut mock = MockMainVm::new();
            let mut main_view = MainView::new();

            assert_eq!(main_view.quit, false);

            main_view.handle_key(Command::Quit, &mut mock).await;

            assert_eq!(main_view.quit, true);

            main_view.handle_key(Command::Quit, &mut mock).await;

            assert_eq!(main_view.quit, true);
        }

        #[tokio::test]
        async fn from_not_main_menu_return_then_quit() {
            let mut mock = MockMainVm::new();
            let mut main_view = MainView::new();

            assert_eq!(main_view.quit, false);
            assert_eq!(main_view.router.route(), Route::MainMenu);

            main_view.handle_key(Command::Logger, &mut mock).await;

            assert_eq!(main_view.quit, false);
            assert_eq!(main_view.router.route(), Route::Logger);

            main_view.handle_key(Command::Quit, &mut mock).await;

            assert_eq!(main_view.quit, false);
            assert_eq!(main_view.router.route(), Route::MainMenu);

            main_view.handle_key(Command::Quit, &mut mock).await;

            assert_eq!(main_view.quit, true);
        }
    }
}
