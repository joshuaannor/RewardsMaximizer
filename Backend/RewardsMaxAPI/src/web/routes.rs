// Backend/RewardsMaxAPI/src/web/routes.rs
use axum::{routing::{post}, Json, Router, Extension};
use crate::web::models::{NewCard, Card}; // Your NewCard model here
use crate::web::db::DbManager;
use axum::http::StatusCode;

pub fn router() -> Router {
    Router::new()
        .route("/add_card", post(add_card)) // POST route for adding a new card
}

async fn add_card(
    Extension(db_manager): Extension<DbManager>, 
    Json(new_card): Json<NewCard>,  // Get the new card data from the frontend
) -> Result<impl IntoResponse, StatusCode> {
    let mut connection = db_manager.get_pool().get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insert the new card into the cards table using Diesel
    diesel::insert_into(schema::cards::table)
        .values(&new_card)
        .execute(&mut connection)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED) // Return a success status
}
