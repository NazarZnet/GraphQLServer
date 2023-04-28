use juniper::FieldError;
use log::error;
use sqlx::PgPool;

use crate::{
    errors::{AppError, AppErrorType},
    schemas::User,
};

//get user from database
pub async fn db_get_user(name: &str, pool: &PgPool) -> Result<User, FieldError> {
    //find provided user
    let row = sqlx::query!("select id,name from users where name = $1", name)
        .fetch_one(pool)
        .await
        .map_err(|_e| {
            error!("Can not find a user named: {}", name);
            AppError::new(None, None, AppErrorType::NotFoundError)
        })?;

    //get user's friends
    let friends = sqlx::query_scalar!("select friend from friends where user_id=$1", row.id)
        .fetch_all(pool)
        .await;

    let user = User {
        id: row.id,
        name: row.name,
        friends: friends.unwrap_or_default(),
    };

    Ok(user)
}
