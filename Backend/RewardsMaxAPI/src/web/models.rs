use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use crate::web::schema;

#[derive(Clone, Serialize, Deserialize, Insertable, Queryable, Selectable, FromRow)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub user_id: i32,
    pub password: String,
    pub username: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub username: String,
    pub created: String,
    pub updated: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::user_cards)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserCard {
    pub user_card_id: i32,
    pub added: String,
    pub card_id: i32,
    pub expires_on: String,
    pub updated: String,
    pub user_id: i32,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::cards)]
pub struct Card {
    pub card_id: i32,                // Primary key
    pub company_id: i32,             // Foreign key
    pub name: String,                // Non-nullable
    pub r#type: String,              // Non-nullable
    pub icon: Option<String>,        // Nullable
    pub color: Option<String>,       // Nullable
    pub benefits: Option<String>,    // Nullable
    pub category: String,            // Non-nullable
    pub rating: Option<i32>,         // Nullable
    pub created: String,             // Non-nullable
    pub updated: String,             // Non-nullable
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::cards)]
pub struct NewCard {
    pub company_id: i32,
    pub name: String,
    pub r#type: String,
    pub icon: Option<String>,        // Nullable
    pub color: Option<String>,       // Nullable
    pub benefits: Option<String>,    // Nullable
    pub category: String,
    pub rating: Option<i32>,      
    pub created: String,
    pub updated: String,
}

#[derive(QueryableByName, Serialize, Selectable)]
#[diesel(table_name = schema::companies)]
pub struct Company {
    pub company_id: i32,
    pub name: String,
    pub description: String,
    pub website: String,
    pub contact_email: String,
    pub created: String,
    pub updated: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = schema::companies)]
pub struct NewCompany {
    pub contact_email: String,
    pub description: String,
    pub name: String,
    pub website: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = schema::user_feedback)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserFeedback {
    pub feedback_id: i32,
    pub user_id: i32,
    pub company_id: i32,
    pub rating: i32,
    pub comments: String,
    pub created: String,
    pub updated: String,
}

#[derive(Queryable, Serialize)]
#[diesel(table_name = schema::vendor_deals)]
pub struct VendorDeal {
    pub deal_id: i32,
    pub company_id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
}


#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = schema::vendor_deals)]
pub struct NewVendorDeal {
    pub deal_id: i32,
    pub company_id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = schema::rewards)]
pub struct Rewards {
    pub reward_id: i32,
    pub company_id: i32,
    pub name: String,
    pub description: String,
    pub created: String,
    pub updated: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::comments)]
pub struct Comment {
    pub comment_id: i32,
    pub comment_info: String,
    pub created: String,
    pub entity_type: String,
    pub updated: String,
    pub user_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::notifications)]
pub struct Notification {
    pub notification_id: i32,
    pub created: String,
    pub message: String,
    pub r#type: String,
    pub updated: String,
    pub user_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::user_rewards)]
pub struct UserReward {
    pub user_reward_id: i32,
    pub added: String,
    pub expires_on: String,
    pub reward_id: i32,
    pub status: String,
    pub updated: String,
    pub user_id: i32,
}
