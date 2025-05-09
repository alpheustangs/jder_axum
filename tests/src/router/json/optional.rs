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

#[axum::debug_handler]
pub async fn route_json_optional(
    data: Option<Json<RouteJsonResponseData>>
) -> Response {
    if let Some(Json(data)) = data {
        CreateJsonResponse::success::<RouteJsonResponseData>()
            .data(RouteJsonResponseData { id: data.id, name: data.name })
            .send()
    } else {
        CreateJsonResponse::success::<RouteJsonResponseData>()
            .data(RouteJsonResponseData { id: None, name: None })
            .send()
    }
}
