use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::{header, StatusCode},
    response::IntoResponse,
    Json as _Json,
};
use bytes::{buf::Writer, BufMut, BytesMut};
use serde::{de::DeserializeOwned, Serialize};

use crate::utils::response::{
    error::{ResponseErrorCode, SERVER_ERROR_RESPONSE},
    json::{CreateJsonResponse, JsonResponse, JsonResponseError},
    Response,
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

fn match_rejection(rejection: JsonRejection) -> Response {
    match rejection {
        | JsonRejection::JsonDataError(inner) => CreateJsonResponse::failure()
            .status(inner.status())
            .error_code(ResponseErrorCode::ParseError.to_string())
            .error_message(inner.to_string())
            .send(),
        | JsonRejection::JsonSyntaxError(inner) => {
            CreateJsonResponse::failure()
                .status(inner.status())
                .error_code(ResponseErrorCode::ParseError.to_string())
                .error_message(inner.to_string())
                .send()
        },
        | JsonRejection::MissingJsonContentType(inner) => {
            CreateJsonResponse::failure()
                .status(inner.status())
                .error_code(ResponseErrorCode::ParseError.to_string())
                .error_message(inner.to_string())
                .send()
        },
        | JsonRejection::BytesRejection(inner) => CreateJsonResponse::failure()
            .status(inner.status())
            .error_code(ResponseErrorCode::ParseError.to_string())
            .error_message(inner.to_string())
            .send(),
        | _ => CreateJsonResponse::failure()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .error_code(ResponseErrorCode::ServerError.to_string())
            .error_message(rejection.to_string())
            .send(),
    }
}

#[async_trait]
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
        match _Json::<T>::from_request(req, state).await {
            | Ok(value) => Ok(Self(value.0)),
            | Err(rejection) => Err(match_rejection(rejection)),
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
    /// Construct a `Json<T>` from a byte slice.
    /// Most users should prefer to use the `FromRequest` impl
    /// but special cases may require first extracting
    /// a `Request` into `Bytes` then optionally
    /// constructing a `Json<T>`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Response> {
        match _Json::<T>::from_bytes(bytes) {
            | Ok(value) => Ok(Self(value.0)),
            | Err(rejection) => Err(match_rejection(rejection)),
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
            | Ok(()) => (
                [(header::CONTENT_TYPE, "application/json".to_string())],
                buf.into_inner().freeze(),
            )
                .into_response(),
            | Err(err) => {
                let res: JsonResponse = JsonResponse {
                    success: false,
                    data: None,
                    error: Some(JsonResponseError {
                        code: ResponseErrorCode::ServerError.to_string(),
                        field: None,
                        message: Some(err.to_string()),
                    }),
                };

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    [(header::CONTENT_TYPE, "application/json".to_string())],
                    match serde_json::to_string(&res) {
                        | Ok(value) => value,
                        | Err(_) => SERVER_ERROR_RESPONSE.to_string(),
                    },
                )
            }
            .into_response(),
        }
    }
}
