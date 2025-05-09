use std::sync::Arc;

use axum::{extract::NestedPath as _NestedPath, http::request::Parts};
use axum_core::extract::FromRequestParts;

use crate::response::{
    Response,
    json::{CreateJsonResponse, error::JsonResponseErrorCode},
};

/// Access the path the matched the route is nested at.
///
/// Check [`NestedPath`](axum::extract::NestedPath) for more information.
///
/// ## Example
///
/// ```no_run
/// use axum::{
///     Router,
///     routing::get,
/// };
/// use jder_axum::extract::NestedPath;
///
/// async fn route(nested_path: NestedPath) {
///     let path: &str = nested_path.as_str();
///     // "/{id}"
/// }
///
/// let router_users: Router = Router::new()
///     .route("/profile", get(route));
///
/// let router: Router = Router::new()
///     .nest("/{id}", router_users);
/// ```
#[derive(Debug, Clone)]
pub struct NestedPath(Arc<str>);

impl NestedPath {
    /// Returns a `str` representation of the path.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<S> FromRequestParts<S> for NestedPath
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match _NestedPath::from_request_parts(parts, state).await {
            | Ok(val) => Ok(NestedPath(val.as_str().into())),
            | Err(rej) => Err(CreateJsonResponse::failure()
                .status(rej.status())
                .error_code(JsonResponseErrorCode::Parse.as_str())
                .error_message(rej.body_text())
                .send()),
        }
    }
}
