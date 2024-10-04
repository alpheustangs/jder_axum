use axum::http::{HeaderName, StatusCode};
use serde::Serialize;

use crate::utils::response::{
    json::{create_json_response_send, JsonResponseState},
    Response,
};

/// Functions for creating a success response.
pub struct JsonSuccessResponseFunctions<D> {
    pub state: JsonResponseState<D>,
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
    ///     CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::dataless()
    ///         .status(StatusCode::CREATED)
    ///         .send()
    /// }
    /// ```
    pub fn status(
        mut self,
        status: StatusCode,
    ) -> Self {
        self.state.status = status;
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
    ///     CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::dataless()
    ///         .version(Version::HTTP_3)
    ///         .send()
    /// }
    /// ```
    pub fn version(
        mut self,
        version: axum::http::Version,
    ) -> Self {
        self.state.version = version;
        self
    }

    /// Set a header for the response.
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
    ///     CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::dataless()
    ///         .header(
    ///             header::CONTENT_TYPE,
    ///             "application/json".to_string()
    ///         )
    ///         .send()
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
    ///     CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::dataless()
    ///         .headers(vec![
    ///             (
    ///                 header::CONTENT_TYPE,
    ///                 "application/json".to_string()
    ///             ),
    ///             (
    ///                 header::ACCESS_CONTROL_ALLOW_ORIGIN,
    ///                 "*".to_string()
    ///             ),
    ///         ])
    ///         .send()
    /// }
    /// ```
    pub fn headers(
        mut self,
        headers: Vec<(HeaderName, String)>,
    ) -> Self {
        self.state.headers.extend(headers);
        self
    }

    /// Send the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::{
    ///     Response,
    ///     CreateJsonResponse,
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
    ///     CreateJsonResponse
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
