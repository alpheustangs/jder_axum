use jder_axum::{
    extract::MatchedPath,
    response::{Response, json::CreateJsonResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RouteMatchedPathResponseData {
    pub path: String,
}

pub async fn route_matched_path(path: MatchedPath) -> Response {
    CreateJsonResponse::success::<RouteMatchedPathResponseData>()
        .data(RouteMatchedPathResponseData { path: path.as_str().to_string() })
        .send()
}
