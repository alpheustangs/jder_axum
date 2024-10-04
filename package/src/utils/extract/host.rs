use axum::{
    async_trait,
    extract::{rejection::HostRejection, FromRequestParts, Host as _Host},
    http::{request::Parts, StatusCode},
};

use crate::utils::response::{
    error::ResponseErrorCode, json::CreateJsonResponse, Response,
};

/// Extractor that resolves the hostname of the request.
///
/// Check [`Host`](axum::extract::Host) for more information.
///
/// ## Example
///
/// ```no_run
/// use jder_axum::extract::Host;
///
/// async fn route(host: Host) {
///     let host: String = host.0;
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Host(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for Host
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match _Host::from_request_parts(parts, state).await {
            | Ok(value) => Ok(Self(value.0)),
            | Err(rejection) => Err(match rejection {
                | HostRejection::FailedToResolveHost(inner) => {
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
