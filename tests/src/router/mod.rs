pub mod connect_info;
pub mod form;
pub mod host;
pub mod json;
pub mod matched_path;
pub mod multipart;
pub mod nested_path;
pub mod path;
pub mod query;
pub mod scheme;
pub mod typed_header;

use std::net::SocketAddr;

use axum::{
    Router,
    extract::{DefaultBodyLimit, connect_info::IntoMakeServiceWithConnectInfo},
    routing::{get, post},
};
use axum_test::TestServer;
use jder_axum::{
    layers::RequestBodyLimit,
    response::{Response, json::CreateJsonResponse},
};

use crate::router::connect_info::route_connect_info;
use crate::router::form::route_form;
use crate::router::host::route_host;
use crate::router::json::{optional::route_json_optional, route_json};
use crate::router::matched_path::route_matched_path;
use crate::router::multipart::file::route_multipart_file;
use crate::router::multipart::route_multipart;
use crate::router::nested_path::route_nested_path;
use crate::router::path::route_path;
use crate::router::query::route_query;
use crate::router::scheme::route_scheme;
use crate::router::typed_header::{
    optional::route_typed_header_optional, route_typed_header,
};

pub async fn route_index() -> Response {
    CreateJsonResponse::dataless().send()
}

/// Create router for the app
pub fn create_router() -> IntoMakeServiceWithConnectInfo<Router, SocketAddr> {
    Router::new()
        .route("/", get(route_index))
        .route("/connect_info", post(route_connect_info))
        .route("/host", post(route_host))
        .route("/form", post(route_form))
        .route("/json", post(route_json))
        .route("/json/optional", post(route_json_optional))
        .route("/matched_path", post(route_matched_path))
        .route("/multipart", post(route_multipart))
        .route("/multipart/file", post(route_multipart_file))
        .route("/nested_path", post(route_nested_path))
        .nest(
            "/{id}",
            Router::new().route("/nested_path", post(route_nested_path)),
        )
        .route("/path/{id}/{name}", post(route_path))
        .route("/query", post(route_query))
        .route("/scheme", post(route_scheme))
        .route("/typed_header", post(route_typed_header))
        .route("/typed_header/optional", post(route_typed_header_optional))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimit::max(10 * 1024 * 1024))
        .into_make_service_with_connect_info::<SocketAddr>()
}

#[allow(dead_code)]
pub fn create_server() -> TestServer {
    TestServer::new(create_router()).unwrap()
}
