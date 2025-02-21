use std::sync::Arc;

use axum::{
    extract::{
        FromRequestParts, MatchedPath as _MatchedPath,
        rejection::MatchedPathRejection,
    },
    http::{StatusCode, request::Parts},
};

use crate::internal::response::{
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
///     // "/users/:id"
/// }
///
/// let router: Router = Router::new()
///     .route("/users/:id", get(route));
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
        match _MatchedPath::from_request_parts(parts, state).await {
            | Ok(value) => Ok(MatchedPath(value.as_str().into())),
            | Err(rejection) => Err(match rejection {
                | MatchedPathRejection::MatchedPathMissing(inner) => {
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
            }),
        }
    }
}
