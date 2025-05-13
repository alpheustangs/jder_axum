use jder_axum::{
    extract::query::{Query, empty_as_none},
    response::{Response, json::CreateJsonResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RouteQueryData {
    #[serde(default, deserialize_with = "empty_as_none")]
    num: Option<usize>,
    empty: Option<String>,
    #[serde(default, deserialize_with = "empty_as_none")]
    none: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RouteQueryResponseData {
    pub num: Option<usize>,
    pub empty: Option<String>,
    pub none: Option<String>,
}

pub async fn route_query(Query(query): Query<RouteQueryData>) -> Response {
    CreateJsonResponse::success::<RouteQueryResponseData>()
        .data(RouteQueryResponseData {
            num: query.num,
            empty: query.empty,
            none: query.none,
        })
        .send()
}
