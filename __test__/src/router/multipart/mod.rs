pub mod file;

use axum_typed_multipart::TryFromMultipart;
use jder_axum::{
    extract::multipart::Multipart,
    response::{json::CreateJsonResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(TryFromMultipart)]
pub struct RouteMultipartData {
    string: Option<String>,
    number: Option<usize>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct RouteMultipartResponseData {
    pub string: Option<String>,
    pub number: Option<usize>,
}

pub async fn route_multipart(data: Multipart<RouteMultipartData>) -> Response {
    CreateJsonResponse::success::<RouteMultipartResponseData>()
        .data(RouteMultipartResponseData {
            string: data.string.clone(),
            number: data.number,
        })
        .send()
}
