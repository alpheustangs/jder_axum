use axum::http::StatusCode;
use jder_axum::{
    extract::Path,
    response::{CreateJsonResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RoutePathResponseData {
    pub id: usize,
    pub name: String,
}

pub async fn route_path(Path((id, name)): Path<(usize, String)>) -> Response {
    CreateJsonResponse::success::<RoutePathResponseData>()
        .status(StatusCode::CREATED)
        .data(RoutePathResponseData { id, name })
        .send()
}
