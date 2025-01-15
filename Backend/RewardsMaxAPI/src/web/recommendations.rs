// use axum::{extract::Path, response::Json};
// //use diesel::r2d2::{ConnectionManager, Pool}; // Commented in order to compile
// //use crate::models::{Reward, UserFeedback};
// use crate::schema::{user_rewards, rewards, user_feedback}; 
// use diesel::prelude::*;

// pub async fn get_recommendations(
//     Path(user_id): Path<i32>,
//     pool: Pool<ConnectionManager<SqliteConnection>>,
// ) -> Json<Vec<Reward>> {
//     let conn = pool.get().expect("Failed to get DB connection");

//     // Get user rewards or recommendations logic
//     let rewards = get_recommendations_for_user(user_id, &conn);

//     Json(rewards)
// }

// fn get_recommendations_for_user(user_id: i32, conn: &SqliteConnection) -> Vec<Reward> {
//     // Get user feedback to determine interests
//     let feedbacks: Vec<UserFeedback> = user_feedback::table
//         .filter(user_feedback::user_id.eq(user_id))
//         .load::<UserFeedback>(conn)
//         .expect("Error loading user feedback");

//     let mut recommended_company_ids: Vec<i32> = Vec::new();
//     for feedback in feedbacks {
//         if feedback.rating >= 4 {
//             recommended_company_ids.push(feedback.company_id);
//         }
//     }

//     // Get rewards associated with the recommended companies
//     let recommended_rewards: Vec<Reward> = rewards::table
//         .filter(rewards::company_id.eq_any(recommended_company_ids))
//         .load::<Reward>(conn)
//         .expect("Error loading rewards");

//     recommended_rewards
// }
