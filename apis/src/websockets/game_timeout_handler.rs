use crate::{
    common::server_result::{InternalServerMessage, MessageDestination, ServerMessage},
    responses::game::GameResponse,
};
use anyhow::Result;
use db_lib::{models::game::Game, DbPool};
use uuid::Uuid;

#[allow(dead_code)]
pub struct GameTimeoutHandler {
    game: Game,
    nanoid: String,
    username: String,
    user_id: Uuid,
    pool: DbPool,
}

impl GameTimeoutHandler {
    pub async fn new(nanoid: &str, username: &str, user_id: Uuid, pool: &DbPool) -> Result<Self> {
        let game = Game::find_by_nanoid(nanoid, pool).await?;
        Ok(Self {
            game,
            nanoid: nanoid.to_owned(),
            username: username.to_owned(),
            user_id,
            pool: pool.clone(),
        })
    }

    pub async fn handle(&self) -> Result<Vec<InternalServerMessage>> {
        let mut messages = Vec::new();
        let game = self.game.check_time(&self.pool).await?;
        messages.push(InternalServerMessage {
            destination: MessageDestination::Game(self.game.nanoid.clone()),
            message: ServerMessage::GameTimeoutCheck(
                GameResponse::new_from_db(&game, &self.pool).await?,
            ),
        });
        Ok(messages)
    }
}
