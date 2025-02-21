use jder_axum::{
    extract::Json,
    response::{Response, json::CreateJsonResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RouteJsonResponseData {
    pub id: Option<usize>,
    pub name: Option<String>,
}

pub async fn route_json(Json(data): Json<RouteJsonResponseData>) -> Response {
    CreateJsonResponse::success::<RouteJsonResponseData>()
        .data(RouteJsonResponseData { id: data.id, name: data.name })
        .send()
}
