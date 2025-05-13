use axum::extract::Path as _Path;
use axum_core::extract::{FromRequestParts, OptionalFromRequestParts};
use http::request::Parts;
use serde::de::DeserializeOwned;

use crate::response::{
    Response,
    json::{CreateJsonResponse, JsonResponseErrorCode},
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
/// // /users/{id}/{name}
/// async fn route(
///     Path((id, name)): Path<(String, String)>,
/// ) {
///     // ...
/// }
/// ```
#[derive(Debug)]
pub struct Path<T>(pub T);

axum_core::__impl_deref!(Path);

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
        match <_Path<T> as FromRequestParts<S>>::from_request_parts(
            parts, state,
        )
        .await
        {
            | Ok(val) => Ok(Self(val.0)),
            | Err(rej) => Err(CreateJsonResponse::failure()
                .status(rej.status())
                .error_code(JsonResponseErrorCode::Parse.as_str())
                .error_message(rej.body_text())
                .send()),
        }
    }
}

impl<T, S> OptionalFromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Send + 'static,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        match <_Path<T> as OptionalFromRequestParts<S>>::from_request_parts(
            parts, state,
        )
        .await
        {
            | Ok(Some(val)) => Ok(Some(Self(val.0))),
            | Ok(None) => Ok(None),
            | Err(rej) => Err(CreateJsonResponse::failure()
                .status(rej.status())
                .error_code(JsonResponseErrorCode::Parse.as_str())
                .error_message(rej.body_text())
                .send()),
        }
    }
}
