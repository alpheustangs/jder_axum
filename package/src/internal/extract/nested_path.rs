use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequestParts, NestedPath as _NestedPath},
    http::{request::Parts, StatusCode},
};

use crate::internal::response::{
    json::{error::JsonResponseErrorCode, CreateJsonResponse},
    Response,
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
///     // "/:id"
/// }
///
/// let router_users: Router = Router::new()
///     .route("/profile", get(route));
///
/// let router: Router = Router::new()
///     .nest("/:id", router_users);
/// ```
#[derive(Debug, Clone)]
pub struct NestedPath(Arc<str>);

impl NestedPath {
    /// Returns a `str` representation of the path.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[async_trait]
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
            | Ok(value) => Ok(NestedPath(value.as_str().into())),
            | Err(rejection) => Err(CreateJsonResponse::failure()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .error_code(JsonResponseErrorCode::Server.as_str())
                .error_message(rejection.body_text())
                .send()),
        }
    }
}