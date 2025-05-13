use axum::extract::Json as _Json;
use axum_core::{
    extract::{FromRequest, OptionalFromRequest, Request},
    response::IntoResponse,
};
use bytes::{BufMut, BytesMut, buf::Writer};
use http::{StatusCode, header};
use serde::{Serialize, de::DeserializeOwned};

use crate::response::{
    CreateResponse, Response,
    json::{CreateJsonResponse, JsonResponseErrorCode},
};

/// JSON extractor / response.
///
/// Check [`Json`](axum::extract::Json) for more information.
///
/// ## Examples
///
/// An example of using `Json` as an extractor:
///
/// ```no_run
/// use jder_axum::extract::Json;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct CreateUser {
///     email: String,
///     password: String,
/// }
///
/// async fn route(
///     Json(payload): Json<CreateUser>
/// ) {
///     // ...
/// }
/// ```
///
/// An example of using `Json` as a response:
///
/// ```no_run
/// use jder_axum::extract::Json;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct User {
///     id: usize,
///     username: String,
/// }
///
/// async fn route() -> Json<User> {
///     Json(User {
///         id: 1,
///         username: "Name".to_string(),
///     })
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Json<T>(pub T);

impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(
        req: Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match <_Json<T> as FromRequest<S>>::from_request(req, state).await {
            | Ok(val) => Ok(Self(val.0)),
            | Err(rej) => Err(CreateJsonResponse::failure()
                .status(rej.status())
                .error_code(JsonResponseErrorCode::Parse.as_str())
                .error_message(rej.body_text())
                .send()),
        }
    }
}

impl<T, S> OptionalFromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(
        req: Request,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        match <_Json<T> as OptionalFromRequest<S>>::from_request(req, state)
            .await
        {
            | Ok(Some(val)) => Ok(Some(Self(val.0))),
            | Ok(None) => Ok(None),
            | Err(rej) => Err(CreateJsonResponse::failure()
                .status(rej.status())
                .error_code(JsonResponseErrorCode::Parse.as_str())
                .error_message(rej.body_text())
                .send()),
        }
    }
}

axum_core::__impl_deref!(Json);

impl<T> From<T> for Json<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> Json<T>
where
    T: DeserializeOwned,
{
    /// Construct a `Json<T>` from a byte slice. Most users should prefer to use the `FromRequest` impl
    /// but special cases may require first extracting a `Request` into `Bytes` then optionally
    /// constructing a `Json<T>`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Response> {
        match _Json::<T>::from_bytes(bytes) {
            | Ok(val) => Ok(Self(val.0)),
            | Err(rej) => Err(CreateJsonResponse::failure()
                .status(rej.status())
                .error_code(JsonResponseErrorCode::Parse.as_str())
                .error_message(rej.body_text())
                .send()),
        }
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let mut buf: Writer<BytesMut> = BytesMut::with_capacity(128).writer();

        match serde_json::to_writer(&mut buf, &self.0) {
            | Ok(_) => CreateResponse::success()
                .header(header::CONTENT_TYPE, "application/json")
                .body(buf.into_inner().freeze()),
            | Err(err) => CreateJsonResponse::failure()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .error_code(JsonResponseErrorCode::Server.as_str())
                .error_message(err.to_string())
                .send(),
        }
    }
}
