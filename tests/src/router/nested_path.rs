use jder_axum::{
    extract::NestedPath,
    response::{Response, json::CreateJsonResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RouteNestedPathResponseData {
    pub path: String,
}

pub async fn route_nested_path(path: NestedPath) -> Response {
    CreateJsonResponse::success::<RouteNestedPathResponseData>()
        .data(RouteNestedPathResponseData { path: path.as_str().to_string() })
        .send()
}
