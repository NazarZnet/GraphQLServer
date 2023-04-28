use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use time::{serde::rfc3339, OffsetDateTime};
use uuid::Uuid;

use validator::Validate;

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct FullUser {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    #[graphql(skip)]
    pub password_hash: String,
    #[serde(with = "rfc3339")]
    pub logged_at: OffsetDateTime,
    pub friends: Vec<String>,
}

#[derive(Serialize, Deserialize, GraphQLObject)]
#[graphql(description = "A User object")]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub friends: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct UserAuth {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    #[serde(with = "rfc3339")]
    pub logged_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct LogInUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct SignUpUser {
    #[validate(length(min = 4))]
    pub name: String,
    #[validate(length(min = 4))]
    pub username: String,
    #[validate(length(min = 8))]
    pub password: String,
}
