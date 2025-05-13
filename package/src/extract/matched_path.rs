use std::sync::Arc;

use axum::extract::MatchedPath as _MatchedPath;
use axum_core::extract::{FromRequestParts, OptionalFromRequestParts};
use http::{StatusCode, request::Parts};

use crate::response::{
    Response,
    json::{CreateJsonResponse, error::JsonResponseErrorCode},
};

/// Access the path in the router that matches the request.
///
/// Check [`MatchedPath`](axum::extract::MatchedPath) for more information.
///
/// ## Example
///
/// ```no_run
/// use axum::{
///     Router,
///     routing::get,
/// };
/// use jder_axum::extract::MatchedPath;
///
/// async fn route(path: MatchedPath) {
///     let path: &str = path.as_str();
///     // "/users/{id}"
/// }
///
/// let router: Router = Router::new()
///     .route("/users/{id}", get(route));
/// ```
#[derive(Debug, Clone)]
pub struct MatchedPath(pub(crate) Arc<str>);

impl MatchedPath {
    /// Returns a `str` representation of the path.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<S> FromRequestParts<S> for MatchedPath
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match <_MatchedPath as FromRequestParts<S>>::from_request_parts(
            parts, state,
        )
        .await
        {
            | Ok(val) => Ok(MatchedPath(val.as_str().into())),
            | Err(rej) => Err(CreateJsonResponse::failure()
                .status(rej.status())
                .error_code(JsonResponseErrorCode::Parse.as_str())
                .error_message(rej.body_text())
                .send()),
        }
    }
}

impl<S> OptionalFromRequestParts<S> for MatchedPath
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        match <_MatchedPath as OptionalFromRequestParts<S>>::from_request_parts(
            parts, state,
        )
        .await
        {
            | Ok(Some(val)) => Ok(Some(Self(val.as_str().into()))),
            | Ok(None) => Ok(None),
            | Err(_) => Err(CreateJsonResponse::failure()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .error_code(JsonResponseErrorCode::Server.as_str())
                .send()),
        }
    }
}
