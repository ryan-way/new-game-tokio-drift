use crate::utils::Route;

#[derive(Default, Clone, Copy)]
pub struct Router {
    previous: Route,
    current: Route,
}

impl Router {
    pub fn set_route(&mut self, route: Route) {
        self.previous = self.current;
        self.current = route;
    }

    pub fn route(&self) -> Route {
        self.current
    }

    pub fn previous(&mut self) {
        self.current = self.previous;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_is_main_menu() {
        let router = Router::default();
        assert_eq!(router.route(), Route::MainMenu);
    }

    #[test]
    fn previous_does_nothing() {
        let mut router = Router::default();
        router.previous();
        assert_eq!(router.route(), Route::MainMenu);
    }

    #[test]
    fn route_is_updated() {
        let mut router = Router::default();
        router.set_route(Route::Counter);
        assert_eq!(router.route(), Route::Counter);
    }
}
