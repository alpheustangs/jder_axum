use headers::UserAgent;
use jder_axum::{
    extract::extra::TypedHeader,
    response::{Response, json::CreateJsonResponse},
};

#[axum::debug_handler]
pub async fn route_typed_header_optional(
    data: Option<TypedHeader<UserAgent>>
) -> Response {
    if let Some(TypedHeader(data)) = data {
        CreateJsonResponse::success().data(data.to_string()).send()
    } else {
        CreateJsonResponse::dataless().send()
    }
}
