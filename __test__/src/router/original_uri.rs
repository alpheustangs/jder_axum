use axum::http::Uri;
use jder_axum::{
    extract::OriginalUri,
    response::{Response, json::CreateJsonResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RouteOriginalUriResponseData {
    pub uri: String,
    pub original_uri: String,
}

pub async fn route_original_uri(
    uri: Uri,
    OriginalUri(path): OriginalUri,
) -> Response {
    CreateJsonResponse::success::<RouteOriginalUriResponseData>()
        .data(RouteOriginalUriResponseData {
            uri: uri.to_string(),
            original_uri: path.to_string(),
        })
        .send()
}
