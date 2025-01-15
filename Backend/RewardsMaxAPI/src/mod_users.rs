use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use diesel::prelude::*;
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use tokio::task;
use crate::web::schema::users;
use crate::web::models::{User};
use crate::web::lib::establish_connection;

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("user_id", &self.user_id)
            .field("username", &self.username)
            .field("email", &self.email)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.user_id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Backend;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Diesel error")]
    Diesel(#[from] diesel::result::Error),

    #[error("Task join error")]
    TaskJoin(#[from] task::JoinError),
}

//#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Error;

    // async fn authenticate(
    //     &self,
    //     creds: Self::Credentials,
    // ) -> Result<Option<User>, Self::Error> {
    //     let mut connection = establish_connection();

    //     let user = task::spawn_blocking(move || {
    //         use crate::web::schema::users::dsl::*;
    //         users
    //             .filter(username.eq(&creds.username))
    //             .first::<User>(&mut connection)
    //             .optional()
    //     })
    //     .await?;

    //     let authenticated_user = task::spawn_blocking(move || {
    //         //Ok(user.into_iter().filter(|user| verify_password(creds.password, &user.password).is_ok()))
    //     })
    //     .await?;

    //     //Ok(authenticated_user)
    // }

    // async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
    //     let mut connection = establish_connection();
    //     let user_id_value = *user_id;

    //     let user = task::spawn_blocking(move || {
    //         use crate::web::schema::users::dsl::*;
    //         users.filter(user_id.eq(user_id_value)).first::<User>(&mut connection).optional()
    //     })
    //     .await?;

    //    // Ok(user)
    // }
}

pub type AuthSession = axum_login::AuthSession<Backend>;