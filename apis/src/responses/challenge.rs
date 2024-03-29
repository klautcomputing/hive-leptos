use crate::common::challenge_action::ChallengeVisibility;
use crate::responses::user::UserResponse;
use chrono::prelude::*;
use hive_lib::color::ColorChoice;
use serde::{Deserialize, Serialize};
use shared_types::time_mode::TimeMode;
use std::str;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChallengeResponse {
    pub id: Uuid,
    pub nanoid: String,
    pub challenger: UserResponse,
    pub opponent: Option<UserResponse>,
    pub game_type: String,
    pub rated: bool,
    pub visibility: ChallengeVisibility,
    pub color_choice: ColorChoice,
    pub created_at: DateTime<Utc>,
    pub challenger_rating: f64,
    pub time_mode: TimeMode,         // Correspondence, Timed, Untimed
    pub time_base: Option<i32>,      // Secons
    pub time_increment: Option<i32>, // Seconds
}

use cfg_if::cfg_if;
cfg_if! { if #[cfg(feature = "ssr")] {
use std::str::FromStr;
use db_lib::{
    models::{challenge::Challenge, rating::Rating, user::User},
    DbPool,
};
use anyhow::Result;
impl ChallengeResponse {
    pub async fn from_model(challenge: &Challenge, pool: &DbPool) -> Result<Self> {
        let challenger = challenge.get_challenger(pool).await?;
        ChallengeResponse::from_model_with_user(challenge, challenger, pool).await
    }

    pub async fn from_model_with_user(
        challenge: &Challenge,
        challenger: User,
        pool: &DbPool,
    ) -> Result<Self> {
        let challenger_rating = Rating::for_uuid(&challenger.id, pool).await?;
        let opponent = match challenge.opponent_id {
            None => None,
            Some(id) => Some(UserResponse::from_uuid(&id, pool).await?),
        };
        Ok(ChallengeResponse {
            id: challenge.id,
            nanoid: challenge.nanoid.to_owned(),
            challenger: UserResponse::from_uuid(&challenger.id, pool).await?,
            opponent,
            game_type: challenge.game_type.clone(),
            rated: challenge.rated,
            visibility: ChallengeVisibility::from_str(&challenge.visibility)?,
            color_choice: ColorChoice::from_str(&challenge.color_choice)?,
            created_at: challenge.created_at,
            challenger_rating: challenger_rating.rating,
            time_mode: TimeMode::from_str(&challenge.time_mode)?,
            time_base: challenge.time_base,
            time_increment: challenge.time_increment,
        })
    }
}
}}
