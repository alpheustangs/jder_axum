use axum::{Router, routing::post};
use jder_axum::{
    layers::RequestBodyLimit,
    response::{Response, json::CreateJsonResponse},
};

async fn route_root() -> Response {
    CreateJsonResponse::dataless().send()
}

pub fn router_request_body_limit() -> Router {
    Router::new()
        .merge(
            Router::new()
                .route("/1mb", post(route_root))
                .layer(RequestBodyLimit::max(1024 * 1024)),
        )
        .merge(
            Router::new()
                .route("/10mb", post(route_root))
                .layer(RequestBodyLimit::max(10 * 1024 * 1024)),
        )
}
