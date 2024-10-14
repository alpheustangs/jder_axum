use jder_axum::{
    extract::Host,
    response::{json::CreateJsonResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RouteHostResponseData {
    pub host: String,
}

pub async fn route_host(host: Host) -> Response {
    CreateJsonResponse::success::<RouteHostResponseData>()
        .data(RouteHostResponseData { host: host.0 })
        .send()
}
