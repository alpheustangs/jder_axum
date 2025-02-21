use jder_axum::{
    extract::query::{Query, empty_to_none},
    response::{Response, json::CreateJsonResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RouteQueryData {
    #[serde(default, deserialize_with = "empty_to_none")]
    page: Option<usize>,
    title: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RouteQueryResponseData {
    pub page: Option<usize>,
    pub title: Option<String>,
}

pub async fn route_query(query: Query<RouteQueryData>) -> Response {
    let query: RouteQueryData = query.0;
    CreateJsonResponse::success::<RouteQueryResponseData>()
        .data(RouteQueryResponseData { page: query.page, title: query.title })
        .send()
}
