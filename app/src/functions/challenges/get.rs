use super::challenge_response::ChallengeResponse;
use leptos::*;
use uuid::Uuid;

#[server]
pub async fn get_challenge(id: Uuid) -> Result<ChallengeResponse, ServerFnError> {
    use crate::functions::db::pool;
    use db_lib::models::challenge::Challenge;
    let pool = pool()?;
    let challenge = Challenge::get(&id, &pool).await?;
    ChallengeResponse::from_model(&challenge, &pool).await
}
