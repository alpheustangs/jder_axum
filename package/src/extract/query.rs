use axum::{
    extract::{Query as _Query, rejection::QueryRejection},
    http::{Uri, request::Parts},
};
use axum_core::extract::FromRequestParts;
use serde::{
    Deserialize, Deserializer,
    de::{self, DeserializeOwned},
};

use crate::response::{
    Response,
    json::{CreateJsonResponse, error::JsonResponseErrorCode},
};

/// Convert empty query to None instead of returning an error.
///
/// ## Example
///
/// ```no_run
/// use serde::Deserialize;
/// use jder_axum::extract::query::{
///     Query,
///     optional_query,
/// };
///
/// #[derive(Deserialize)]
/// struct QueryParams {
///     #[serde(default, deserialize_with = "optional_query")]
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
pub fn optional_query<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
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
/// To accept empty query, [`optional_query`] should be used.
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
    /// use axum::http::Uri;
    /// use jder_axum::extract::Query;
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct ExampleParams {
    ///     foo: String,
    ///     bar: u32,
    /// }
    ///
    /// let uri: Uri = "http://example.com/path?foo=hello&bar=42".parse().unwrap();
    /// let result: Query<ExampleParams> = Query::try_from_uri(&uri).unwrap();
    /// assert_eq!(result.foo, String::from("hello"));
    /// assert_eq!(result.bar, 42);
    /// ```
    pub fn try_from_uri(value: &Uri) -> Result<Self, QueryRejection> {
        match _Query::<T>::try_from_uri(value) {
            | Ok(val) => Ok(Self(val.0)),
            | Err(err) => Err(err),
        }
    }
}

axum_core::__impl_deref!(Query);
