use crate::{
    common::{
        game_reaction::GameReaction,
        server_result::{ChallengeUpdate, GameActionResponse, GameUpdate, ServerMessage},
    },
    responses::game::GameResponse,
    websockets::internal_server_message::{InternalServerMessage, MessageDestination},
};
use anyhow::Result;
use db_lib::{
    models::challenge::Challenge,
    models::game::{Game, NewGame},
    DbPool,
};

use uuid::Uuid;

pub struct AcceptHandler {
    nanoid: String,
    user_id: Uuid,
    username: String,
    pool: DbPool,
}

impl AcceptHandler {
    pub async fn new(nanoid: String, username: &str, user_id: Uuid, pool: &DbPool) -> Result<Self> {
        Ok(Self {
            nanoid,
            user_id,
            username: username.to_owned(),
            pool: pool.clone(),
        })
    }

    pub async fn handle(&self) -> Result<Vec<InternalServerMessage>> {
        let challenge = Challenge::find_by_nanoid(&self.nanoid, &self.pool).await?;
        let (white_id, black_id) = match challenge.color_choice.to_lowercase().as_str() {
            "black" => (self.user_id, challenge.challenger_id),
            "white" => (challenge.challenger_id, self.user_id),
            _ => {
                if rand::random() {
                    (challenge.challenger_id, self.user_id)
                } else {
                    (self.user_id, challenge.challenger_id)
                }
            }
        };

        let new_game = NewGame::new(white_id, black_id, &challenge);
        let (game, deleted_challenges) = Game::create(&new_game, &self.pool).await?;
        let mut messages = Vec::new();
        let game_response = GameResponse::new_from_db(&game, &self.pool).await?;

        messages.push(InternalServerMessage {
            destination: MessageDestination::User(game.white_id),
            message: ServerMessage::Game(GameUpdate::Reaction(GameActionResponse {
                game_action: GameReaction::New,
                game: game_response.clone(),
                game_id: game_response.nanoid.clone(),
                user_id: self.user_id,
                username: self.username.to_owned(),
            })),
        });

        messages.push(InternalServerMessage {
            destination: MessageDestination::User(game.black_id),
            message: ServerMessage::Game(GameUpdate::Reaction(GameActionResponse {
                game_action: GameReaction::New,
                game: game_response.clone(),
                game_id: game_response.nanoid.clone(),
                user_id: self.user_id,
                username: self.username.to_owned(),
            })),
        });

        for challenge_nanoid in deleted_challenges {
            messages.push(InternalServerMessage {
                destination: MessageDestination::Global,
                message: ServerMessage::Challenge(ChallengeUpdate::Removed(challenge_nanoid)),
            });
        }
        Ok(messages)
    }
}
