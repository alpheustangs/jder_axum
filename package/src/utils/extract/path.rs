use axum::{
    async_trait,
    extract::{
        path::ErrorKind, rejection::PathRejection, FromRequestParts,
        Path as _Path,
    },
    http::{request::Parts, StatusCode},
};
use serde::de::DeserializeOwned;

use crate::utils::response::{
    json::{
        error::JsonResponseErrorCode, failure::JsonFailureResponseFunctions,
        CreateJsonResponse,
    },
    Response,
};

/// Extractor that parses path parameters.
///
/// Check [`Path`](axum::extract::Path) for more information.
///
/// ## Example
///
/// ```no_run
/// use jder_axum::extract::Path;
///
/// // /users/:id/:name
/// async fn route(
///     Path((id, name)): Path<(String, String)>,
/// ) {
///     // ...
/// }
/// ```
#[derive(Debug)]
pub struct Path<T>(pub T);

#[async_trait]
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
            | Err(rejection) => Err(match rejection {
                | PathRejection::FailedToDeserializePathParams(inner) => {
                    let kind: ErrorKind = inner.into_kind();

                    let status: StatusCode = match kind {
                        | ErrorKind::UnsupportedType { .. } => {
                            StatusCode::INTERNAL_SERVER_ERROR
                        },
                        | _ => StatusCode::BAD_REQUEST,
                    };

                    let field: Option<String> = match &kind {
                        | ErrorKind::ParseErrorAtKey { key, .. } => {
                            Some(key.clone())
                        },
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
                            .error_message(&kind.to_string());

                    match field {
                        | Some(field) => res.error_field(&field).send(),
                        | None => res.send(),
                    }
                },
                | PathRejection::MissingPathParams(inner) => {
                    CreateJsonResponse::failure()
                        .status(inner.status())
                        .error_code(JsonResponseErrorCode::Parse.as_str())
                        .error_message(&inner.body_text())
                        .send()
                },
                | _ => CreateJsonResponse::failure()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .error_code(JsonResponseErrorCode::Server.as_str())
                    .error_message(&rejection.body_text())
                    .send(),
            }),
        }
    }
}
