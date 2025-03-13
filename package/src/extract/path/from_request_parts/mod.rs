pub mod optional;

use axum::{
    extract::{
        path::ErrorKind, rejection::PathRejection, FromRequestParts,
        Path as _Path,
    },
    http::{request::Parts, StatusCode},
};
use serde::de::DeserializeOwned;

use crate::{
    extract::path::Path,
    response::{
        json::{
            error::JsonResponseErrorCode,
            failure::JsonFailureResponseFunctions, CreateJsonResponse,
        },
        Response,
    },
};

pub(crate) fn match_path_rejection(rejection: PathRejection) -> Response {
    match rejection {
        | PathRejection::FailedToDeserializePathParams(inner) => {
            let kind: ErrorKind = inner.into_kind();

            let status: StatusCode = match kind {
                | ErrorKind::UnsupportedType { .. } => {
                    StatusCode::INTERNAL_SERVER_ERROR
                },
                | _ => StatusCode::BAD_REQUEST,
            };

            let field: Option<String> = match &kind {
                | ErrorKind::ParseErrorAtKey { key, .. } => Some(key.clone()),
                | ErrorKind::ParseErrorAtIndex { index, .. } => {
                    Some(index.to_string())
                },
                | ErrorKind::InvalidUtf8InPathParam { key } => {
                    Some(key.clone())
                },
                | _ => None,
            };

            let res: JsonFailureResponseFunctions<()> =
                CreateJsonResponse::failure()
                    .status(status)
                    .error_code(JsonResponseErrorCode::Parse.as_str())
                    .error_message(kind.to_string());

            match field {
                | Some(field) => res.error_field(&field).send(),
                | None => res.send(),
            }
        },
        | PathRejection::MissingPathParams(inner) => {
            CreateJsonResponse::failure()
                .status(inner.status())
                .error_code(JsonResponseErrorCode::Parse.as_str())
                .error_message(inner.body_text())
                .send()
        },
        | _ => CreateJsonResponse::failure()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .error_code(JsonResponseErrorCode::Server.as_str())
            .error_message(rejection.body_text())
            .send(),
    }
}

impl<T, S> FromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match _Path::<T>::from_request_parts(parts, state).await {
            | Ok(value) => Ok(Self(value.0)),
            | Err(rejection) => Err(match_path_rejection(rejection)),
        }
    }
}
