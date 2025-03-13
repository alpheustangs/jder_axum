pub mod from_request_parts;

use std::sync::Arc;

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
#[derive(Debug, Clone, Default)]
pub struct MatchedPath(pub(crate) Arc<str>);

impl MatchedPath {
    /// Returns a `str` representation of the path.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
