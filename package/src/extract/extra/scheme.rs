use axum_core::extract::FromRequestParts;
use axum_extra::extract::Scheme as _Scheme;
use http::request::Parts;

use crate::response::{
    Response,
    json::{CreateJsonResponse, error::JsonResponseErrorCode},
};

/// Extractor that resolves the scheme / protocol of a request.
///
/// The scheme is resolved through the following, in order:
/// - `Forwarded` header
/// - `X-Forwarded-Proto` header
/// - Request URI (If the request is an HTTP/2 request! e.g. use `--http2(-prior-knowledge)` with cURL)
///
/// Note that user agents can set the `X-Forwarded-Proto` header to arbitrary values so make
/// sure to validate them to avoid security issues.
///
/// ## Example
///
/// ```no_run
/// use jder_axum::extract::extra::Scheme;
///
/// async fn route(
///     Scheme(scheme): Scheme
/// ) {
///     // ...
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Scheme(pub String);

impl<S> FromRequestParts<S> for Scheme
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match _Scheme::from_request_parts(parts, state).await {
            | Ok(val) => Ok(Self(val.0)),
            | Err(rej) => Err(CreateJsonResponse::failure()
                .status(rej.status())
                .error_code(JsonResponseErrorCode::Parse.as_str())
                .error_message(rej.body_text())
                .send()),
        }
    }
}
