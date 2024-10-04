pub mod connect_info;
pub mod host;
pub mod json;
pub mod matched_path;
pub mod multipart;
pub mod nested_path;
pub mod original_uri;
pub mod path;
pub mod query;
pub mod state;

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{connect_info::IntoMakeServiceWithConnectInfo, DefaultBodyLimit},
    routing::{get, post},
    Router,
};
use axum_test::TestServer;
use jder_axum::response::{CreateJsonResponse, Response};
use json::route_json;
use matched_path::route_matched_path;
use nested_path::route_nested_path;
use original_uri::route_original_uri;
use state::{route_state, AppState};

use crate::router::connect_info::route_connect_info;
use crate::router::host::route_host;
use crate::router::multipart::file::route_multipart_file;
use crate::router::multipart::route_multipart;
use crate::router::path::route_path;
use crate::router::query::route_query;

pub async fn route_index() -> Response {
    CreateJsonResponse::dataless().send()
}

/// Create router for the app
pub fn create_router() -> IntoMakeServiceWithConnectInfo<Router, SocketAddr> {
    let app_state: AppState = AppState { view: Arc::new(Mutex::new(0)) };

    Router::new()
        .route("/", get(route_index))
        .route("/connect_info", post(route_connect_info))
        .route("/host", post(route_host))
        .route("/json", post(route_json))
        .route("/matched_path", post(route_matched_path))
        .route("/multipart", post(route_multipart))
        .route("/multipart/file", post(route_multipart_file))
        .route("/nested_path", post(route_nested_path))
        .route("/original_uri", post(route_original_uri))
        .nest(
            "/:id",
            Router::new()
                .route("/nested_path", post(route_nested_path))
                .route("/original_uri", post(route_original_uri)),
        )
        .route("/path/:id/:name", post(route_path))
        .route("/query", post(route_query))
        .route("/state", post(route_state))
        .layer(DefaultBodyLimit::disable())
        .with_state(app_state)
        .into_make_service_with_connect_info::<SocketAddr>()
}

#[allow(dead_code)]
pub fn create_server() -> TestServer {
    TestServer::new(create_router()).unwrap()
}
