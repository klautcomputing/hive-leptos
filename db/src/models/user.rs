use crate::{
    db_error::DbError,
    get_conn,
    models::{game::Game, game_user::GameUser, rating::NewRating},
    schema::{
        games::{self, current_player_id, finished},
        ratings::{self, rating},
        users,
        users::dsl::email as email_field,
        users::dsl::password as password_field,
        users::dsl::updated_at,
        users::dsl::username as username_field,
        users::dsl::users as users_table,
    },
    DbPool,
};
use chrono::{DateTime, Utc};
use diesel::{
    query_dsl::BelongingToDsl, ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable,
    SelectableHelper,
};
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::rating::Rating;

const MAX_USERNAME_LENGTH: usize = 40;
const VALID_USERNAME_CHARS: &str = "-_";
fn valid_username_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || VALID_USERNAME_CHARS.contains(c)
}

fn validate_username(username: &str) -> Result<(), DbError> {
    if !username.chars().all(valid_username_char) {
        let reason = format!("invalid username characters: {:?}", username);
        return Err(DbError::InvalidInput {
            info: String::from("Username has invalid characters"),
            error: reason,
        });
    } else if username.len() > MAX_USERNAME_LENGTH {
        let reason = format!("username must be <= {} chars", MAX_USERNAME_LENGTH);
        return Err(DbError::InvalidInput {
            info: String::from("Username is too long."),
            error: reason,
        });
    }
    Ok(())
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl NewUser {
    pub fn new(username: &str, password: &str, email: &str) -> Result<Self, DbError> {
        validate_username(username)?;
        Ok(Self {
            username: username.to_owned(),
            password: password.to_owned(),
            email: email.to_owned(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(primary_key(id))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn create(new_user: &NewUser, pool: &DbPool) -> Result<User, DbError> {
        let connection = &mut get_conn(pool).await?;
        connection
            .transaction::<_, DbError, _>(|conn| {
                async move {
                    let user: User = diesel::insert_into(users::table)
                        .values(new_user)
                        .get_result(conn)
                        .await?;
                    let new_rating = NewRating::for_uuid(&user.id);
                    diesel::insert_into(ratings::table)
                        .values(&new_rating)
                        .execute(conn)
                        .await?;
                    Ok(user)
                }
                .scope_boxed()
            })
            .await
    }

    pub async fn edit(
        &self,
        new_password: &str,
        new_email: &str,
        pool: &DbPool,
    ) -> Result<User, DbError> {
        let conn = &mut get_conn(pool).await?;
        Ok(match (new_password.is_empty(), new_email.is_empty()) {
            (true, true) => users_table.find(&self.id).first(conn).await?,
            (true, false) => {
                diesel::update(self)
                    .set((email_field.eq(new_email), updated_at.eq(Utc::now())))
                    .get_result(conn)
                    .await?
            }
            (false, true) => {
                diesel::update(self)
                    .set((password_field.eq(new_password), updated_at.eq(Utc::now())))
                    .get_result(conn)
                    .await?
            }
            (false, false) => {
                diesel::update(self)
                    .set((
                        password_field.eq(new_password),
                        email_field.eq(new_email),
                        updated_at.eq(Utc::now()),
                    ))
                    .get_result(conn)
                    .await?
            }
        })
    }

    pub async fn find_by_uuid(uuid: &Uuid, pool: &DbPool) -> Result<User, DbError> {
        let conn = &mut get_conn(pool).await?;
        Ok(users_table.find(uuid).first(conn).await?)
    }

    pub async fn find_by_username(username: &str, pool: &DbPool) -> Result<User, DbError> {
        let conn = &mut get_conn(pool).await?;
        Ok(users_table
            .filter(username_field.eq(username))
            .first(conn)
            .await?)
    }

    pub async fn delete(&self, pool: &DbPool) -> Result<usize, DbError> {
        let conn = &mut get_conn(pool).await?;
        Ok(diesel::delete(users_table.find(&self.id))
            .execute(conn)
            .await?)
    }

    pub async fn get_games_with_notifications(&self, pool: &DbPool) -> Result<Vec<Game>, DbError> {
        let conn = &mut get_conn(pool).await?;
        Ok(GameUser::belonging_to(self)
            .inner_join(games::table)
            .select(Game::as_select())
            .filter(current_player_id.eq(self.id))
            .filter(finished.eq(false))
            .get_results(conn)
            .await?)
    }

    pub async fn get_urgent_nanoids(&self, pool: &DbPool) -> Result<Vec<String>, DbError> {
        let conn = &mut get_conn(pool).await?;
        Ok(GameUser::belonging_to(self)
            .inner_join(games::table)
            .select(Game::as_select())
            .filter(current_player_id.eq(self.id))
            .filter(finished.eq(false))
            .get_results(conn)
            .await?
            .into_iter()
            .map(|game| game.nanoid)
            .collect())
    }

    pub async fn get_games(&self, pool: &DbPool) -> Result<Vec<Game>, DbError> {
        let conn = &mut get_conn(pool).await?;
        Ok(GameUser::belonging_to(self)
            .inner_join(games::table)
            .select(Game::as_select())
            .get_results(conn)
            .await?)
    }

    pub async fn get_top_users(pool: &DbPool, limit: i64) -> Result<Vec<(User, Rating)>, DbError> {
        let conn = &mut get_conn(pool).await?;

        let result = users::table
            .inner_join(ratings::table)
            .filter(ratings::played.ne(0))
            .order_by(rating.desc())
            .limit(limit)
            .load::<(User, Rating)>(conn)
            .await?;

        Ok(result)
    }
}
