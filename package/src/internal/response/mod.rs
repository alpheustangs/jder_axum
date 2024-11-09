pub mod header;
pub mod json;

use axum::{
    body::Body,
    http::{
        response::Builder, Error as HTTPError, HeaderMap, HeaderName,
        HeaderValue, Response as _Response, StatusCode, Version,
    },
};

/// Response for routes.
pub type Response<B = Body> = _Response<B>;

/// Internal state.
#[derive(Debug, Clone)]
struct ResponseState<B> {
    status: StatusCode,
    version: Version,
    header_map: HeaderMap,
    body: B,
}

/// Functions for creating response.
#[derive(Debug, Clone)]
pub struct ResponseFunctions<B> {
    state: ResponseState<B>,
}

impl<B> ResponseFunctions<B>
where
    Body: From<B>,
{
    /// Set the status code of the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use axum::http::StatusCode;
    /// use jder_axum::response::{
    ///     Response,
    ///     CreateResponse
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateResponse::success()
    ///         .status(StatusCode::CREATED)
    ///         .body("created")
    /// }
    /// ```
    pub fn status<S: Into<StatusCode>>(
        mut self,
        status: S,
    ) -> Self {
        self.state.status = status.into();

        self
    }

    /// Set the HTTP version of the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use axum::http::Version;
    /// use jder_axum::response::{
    ///     Response,
    ///     CreateResponse
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateResponse::success()
    ///         .version(Version::HTTP_3)
    ///         .body("active")
    /// }
    /// ```
    pub fn version<V: Into<Version>>(
        mut self,
        version: V,
    ) -> Self {
        self.state.version = version.into();

        self
    }

    /// Set a header for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use axum::http::header;
    /// use jder_axum::response::{
    ///     Response,
    ///     CreateResponse
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateResponse::success()
    ///         .header(
    ///             header::CONTENT_TYPE,
    ///             "text/plain"
    ///         )
    ///         .body("active")
    /// }
    /// ```
    pub fn header<K, V>(
        mut self,
        key: K,
        value: V,
    ) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<HTTPError>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<HTTPError>,
    {
        let key: HeaderName = <HeaderName as TryFrom<K>>::try_from(key)
            .map_err(Into::into)
            .unwrap();

        let value: HeaderValue = <HeaderValue as TryFrom<V>>::try_from(value)
            .map_err(Into::into)
            .unwrap();

        self.state.header_map.try_append(key, value).unwrap();

        self
    }

    /// Set multiple headers for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use axum::http::{
    ///     header,
    ///     HeaderName,
    /// };
    /// use jder_axum::response::{
    ///     Response,
    ///     CreateResponse
    /// };
    ///
    /// async fn route() -> Response {
    ///
    ///     let headers: Vec<(HeaderName, &str)> = vec![
    ///         (
    ///             header::ACCESS_CONTROL_ALLOW_ORIGIN,
    ///             "*"
    ///         ),
    ///         (
    ///             header::CONTENT_TYPE,
    ///             "text/plain"
    ///         ),
    ///     ];
    ///
    ///     CreateResponse::success()
    ///         .headers(headers)
    ///         .body("active")
    /// }
    /// ```
    pub fn headers<K, V>(
        mut self,
        headers: impl IntoIterator<Item = (K, V)>,
    ) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<HTTPError>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<HTTPError>,
    {
        for (key, value) in headers {
            self = self.header(key, value);
        }

        self
    }

    /// Set the body of the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::{
    ///     Response,
    ///     CreateResponse
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateResponse::success()
    ///         .body("active")
    /// }
    /// ```
    pub fn body(
        mut self,
        body: B,
    ) -> Response {
        self.state.body = body;

        let mut builder: Builder = Response::builder()
            .status(self.state.status)
            .version(self.state.version);

        for (header, value) in self.state.header_map {
            if let Some(header) = header {
                builder = builder.header(header, value);
            }
        }

        builder.body(Body::from(self.state.body)).unwrap()
    }
}

/// Create a response for a route.
///
/// ## Example
///
/// ```no_run
/// use jder_axum::response::{
///     Response,
///     CreateResponse
/// };
///
/// async fn route() -> Response {
///     CreateResponse::success()
///         .body("active")
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct CreateResponse;

impl CreateResponse {
    /// Create a success response.
    pub fn success<B: Default>() -> ResponseFunctions<B> {
        ResponseFunctions {
            state: ResponseState {
                status: StatusCode::OK,
                version: Version::HTTP_11,
                header_map: HeaderMap::new(),
                body: B::default(),
            },
        }
    }

    /// Create a failure response.
    pub fn failure<B: Default>() -> ResponseFunctions<B> {
        ResponseFunctions {
            state: ResponseState {
                status: StatusCode::BAD_REQUEST,
                version: Version::HTTP_11,
                header_map: HeaderMap::new(),
                body: B::default(),
            },
        }
    }
}
