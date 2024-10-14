use axum::{
    async_trait,
    extract::{
        rejection::ExtensionRejection, ConnectInfo as _ConnectInfo,
        FromRequestParts,
    },
    http::{request::Parts, StatusCode},
};

use crate::utils::response::{
    json::{error::JsonResponseErrorCode, CreateJsonResponse},
    Response,
};

/// Extractor for getting connection information produced
/// by a [`Connected`](axum::extract::connect_info::Connected).
///
/// Check [`ConnectInfo`](axum::extract::ConnectInfo) for more information.
///
/// ## Example
///
/// ```ignore
/// use std::net::{IpAddr, SocketAddr};
///
/// use axum::{
///     Router,
///     routing::get,
/// };
/// use jder_axum::extract::ConnectInfo;
/// use tokio::net::TcpListener;
///
/// async fn route(
///     ConnectInfo(addr): ConnectInfo<SocketAddr>
/// ) {
///     let ip: ipAddr = addr.ip();
///     let is_ipv4: bool = addr.is_ipv4();
///     let is_ipv6: bool = addr.is_ipv6();
///     let port: u16 = addr.port();
/// }
///
/// #[tokio::main]
/// async fn main(){
///     let router: Router = Router::new()
///         .route("/", get(route));
///
///     axum::serve(
///         TcpListener::bind("0.0.0.0:4001").await.unwrap(),
///         router.into_make_service_with_connect_info::<SocketAddr>()
///     ).await.unwrap();
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ConnectInfo<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for ConnectInfo<T>
where
    S: Send + Sync,
    T: Clone + Send + Sync + 'static,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match _ConnectInfo::<T>::from_request_parts(parts, state).await {
            | Ok(value) => Ok(Self(value.0)),
            | Err(rejection) => Err(match rejection {
                | ExtensionRejection::MissingExtension(inner) => {
                    CreateJsonResponse::failure()
                        .status(inner.status())
                        .error_code(JsonResponseErrorCode::Parse.as_str())
                        .error_message(&inner.body_text())
                        .send()
                },
                | _ => CreateJsonResponse::failure()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .error_code(JsonResponseErrorCode::Server.as_str())
                    .error_message(&rejection.body_text())
                    .send(),
            }),
        }
    }
}
