use axum::extract::{Query as _Query, rejection::QueryRejection};
use axum_core::extract::FromRequestParts;
use http::{Uri, request::Parts};
use serde::{
    Deserialize, Deserializer,
    de::{self, DeserializeOwned},
};

use crate::response::{
    Response,
    json::{CreateJsonResponse, error::JsonResponseErrorCode},
};

/// Deserializes empty query parameters as `None` instead of empty strings.
///
/// This can help prevent unintended values from being parsed into the query struct,
/// especially when parameters are present but left empty (e.g., `?title=&page=`).
///
/// ## Example
///
/// ```no_run
/// use serde::Deserialize;
/// use jder_axum::extract::query::{
///     Query,
///     empty_as_none,
/// };
///
/// #[derive(Deserialize)]
/// struct QueryParams {
///     #[serde(default, deserialize_with = "empty_as_none")]
///     page: Option<usize>,
///     #[serde(default, deserialize_with = "empty_as_none")]
///     title: Option<String>,
/// }
///
/// // /products?page=&title=
/// async fn route(
///     Query(query): Query<QueryParams>,
/// ) {
///     // page = None
///     // title = None
/// }
/// ```
pub fn empty_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
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

/// Extractor for deserializing query strings into a specified type.
///
/// Can be used with [`empty_as_none`] to treat empty query parameters as `None`.
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
///     Query(query): Query<QueryParams>,
/// ) {
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
            | Ok(val) => Ok(Self(val.0)),
            | Err(rej) => Err(CreateJsonResponse::failure()
                .status(rej.status())
                .error_code(JsonResponseErrorCode::Parse.as_str())
                .error_message(rej.body_text())
                .send()),
        }
    }
}

impl<T> Query<T>
where
    T: DeserializeOwned,
{
    /// Attempts to construct a [`Query`] from a reference to a [`Uri`].
    ///
    /// # Example
    /// ```no_run
    /// use serde::Deserialize;
    /// use axum::http::Uri;
    /// use jder_axum::extract::Query;
    ///
    /// #[derive(Deserialize)]
    /// struct Params {
    ///     str: String,
    ///     num: u32,
    /// }
    ///
    /// let uri: Uri = "http://example.com/path?str=hello&num=42".parse().unwrap();
    ///
    /// let result: Query<Params> = Query::try_from_uri(&uri).unwrap();
    ///
    /// assert_eq!(result.str, String::from("hello"));
    /// assert_eq!(result.num, 42);
    /// ```
    pub fn try_from_uri(value: &Uri) -> Result<Self, QueryRejection> {
        match _Query::<T>::try_from_uri(value) {
            | Ok(val) => Ok(Self(val.0)),
            | Err(err) => Err(err),
        }
    }
}

axum_core::__impl_deref!(Query);
