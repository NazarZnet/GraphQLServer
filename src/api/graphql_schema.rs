use juniper::{graphql_object, EmptySubscription, FieldError};

use log::{error, info};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app::db_get_user,
    errors::{AppError, AppErrorType},
    schemas::{FullUser, User},
};

#[derive(Clone)]
pub struct Context {
    pub pool: PgPool,
    pub user_id: Uuid,
}
impl juniper::Context for Context {}

#[derive(Clone)]
pub struct Query;

#[graphql_object(context=Context)]
impl Query {
    pub async fn version() -> &'static str {
        "0.1"
    }
    pub async fn users(context: &Context) -> Result<Vec<User>, FieldError> {
        info!("Get all users!");
        let record = match sqlx::query!("select name from users;")
            .fetch_all(&context.pool)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                error!("Can not get users from db!");
                return Err(e.into());
            }
        };

        //create vector woth users;
        let mut users = Vec::with_capacity(record.len());
        for user in record {
            users.push(db_get_user(&user.name, &context.pool).await?);
        }

        Ok(users)
    }

    pub async fn user(name: String, context: &Context) -> Result<User, FieldError> {
        info!("Get user '{}' from db!", name);
        db_get_user(&name, &context.pool).await
    }

    pub async fn about_me(context: &Context) -> Result<FullUser, FieldError> {
        info!("About user request, user_id:'{}'", context.user_id);
        //find logged user
        let row = match sqlx::query!("select * from users where id = $1", context.user_id)
            .fetch_one(&context.pool)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                error!("Can not find a user whith the id '{}'!", context.user_id);
                return Err(e.into());
            }
        };

        //get user's friends
        let friends = sqlx::query_scalar!("select friend from friends where user_id=$1", row.id)
            .fetch_all(&context.pool)
            .await;

        let user = FullUser {
            id: row.id,
            name: row.name,
            username: row.username,
            password_hash: row.password_hash,
            logged_at: row.logged_at,
            friends: friends.unwrap_or_default(),
        };

        Ok(user)
    }
}

pub struct Mutation;

#[graphql_object(context=Context)]
impl Mutation {
    pub async fn add_friend(name: String, context: &Context) -> Result<User, FieldError> {
        info!(
            "Add friend {} to the user with the id: {}",
            name, context.user_id
        );
        //check if user with the name exist
        let friend = db_get_user(&name, &context.pool).await?;

        //check if it isn't the same user
        if friend.id == context.user_id {
            error!("Trying to add yourself as a friend");
            return Err(AppError::new(None, None, AppErrorType::InvalidField).into());
        }

        //check if provided friend already exist
        if sqlx::query!(
            "select * from friends where user_id=$1 and friend=$2",
            context.user_id,
            name
        )
        .fetch_optional(&context.pool)
        .await?
        .is_some()
        {
            error!("The user already has friend named: {}", name);
            return Err(AppError::new(
                Some("The user already has this friend!".to_string()),
                None,
                AppErrorType::InvalidField,
            )
            .into());
        }

        //insert new friend
        if let Err(e) = sqlx::query!(
            "insert into friends (user_id,friend) values($1,$2)",
            context.user_id,
            name
        )
        .execute(&context.pool)
        .await
        {
            error!("Insert query error!");
            return Err(e.into());
        }

        let user_name = match sqlx::query!("select name from users where id=$1", context.user_id)
            .fetch_one(&context.pool)
            .await
        {
            Ok(u) => u,
            Err(e) => {
                error!("Can not get the logged user name!");
                return Err(e.into());
            }
        };

        db_get_user(&user_name.name, &context.pool).await
    }

    async fn delete_friend(name: String, context: &Context) -> Result<User, FieldError> {
        //check if user with the name exist
        let _friend = db_get_user(&name, &context.pool).await?;

        //check if provided friend is a user's friend
        if sqlx::query!(
            "select * from friends where user_id=$1 and friend=$2",
            context.user_id,
            name
        )
        .fetch_optional(&context.pool)
        .await?
        .is_none()
        {
            error!("The user doesn't have friend named: {}", name);
            return Err(AppError::new(None, None, AppErrorType::InvalidField).into());
        }

        //delete the friend
        if let Err(e) = sqlx::query!(
            "delete from friends where user_id=$1 and friend=$2",
            context.user_id,
            name
        )
        .execute(&context.pool)
        .await
        {
            error!("Delete query error!");
            return Err(e.into());
        }

        let user_name = match sqlx::query!("select name from users where id=$1", context.user_id)
            .fetch_one(&context.pool)
            .await
        {
            Ok(u) => u,
            Err(e) => {
                error!("Can not get the logged user name!");
                return Err(e.into());
            }
        };

        db_get_user(&user_name.name, &context.pool).await
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}

pub fn create_context(pool: PgPool, user_id: Uuid) -> Context {
    Context { pool, user_id }
}
