use std::sync::Arc;

use axum::{
    async_trait,
    extract::{
        rejection::MatchedPathRejection, FromRequestParts,
        MatchedPath as _MatchedPath,
    },
    http::{request::Parts, StatusCode},
};

use crate::utils::response::{
    error::ResponseErrorCode, json::CreateJsonResponse, Response,
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
#[derive(Clone, Debug)]
pub struct MatchedPath(pub(crate) Arc<str>);

impl MatchedPath {
    /// Returns a `str` representation of the path.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[async_trait]
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
                        .error_code(ResponseErrorCode::ParseError.to_string())
                        .error_message(inner.to_string())
                        .send()
                },
                | _ => CreateJsonResponse::failure()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .error_code(ResponseErrorCode::ServerError.to_string())
                    .error_message(rejection.to_string())
                    .send(),
            }),
        }
    }
}
