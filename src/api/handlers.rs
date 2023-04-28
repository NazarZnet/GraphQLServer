use actix_session::Session;
use actix_web::{get, post, web, HttpResponse};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use log::{error, info};
use serde_json::json;

use time::OffsetDateTime;
use uuid::Uuid;

use validator::Validate;

use crate::{
    api::create_context,
    errors::{AppError, AppErrorType},
    schemas::{AppData, LogInUser, SignUpUser, UserAuth},
};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[get("/")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}


#[get("/graphiql")]
async fn playground() -> HttpResponse {
    let html = graphiql_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[post("graphql")]
async fn graphql(
    data: web::Data<AppData>,
    graphreq: web::Json<GraphQLRequest>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    let user_id = session.get::<Uuid>("SessionId")?;
    if user_id.is_none() {
        error!("Can not find sessions id. Authorization failed!");
        return Err(AppError::new(None, None, AppErrorType::Authorization));
    }

    let context = create_context(data.db.clone(), user_id.unwrap());

    let res = graphreq.execute(&data.schema, &context).await;
    match res.is_ok() {
        true => Ok(HttpResponse::Ok().json(res)),
        false => {
            error!("GraphQl error!");
            Ok(HttpResponse::BadRequest().json(res))
        }
    }
}

#[post("signup")]
async fn signup(
    data: web::Data<AppData>,
    user: web::Json<SignUpUser>,
) -> Result<HttpResponse, AppError> {
    user.validate()?;

    let user_id = Uuid::new_v4();

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(user.password.as_bytes(), &salt)
        .map_err(|e| {
            error!("Password hashing failed!");
            AppError::new(None, Some(e.to_string()), AppErrorType::InvalidField)
        })?
        .to_string();

    if let Err(e) = sqlx::query!(
        "insert into users(id,name,username,password_hash) values ($1,$2,$3,$4);",
        user_id,
        user.name,
        user.username,
        password_hash
    )
    .execute(&data.db)
    .await
    {
        error!("Signup failed! Insert query error!");
        return Err(e.into());
    }

    Ok(HttpResponse::Ok().json(json!({
    "status":"success",
    "message":"Now you can log in",
    "user_id":user_id
    })))
}

#[post("login")]
async fn login(
    data: web::Data<AppData>,
    sessions: Session,
    log_user: web::Json<LogInUser>,
) -> Result<HttpResponse, AppError> {
    let user = sqlx::query_as!(
        UserAuth,
        "select id,username,password_hash,logged_at from users where username=$1",
        log_user.username
    )
    .fetch_one(&data.db)
    .await
    .map_err(|_e| {
        error!("Can not find a user with username: {}", log_user.username);
        AppError::new(None, None, AppErrorType::Authentification)
    })?;

    //check if password correct
    let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|e| {
        error!("Can not create password hash from user's password!");
        AppError::new(Some(e.to_string()), None, AppErrorType::DbError)
    })?;
    if Argon2::default()
        .verify_password(log_user.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        error!("Authentification failed! Incorrect password");
        return Err(AppError::new(None, None, AppErrorType::Authentification));
    }
    let now = OffsetDateTime::now_utc();

    if let Err(e) = sqlx::query!("update users set logged_at=$1 where id=$2", now, user.id)
        .execute(&data.db)
        .await
    {
        error!("User's logged time updation failed!");
        return Err(e.into());
    }

    sessions.insert("SessionId", user.id).map_err(|e| {
        error!("Session inser error!");
        e
    })?;
    sessions.renew();

    info!("Log in successful, username: {}", user.username);
    Ok(HttpResponse::Ok().json(json!({
        "status":"success",
        "message":format!("Welcome, {}",user.username)
    })))
}

#[get("logout")]
async fn logout(sessions: Session) -> Result<HttpResponse, AppError> {
    if sessions.remove("SessionId").is_none() {
        error!("Can not find session id to log out!");
        return Err(AppError::new(
            Some("Can not find session if to log out".to_string()),
            None,
            AppErrorType::NotFoundError,
        ));
    }
    Ok(HttpResponse::Ok().body("Log out successful"))
}
