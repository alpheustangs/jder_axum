use jder_axum::{
    extract::query::{Query, optional_query},
    response::{Response, json::CreateJsonResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RouteQueryData {
    #[serde(default, deserialize_with = "optional_query")]
    page: Option<usize>,
    title: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RouteQueryResponseData {
    pub page: Option<usize>,
    pub title: Option<String>,
}

#[axum::debug_handler]
pub async fn route_query(query: Query<RouteQueryData>) -> Response {
    let query: RouteQueryData = query.0;
    CreateJsonResponse::success::<RouteQueryResponseData>()
        .data(RouteQueryResponseData { page: query.page, title: query.title })
        .send()
}
