use axum::{routing::get, routing::post, Router, Json, response::IntoResponse,  http::StatusCode};
use diesel::{prelude::*};
use crate::web::models::{NewVendorDeal, VendorDeal, Rewards};
use crate::web::lib::establish_connection;


pub fn router() -> Router<()> {
    Router::new()
        //.route("/addVendorDeals", post(self::post::add_vendor_deals))
        //.route("/searchVendorDealsByCompany", post(self::get::search_vendor_deals_by_company))
        .route("/viewRewards", get(self::get::view_rewards))

}

mod get {
    use super::*;

    pub async fn search_vendor_deals_by_company(query: Option<String>) -> &'static str  {
        "View Vendor deals endpoint"
   /* 
    -> impl IntoResponse {
        use crate::web::schema::vendor_deals::dsl::*;
        let mut connection = establish_connection();


        
        let query1 = "SELECT companies.name, vendor_deals.title, vendor_deals.description FROM companies JOIN vendor_deals ON vendor_deals.company_id = companies.company_id";
        let query2 = match query {
            Some(search) => format!("{} WHERE companies.name LIKE '%{}%'", query1, search),
            None => query1.to_string(),
        };

    
        let results = sql_query(query2).load::<VendorDeal>(&mut connection);



        match results{
            Ok(vendor_deals_list) => (StatusCode::OK, Json(vendor_deals_list)).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to retrieve data: {}", e),
            )
            .into_response(),
        }
        */
    }

    pub async fn view_rewards() -> impl IntoResponse {
       use crate::web::schema::rewards::dsl::*;
        let mut connection = establish_connection();

        match rewards
        .select(Rewards::as_select())
        .load::<Rewards>(&mut connection)
        {
            Ok(results) => Ok(Json(results).into_response()),
            Err(e) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get data: {}", e),
            )
            .into_response()),
        }

    }
 }

mod post {

    use super::*;

    // pub async fn add_vendor_deals(Json(new_vendor_deal): Json<NewVendorDeal>) -> impl IntoResponse {
    //     use crate::web::schema::vendor_deals::dsl::*;
    //     let mut connection = establish_connection();

    //     match diesel::insert_into(vendor_deals)
    //         .values(&new_vendor_deal)
    //         .execute(&mut connection)
    //     {
    //         Ok(_) => (StatusCode::OK, "data inserted into the database").into_response(),
    //         Err(e) => (
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             format!("Failed to insert data: {}", e),
    //         )
    //         .into_response(),
    //     }
    // }
}