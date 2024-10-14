use std::net::SocketAddr;

use jder_axum::{
    extract::ConnectInfo,
    response::{json::CreateJsonResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RouteConnectInfoResponseData {
    pub addr: String,
}

pub async fn route_connect_info(
    ConnectInfo(addr): ConnectInfo<SocketAddr>
) -> Response {
    CreateJsonResponse::success::<RouteConnectInfoResponseData>()
        .data(RouteConnectInfoResponseData { addr: addr.ip().to_string() })
        .send()
}
