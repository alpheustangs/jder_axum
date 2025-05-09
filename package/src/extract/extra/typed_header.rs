use axum::http::{StatusCode, request::Parts};
use axum_core::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    response::{IntoResponse, IntoResponseParts, ResponseParts},
};
use axum_extra::{
    extract::TypedHeader as _TypedHeader,
    headers::{Header, HeaderMapExt as _},
};

use crate::response::{
    Response,
    json::{CreateJsonResponse, error::JsonResponseErrorCode},
};

/// Extractor and response that works with typed header values from [`headers`].
///
/// Check [`TypedHeader`](axum_extra::extract::TypedHeader) for more information.
///
/// ## Examples
///
/// An example of using `TypedHeader` as an extractor:
///
/// ```no_run
/// use jder_axum::extract::extra::TypedHeader;
/// use headers::UserAgent;
///
/// async fn route(
///     TypedHeader(agent): TypedHeader<UserAgent>,
/// ) {
///     // ...
/// }
/// ```
///
/// An example of using `TypedHeader` as a response:
///
/// ```no_run
/// use jder_axum::extract::extra::TypedHeader;
/// use headers::ContentType;
///
/// async fn route() -> (TypedHeader<ContentType>, &'static str) {
///     (
///         TypedHeader(ContentType::text_utf8()),
///         "Hello, World!",
///     )
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct TypedHeader<T>(pub T);

impl<T, S> FromRequestParts<S> for TypedHeader<T>
where
    T: Header,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match <_TypedHeader<T> as FromRequestParts<S>>::from_request_parts(
            parts, state,
        )
        .await
        {
            | Ok(val) => Ok(Self(val.0)),
            | Err(rej) => Err(CreateJsonResponse::failure()
                .status(StatusCode::BAD_REQUEST)
                .error_code(JsonResponseErrorCode::Parse.as_str())
                .error_field(rej.name().as_str())
                .error_message(format!("{:?}", rej.reason()))
                .send()),
        }
    }
}

impl<T, S> OptionalFromRequestParts<S> for TypedHeader<T>
where
    T: Header,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        match <_TypedHeader<T> as OptionalFromRequestParts<S>>::from_request_parts(
            parts, state,
        )
        .await
        {
            | Ok(Some(val)) => Ok(Some(Self(val.0))),
            | Ok(None) => Ok(None),
            | Err(rej) => Err(CreateJsonResponse::failure()
                .status(StatusCode::BAD_REQUEST)
                .error_code(JsonResponseErrorCode::Parse.as_str())
                .error_field(rej.name().as_str())
                .error_message(format!("{:?}", rej.reason()))
                .send()),
        }
    }
}

axum_core::__impl_deref!(TypedHeader);

impl<T> IntoResponseParts for TypedHeader<T>
where
    T: Header,
{
    type Error = Response;

    fn into_response_parts(
        self,
        mut res: ResponseParts,
    ) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().typed_insert(self.0);
        Ok(res)
    }
}

impl<T> IntoResponse for TypedHeader<T>
where
    T: Header,
{
    fn into_response(self) -> Response {
        let mut res = ().into_response();
        res.headers_mut().typed_insert(self.0);
        res
    }
}
