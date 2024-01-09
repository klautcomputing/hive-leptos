use lazy_static::lazy_static;
use leptos::{logging::log, *};
use regex::Regex;

use crate::providers::{api_requests::ApiRequests, game_state::GameStateSignal};
lazy_static! {
    static ref NANOID: Regex =
        Regex::new(r"/game/(?<nanoid>.*)").expect("This regex should compile");
}

#[derive(Clone, Debug, Copy)]
pub struct NavigationControllerSignal {
    pub signal: RwSignal<NavigationControllerState>,
}

impl Default for NavigationControllerSignal {
    fn default() -> Self {
        Self::new()
    }
}

impl NavigationControllerSignal {
    pub fn new() -> Self {
        Self {
            signal: create_rw_signal(NavigationControllerState::new()),
        }
    }

    pub fn update_nanoid(&mut self, nanoid: Option<String>) {
        log!(
            "Updating nanoid to: {}",
            nanoid.clone().unwrap_or(String::from("noting"))
        );
        self.signal.update(|s| s.nanoid = nanoid.clone());
        let api = ApiRequests::new();
        if let Some(game_id) = nanoid {
            let mut game_state = expect_context::<GameStateSignal>();
            api.join(game_id.clone());
            game_state.set_game_id(game_id);
        }
    }

    pub fn set_next_games(&mut self, games: Vec<String>) {
        self.signal.update(|s| s.next_games = games)
    }

    pub fn delete_from_next_games(&mut self, game: String) {
        self.signal.update(|s| s.next_games.retain(|s| *s != game))
    }
}

#[derive(Clone, Debug)]
pub struct NavigationControllerState {
    pub nanoid: Option<String>,
    pub next_games: Vec<String>,
}

impl NavigationControllerState {
    pub fn new() -> Self {
        Self {
            nanoid: None,
            next_games: Vec::new(),
        }
    }
}

impl Default for NavigationControllerState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn provide_navigation_controller() {
    provide_context(NavigationControllerSignal::new())
}
