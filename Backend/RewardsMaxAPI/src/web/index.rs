use askama::Template;
use axum::{routing::get, response::IntoResponse, Router};

//use crate::users::AuthSession;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;


pub fn router() -> Router<()> {
    Router::new().route("/", get(self::get::index))
}

mod get {
    use super::*;
    
    pub async fn index() -> impl IntoResponse {
        IndexTemplate.into_response()
    }
}