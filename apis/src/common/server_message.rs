use crate::common::game_action::GameAction;
use crate::responses::game::GameResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMessage {
    pub game_action: GameAction,
    pub game: GameResponse,
    pub game_id: String, // nanoid
    pub user_id: Uuid,
    pub username: String,
}

use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {

use db_lib::DbPool;
use leptos::ServerFnError;

impl ServerMessage {
    pub async fn new(game_id: &str, game_action: GameAction, user_id: &Uuid, username: &str, pool: &DbPool) -> Result<ServerMessage, ServerFnError> {
        Ok(ServerMessage {
            game_action,
            game: GameResponse::new_from_nanoid(game_id, pool).await.map_err(ServerFnError::new)?,
            game_id: game_id.to_owned(),
            user_id: user_id.to_owned(),
            username: username.to_owned(),
        })
    }
}

}}
