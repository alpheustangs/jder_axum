use axum::{
    body::Bytes,
    http::{StatusCode, header},
};
use axum_typed_multipart::{FieldData, TryFromMultipart};
use jder_axum::{
    extract::multipart::TypedMultipart,
    response::{CreateResponse, Response, json::CreateJsonResponse},
};

#[derive(Debug, TryFromMultipart)]
pub struct RouteMultipartFileData {
    #[form_data(limit = "10MiB")]
    image: Option<FieldData<Bytes>>,
}

#[axum::debug_handler]
pub async fn route_multipart_file(
    data: TypedMultipart<RouteMultipartFileData>
) -> Response {
    match &data.image {
        | Some(image) => {
            return CreateResponse::success()
                .header(
                    header::CONTENT_TYPE,
                    match &image.metadata.content_type {
                        | Some(content_type) => content_type,
                        | None => "text/plain",
                    },
                )
                .body(image.contents.clone());
        },
        | None => {},
    }

    CreateJsonResponse::failure().status(StatusCode::NOT_FOUND).send()
}
