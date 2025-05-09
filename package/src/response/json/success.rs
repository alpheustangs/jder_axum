use axum::http::{
    Error as HTTPError, HeaderName, HeaderValue, StatusCode, Version,
};
use serde::Serialize;

use crate::response::{
    Response,
    json::{JsonResponseState, create_json_response_send},
};

/// Functions for creating a success response.
#[derive(Debug, Clone, Default)]
pub struct JsonSuccessResponseFunctions<D> {
    /// Internal state.
    pub(crate) state: JsonResponseState<D>,
}

impl<D: Serialize> JsonSuccessResponseFunctions<D> {
    /// Set the status code for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use axum::http::StatusCode;
    /// use jder_axum::response::{
    ///     Response,
    ///     json::CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::dataless()
    ///         .status(StatusCode::CREATED)
    ///         .send()
    /// }
    /// ```
    pub fn status<S: Into<StatusCode>>(
        mut self,
        status: S,
    ) -> Self {
        self.state.status = status.into();

        self
    }

    /// Set the HTTP version for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use axum::http::Version;
    /// use jder_axum::response::{
    ///     Response,
    ///     json::CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::dataless()
    ///         .version(Version::HTTP_3)
    ///         .send()
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
    /// For validation on key value, see
    /// [`get_header_from_key_value`](crate::response::header::get_header_from_key_value).
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
    ///     json::CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::dataless()
    ///         .header(
    ///             header::CONTENT_TYPE,
    ///             "application/json"
    ///         )
    ///         .send()
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
        let key: HeaderName = match <HeaderName as TryFrom<K>>::try_from(key) {
            | Ok(k) => k,
            | Err(_) => {
                self.state.is_header_map_failed = true;
                return self;
            },
        };

        let value: HeaderValue =
            match <HeaderValue as TryFrom<V>>::try_from(value) {
                | Ok(v) => v,
                | Err(_) => {
                    self.state.is_header_map_failed = true;
                    return self;
                },
            };

        self.state.header_map.try_append(key, value).unwrap();

        self
    }

    /// Set multiple headers for the response.
    ///
    /// For validation on key value, see
    /// [`get_header_from_key_value`](crate::response::header::get_header_from_key_value).
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
    ///     json::CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     let headers: Vec<(HeaderName, &str)> = vec![
    ///         (
    ///             header::CONTENT_TYPE,
    ///             "application/json"
    ///         ),
    ///         (
    ///             header::ACCESS_CONTROL_ALLOW_ORIGIN,
    ///             "*"
    ///         ),
    ///     ];
    ///
    ///     CreateJsonResponse::dataless()
    ///         .headers(headers)
    ///         .send()
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

    /// Send the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::{
    ///     Response,
    ///     json::CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::dataless().send()
    /// }
    /// ```
    pub fn send(self) -> Response {
        create_json_response_send(self.state)
    }
}

impl<D> JsonSuccessResponseFunctions<D> {
    /// Set the data for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::{
    ///     Response,
    ///     json::CreateJsonResponse
    /// };
    /// use serde::Serialize;
    ///
    /// #[derive(Default, Serialize)]
    /// struct ResponseData {
    ///    name: String,
    /// }
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::success::<ResponseData>()
    ///         .data(ResponseData { name: "Name".to_string() })
    ///         .send()
    /// }
    /// ```
    pub fn data(
        mut self,
        data: D,
    ) -> Self {
        self.state.data = Some(data);

        self
    }
}
