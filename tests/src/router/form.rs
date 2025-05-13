use jder_axum::{
    extract::Form,
    response::{Response, json::CreateJsonResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RouteFormResponseData {
    pub id: Option<usize>,
    pub name: Option<String>,
}

#[axum::debug_handler]
pub async fn route_form(Form(data): Form<RouteFormResponseData>) -> Response {
    CreateJsonResponse::success::<RouteFormResponseData>()
        .data(RouteFormResponseData { id: data.id, name: data.name })
        .send()
}
