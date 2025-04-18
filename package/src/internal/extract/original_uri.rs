use axum::{
    extract::{FromRequestParts, OriginalUri as _OriginalUri},
    http::{StatusCode, Uri, request::Parts},
};

use crate::internal::response::{
    Response,
    json::{CreateJsonResponse, error::JsonResponseErrorCode},
};

/// Extractor that gets the original request URI regardless of nesting.
///
/// Check [`OriginalUri`](axum::extract::OriginalUri) for more information.
///
/// ## Example
///
/// ```no_run
/// use axum::{
///     routing::get,
///     Router,
///     http::Uri
/// };
/// use jder_axum::extract::OriginalUri;
///
/// // `/123/profile`
/// async fn route(
///     uri: Uri,
///     OriginalUri(original_uri): OriginalUri
/// ) {
///     // `uri` is `/profile`
///     // `original_uri` is `/123/profile`
/// }
///
/// let router_users: Router = Router::new()
///     .route("/profile", get(route));
///
/// let app: Router = Router::new()
///     .nest("/:id", router_users);
/// ```
#[derive(Debug, Clone)]
pub struct OriginalUri(pub Uri);

impl<S> FromRequestParts<S> for OriginalUri
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match _OriginalUri::from_request_parts(parts, state).await {
            | Ok(value) => Ok(Self(value.0)),
            | Err(rejection) => Err(CreateJsonResponse::failure()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .error_code(JsonResponseErrorCode::Server.as_str())
                .error_message(rejection.to_string())
                .send()),
        }
    }
}

axum_core::__impl_deref!(OriginalUri: Uri);
