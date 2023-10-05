use crate::schema::{games_users, games_users::dsl::games_users as games_users_table};
use crate::models::{game::Game, user::User};
use crate::{DbPool, get_conn};
use diesel::{prelude::*, result::Error, Identifiable, Insertable, Queryable};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

#[derive(Insertable, Identifiable, Selectable, Queryable, Associations, Debug, Clone)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Game))]
#[diesel(table_name = games_users)]
#[diesel(primary_key(game_id, user_id))]
pub struct GameUser {
    pub game_id: Uuid,
    pub user_id: Uuid,
}

impl GameUser {
    pub fn new(game_id: Uuid, user_id: Uuid) -> Self {
        Self { game_id, user_id }
    }

    pub async fn insert(&self, pool: &DbPool) -> Result<(), Error> {
        let conn = &mut get_conn(pool).await?;
        self.insert_into(games_users_table).execute(conn).await?;
        Ok(())
    }
}
