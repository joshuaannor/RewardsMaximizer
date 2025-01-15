use axum::{routing::get, routing::post, Router, Json, response::IntoResponse, extract::Path};
use diesel::{prelude::*};
use crate::web::models::NewUser;
use crate::web::models::User;
use crate::web::lib::establish_connection;
use password_auth::generate_hash;

pub fn router() -> Router<()> {
    Router::new()
        .route("/signup", post(self::post::signup_user))
        .route("/users/:username", get(self::get::get_user_id))
}

mod post {
    use super::*;

    pub async fn signup_user(Json(mut new_user): Json<NewUser>) -> impl IntoResponse {
        use crate::web::schema::users::dsl::*;
        let mut connection = establish_connection();

        let hashed_password = generate_hash(&new_user.password);
        new_user.password = hashed_password;

        match diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut connection)
        {
            Ok(_) => (axum::http::StatusCode::OK, "user inserted into the database").into_response(),
            Err(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to insert user: {}", e),
            )
            .into_response(),
        }
    }
}

mod get {
    use super::*;
    use axum::extract::Path;

    pub async fn get_user_id(Path(username): Path<String>) -> impl IntoResponse {
        use crate::web::schema::users::dsl::*;
        let mut connection = establish_connection();

        match users
            .filter(username.eq(&username))
            .select(User::as_select())
            .load::<User>(&mut connection)
            {
                Ok(results) => Ok(axum::Json(results).into_response()),
                Err(e) => Err((
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to get data: {}", e),
                )
                .into_response()),
            }
    }
}