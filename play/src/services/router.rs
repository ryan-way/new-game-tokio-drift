#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Route {
    #[default]
    MainMenu,
    Counter,
    Logger,
}

#[derive(Default, Clone, Copy)]
pub struct Router {
    previous: Route,
    current: Route,
    loaded: Route,
}

impl Router {
    pub fn set_route(&mut self, route: Route) {
        self.previous = self.current;
        self.current = route;
    }

    pub fn route(&self) -> Route {
        self.current
    }

    pub fn should_reroute(&self) -> bool {
        self.current != self.loaded
    }

    pub fn routed(&mut self) {
        self.loaded = self.current;
    }

    pub fn previous(&mut self) {
        self.current = self.previous;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod when_created {
        use super::*;

        #[test]
        fn route_is_main_menu() {
            let router = Router::default();
            assert_eq!(router.route(), Route::MainMenu);
        }

        #[test]
        fn should_reroute_is_false() {
            let router = Router::default();
            assert_eq!(router.should_reroute(), false);
        }

        #[test]
        fn previous_does_nothing() {
            let mut router = Router::default();
            router.previous();
            assert_eq!(router.route(), Route::MainMenu);
        }
    }

    mod when_route_is_set {
        use super::*;

        #[test]
        fn should_reroute_is_true() {
            let mut router = Router::default();
            router.set_route(Route::Counter);
            assert_eq!(router.should_reroute(), true);
        }
    }

    mod when_routed {
        use super::*;

        #[test]
        fn route_is_updated() {
            let mut router = Router::default();
            router.set_route(Route::Counter);
            router.routed();
            assert_eq!(router.route(), Route::Counter);
        }

        #[test]
        fn should_reroute_is_false() {
            let mut router = Router::default();
            router.set_route(Route::Counter);
            router.routed();
            assert_eq!(router.should_reroute(), false);
        }

        #[test]
        fn previous_restores_original_route() {
            let mut router = Router::default();
            router.set_route(Route::Counter);
            router.routed();
            router.set_route(Route::Logger);
            router.routed();
            router.previous();
            assert_eq!(router.route(), Route::Counter);
        }
    }
}
