use axum::{
    extract::{FromRequestParts, Query as _Query, rejection::QueryRejection},
    http::{StatusCode, request::Parts},
};
use serde::{
    Deserialize, Deserializer,
    de::{self, DeserializeOwned},
};

use crate::internal::response::{
    Response,
    json::{CreateJsonResponse, error::JsonResponseErrorCode},
};

/// Convert an empty string to None instead of returning an error.
///
/// ## Example
///
/// ```no_run
/// use serde::Deserialize;
/// use jder_axum::extract::query::{
///     Query,
///     empty_to_none,
/// };
///
/// #[derive(Deserialize)]
/// struct QueryParams {
///     #[serde(default, deserialize_with = "empty_to_none")]
///     page: Option<usize>,
///     per_page: Option<usize>,
/// }
///
/// // /products?page=1&per_page=60
/// async fn route(
///     query: Query<QueryParams>,
/// ) {
///     let query: QueryParams = query.0;
///     // ...
/// }
/// ```
pub fn empty_to_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let opt: Option<String> = Option::<String>::deserialize(de)?;

    match opt.as_deref() {
        | None | Some("") => Ok(None),
        | Some(s) => {
            std::str::FromStr::from_str(s).map(Some).map_err(de::Error::custom)
        },
    }
}

/// Extractor that deserializes query strings into some type.
/// To accept empty string, [`empty_to_none`] should be used.
///
/// Check [`Query`](axum::extract::Query) for more information.
///
/// ## Example
///
/// ```no_run
/// use serde::Deserialize;
/// use jder_axum::extract::Query;
///
/// #[derive(Deserialize)]
/// struct QueryParams {
///     page: usize,
///     per_page: usize,
/// }
///
/// // /products?page=1&per_page=60
/// async fn route(
///     query: Query<QueryParams>,
/// ) {
///     let query: QueryParams = query.0;
///     // ...
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Query<T>(pub T);

impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match _Query::<T>::from_request_parts(parts, state).await {
            | Ok(value) => Ok(Self(value.0)),
            | Err(rejection) => Err(match rejection {
                | QueryRejection::FailedToDeserializeQueryString(inner) => {
                    CreateJsonResponse::failure()
                        .status(inner.status())
                        .error_code(JsonResponseErrorCode::Parse.as_str())
                        .error_message(inner.body_text())
                        .send()
                },
                | _ => CreateJsonResponse::failure()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .error_code(JsonResponseErrorCode::Server.as_str())
                    .error_message(rejection.body_text())
                    .send(),
            }),
        }
    }
}
