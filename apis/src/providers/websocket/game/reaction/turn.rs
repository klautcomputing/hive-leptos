use crate::{
    common::server_result::GameActionResponse,
    providers::{
        game_state::GameStateSignal, games::GamesSignal,
        navigation_controller::NavigationControllerSignal, timer::TimerSignal,
    },
};
use hive_lib::turn::Turn;
use leptos::*;

pub fn handle_turn(turn: Turn, gar: GameActionResponse) {
    let mut games = expect_context::<GamesSignal>();
    games.own_games_add(gar.game.clone());
    let mut game_state = expect_context::<GameStateSignal>();
    let navigation_controller = expect_context::<NavigationControllerSignal>();
    let timer = expect_context::<TimerSignal>();
    if let Some(nanoid) = navigation_controller.signal.get_untracked().nanoid {
        if gar.game.nanoid == nanoid {
            timer.update_from(&gar.game);
            game_state.clear_gc();
            game_state.set_game_response(gar.game.clone());
            if game_state.signal.get_untracked().state.history.moves != gar.game.history {
                match turn {
                    Turn::Move(piece, position) => game_state.play_turn(piece, position),
                    _ => unreachable!(),
                };
            }
        }
    }
}
