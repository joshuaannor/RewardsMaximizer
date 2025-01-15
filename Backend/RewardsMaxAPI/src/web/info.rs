use askama::Template;
use axum::{routing::get, response::IntoResponse, Router};

//use crate::users::AuthSession;

pub fn router() -> Router<()> {
    Router::new()
    .route("/contactUs", get(self::get::contact_us))
    .route("/aboutUs", get(self::get::about_us))
    .route("/suggestions", get(self::get::suggestions))
    .route("/team", get(self::get::team_info))
}

mod get {
    //use super::*;
    
    pub async fn contact_us() -> &'static str {
        "Contact us endpoint"
    }

    pub async fn about_us() -> &'static str {
        "About us endpoint"
    }

    pub async fn suggestions() -> &'static str {
        "Suggestions endpoint"
    }

    pub async fn team_info() -> &'static str {
        "Team info endpoint"
    }
}