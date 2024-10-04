pub mod error;
pub mod json;

use axum::{
    body::Body,
    http::{
        response::Builder, HeaderName, Response as _Response, StatusCode,
        Version,
    },
};

/// Response type for routes.
pub type Response<B = Body> = _Response<B>;

struct ResponseState<B> {
    status: StatusCode,
    version: Version,
    headers: Vec<(HeaderName, String)>,
    body: B,
}

/// Functions for creating response.
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
    ///         .body("created".to_string())
    /// }
    /// ```
    pub fn status(
        mut self,
        status: StatusCode,
    ) -> Self {
        self.state.status = status;
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
    ///         .body("active".to_string())
    /// }
    /// ```
    pub fn version(
        mut self,
        version: Version,
    ) -> Self {
        self.state.version = version;
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
    ///             "text/plain".to_string()
    ///         )
    ///         .body("active".to_string())
    /// }
    /// ```
    pub fn header(
        mut self,
        header: HeaderName,
        value: String,
    ) -> Self {
        self.state.headers.push((header, value));
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
    ///     let headers: Vec<(HeaderName, String)> = vec![
    ///         (
    ///             header::ACCESS_CONTROL_ALLOW_ORIGIN,
    ///             "*".to_string()
    ///         ),
    ///         (
    ///             header::CONTENT_TYPE,
    ///             "text/plain".to_string()
    ///         ),
    ///     ];
    ///
    ///     CreateResponse::success()
    ///         .headers(headers)
    ///         .body("active".to_string())
    /// }
    /// ```
    pub fn headers(
        mut self,
        headers: Vec<(HeaderName, String)>,
    ) -> Self {
        self.state.headers.extend(headers);
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
    ///         .body("active".to_string())
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

        for (header, value) in self.state.headers {
            builder = builder.header(header, value);
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
///         .body("active".to_string())
/// }
/// ```
pub struct CreateResponse;

impl CreateResponse {
    /// Create a success response.
    pub fn success<B: Default>() -> ResponseFunctions<B> {
        ResponseFunctions {
            state: ResponseState {
                status: StatusCode::OK,
                version: Version::HTTP_11,
                headers: vec![],
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
                headers: vec![],
                body: B::default(),
            },
        }
    }
}
