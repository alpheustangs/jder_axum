use std::time::Duration;

use axum::{Router, routing::post};
use jder_axum::{
    layers::RequestTimeLimit,
    response::{Response, json::CreateJsonResponse},
};

async fn route_ok() -> Response {
    CreateJsonResponse::dataless().send()
}

async fn route_timeout() -> Response {
    tokio::time::sleep(Duration::from_secs(2)).await;
    CreateJsonResponse::dataless().send()
}

pub fn router_request_time_limit() -> Router {
    Router::new().merge(
        Router::new()
            .route("/ok", post(route_ok))
            .route("/timeout", post(route_timeout))
            .layer(RequestTimeLimit::max(Duration::from_secs(1))),
    )
}
