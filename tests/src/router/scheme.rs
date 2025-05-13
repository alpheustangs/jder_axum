use jder_axum::{
    extract::extra::Scheme,
    response::{Response, json::CreateJsonResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RouteSchemeResponseData {
    pub scheme: String,
}

pub async fn route_scheme(Scheme(scheme): Scheme) -> Response {
    CreateJsonResponse::success::<RouteSchemeResponseData>()
        .data(RouteSchemeResponseData { scheme })
        .send()
}
