use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{auth_middleware::UserData, AppState};

use super::models::{assignments, groups, SpotlightAssignment, SpotlightGroup};

#[derive(Serialize)]
pub struct Response {
    pub groups: Vec<SpotlightGroup>,
    pub assignments: Vec<SpotlightAssignment>,
}

#[derive(Deserialize)]
pub struct SpotlightQuery {
    pub search: String,
}

/// Gets the spotlight search results for the specific search query
#[get("/spotlight")]
pub async fn spotlight(
    query: web::Query<SpotlightQuery>,
    user: web::ReqData<UserData>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let user_data = user.into_inner();
    let db = &mut data.db.db.get().unwrap();
    let search = query.search.clone();

    let search_terms: Vec<&str> = search.split_whitespace().collect();
    let formatted_query = search_terms.join("&");

    let response = Response {
        groups: groups(&formatted_query, &user_data, db),
        assignments: assignments(&formatted_query, &user_data, db),
    };
    HttpResponse::Ok().json(response)
}
