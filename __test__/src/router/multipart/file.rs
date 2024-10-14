use axum::{
    body::Bytes,
    http::{header, StatusCode},
};
use axum_typed_multipart::{FieldData, TryFromMultipart};
use jder_axum::{
    extract::multipart::Multipart,
    response::{json::CreateJsonResponse, CreateResponse, Response},
};

#[derive(TryFromMultipart)]
pub struct RouteMultipartFileData {
    #[form_data(limit = "10MiB")]
    image: Option<FieldData<Bytes>>,
}

pub async fn route_multipart_file(
    data: Option<Multipart<RouteMultipartFileData>>
) -> Response {
    match data {
        | Some(data) => match &data.image {
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
        },
        | None => {},
    }

    CreateJsonResponse::failure().status(StatusCode::NOT_FOUND).send()
}
