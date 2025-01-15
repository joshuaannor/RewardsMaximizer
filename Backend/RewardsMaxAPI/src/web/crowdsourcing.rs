use axum::{routing::get, routing::post, Router, Json, response::IntoResponse};
use diesel::{prelude::*};
use crate::web::models::UserFeedback;
use crate::web::lib::establish_connection;

// #[derive(Template)]
// #[template(path = "crowdsourcing.html")]
// struct CrowdsourcingTemplate;

pub fn router() -> Router<()> {
    Router::new()
        // .route("/crowdsourcing", get(self::get::view_crowdsourcing))
        .route("/crowdsourcing", post(self::post::submit_crowdsourcing))
}
mod post {
    use super::*;

    pub async fn submit_crowdsourcing(Json(mut new_feedback): Json<UserFeedback>) -> impl IntoResponse {
        use crate::web::schema::user_feedback::dsl::*;
        let mut connection = establish_connection();
        
        match diesel::insert_into(user_feedback)
            .values(&new_feedback)
            .execute(&mut connection)
        {
            Ok(_) => (axum::http::StatusCode::OK, "User feedback inserted into database").into_response(),
            Err(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to insert user feedback: {}", e),
            )
            .into_response(),
        }
    }

}
// mod get {
//     use super::*;

//     pub async fn view_crowdsourcing() -> impl IntoResponse {
//         CrowdsourcingTemplate.into_response();
//     }
//  }
