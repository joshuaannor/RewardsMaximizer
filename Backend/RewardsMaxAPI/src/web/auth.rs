use askama::Template;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use axum_messages::{Message, Messages};
use serde::Deserialize;

use crate::users::{AuthSession, Credentials};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    messages: Vec<Message>,
    next: Option<String>,
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTemplate {
    messages: Vec<Message>,
    next: Option<String>,
}

// This allows us to extract the "next" field from the query string. We use this
// to redirect after log in.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(self::post::login))
        .route("/login", get(self::get::login))
        .route("/logout", get(self::get::logout))
}

mod post {
    use super::*;

    pub async fn login(
        auth_session: AuthSession,
        messages: Messages,
        Json(creds): Json<Credentials>,
    ) -> impl IntoResponse {
        // Authenticate the user
        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,               // User authenticated successfully
            Ok(None) => {                         // Authentication failed
                messages.error("Invalid credentials");
    
                let mut login_url = "/login".to_string();
                if let Some(next) = creds.next {
                    login_url = format!("{}?next={}", login_url, next);
                }
    
                return Redirect::to(&login_url).into_response();
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };
    
    
        messages.success(format!("Successfully logged in as {}", user.username));
    
        // Redirect to the specified next URL or home page
        let redirect_to = creds.next.as_deref().unwrap_or("/");
        Redirect::to(redirect_to).into_response()
    }
}

mod get {
    use super::*;

    pub async fn login(
        messages: Messages,
        Query(NextUrl { next }): Query<NextUrl>,
    ) -> LoginTemplate {
        LoginTemplate {
            messages: messages.into_iter().collect(),
            next,
        }
    }

    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.logout().await {
            Ok(_) => Redirect::to("/login").into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

}    

