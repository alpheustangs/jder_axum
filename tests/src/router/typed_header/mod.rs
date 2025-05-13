pub mod optional;

use headers::UserAgent;
use jder_axum::{
    extract::extra::TypedHeader,
    response::{Response, json::CreateJsonResponse},
};

#[axum::debug_handler]
pub async fn route_typed_header(
    TypedHeader(data): TypedHeader<UserAgent>
) -> Response {
    CreateJsonResponse::success().data(data.to_string()).send()
}
