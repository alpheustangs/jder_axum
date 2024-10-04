use axum::http::{HeaderName, StatusCode};
use serde::Serialize;

use crate::utils::response::{
    error::ResponseErrorCode,
    json::{create_json_response_send, JsonResponseError, JsonResponseState},
    Response,
};

/// Functions for creating an failure response.
pub struct JsonFailureResponseFunctions<D> {
    pub state: JsonResponseState<D>,
}

impl<D: Serialize> JsonFailureResponseFunctions<D> {
    /// Set the status code for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use axum::http::StatusCode;
    /// use jder_axum::response::{
    ///     Response,
    ///     CreateJsonResponse
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::failure()
    ///         .status(StatusCode::NOT_FOUND)
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
    ///     CreateJsonResponse
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::failure()
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
    ///     CreateJsonResponse::failure()
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
    ///     CreateJsonResponse::failure()
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
    ///     CreateJsonResponse::failure().send()
    /// }
    /// ```
    pub fn send(self) -> Response {
        create_json_response_send(self.state)
    }
}

impl<D: Serialize> JsonFailureResponseFunctions<D> {
    /// Set an error for the response.
    /// This action will overwrite `error_code`,
    /// `error_field`, and `error_message` if they are set.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::{
    ///     Response,
    ///     CreateJsonResponse,
    ///     JsonResponseError,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::failure()
    ///         .error(JsonResponseError {
    ///             code: "parse_error".to_string(),
    ///             field: Some("title".to_string()),
    ///             message: Some("Invalid title".to_string()),
    ///         })
    ///         .send()
    /// }
    /// ```
    pub fn error(
        mut self,
        error: JsonResponseError,
    ) -> Self {
        self.state.error = Some(error);
        self
    }

    /// Set an error code for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::{
    ///     Response,
    ///     CreateJsonResponse,
    ///     JsonResponseError,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::failure()
    ///         .error_code("parse_error".to_string())
    ///         .send()
    /// }
    /// ```
    pub fn error_code(
        mut self,
        code: String,
    ) -> Self {
        self.state.error = Some(JsonResponseError {
            code,
            field: match &self.state.error {
                | Some(error) => error.field.clone(),
                | None => None,
            },
            message: match self.state.error {
                | Some(error) => error.message.clone(),
                | None => None,
            },
        });
        self
    }

    /// Set an error field for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::{
    ///     Response,
    ///     CreateJsonResponse,
    ///     JsonResponseError,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::failure()
    ///         .error_field("title".to_string())
    ///         .send()
    /// }
    /// ```
    pub fn error_field(
        mut self,
        field: String,
    ) -> Self {
        self.state.error = Some(JsonResponseError {
            code: match &self.state.error {
                | Some(error) => error.code.clone(),
                | None => ResponseErrorCode::UnknownError.to_string(),
            },
            field: Some(field),
            message: match self.state.error {
                | Some(error) => error.message.clone(),
                | None => None,
            },
        });
        self
    }

    /// Set an error message for the response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::{
    ///     Response,
    ///     CreateJsonResponse,
    ///     JsonResponseError,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::failure()
    ///         .error_message("Invalid title".to_string())
    ///         .send()
    /// }
    /// ```
    pub fn error_message(
        mut self,
        message: String,
    ) -> Self {
        self.state.error = Some(JsonResponseError {
            code: match &self.state.error {
                | Some(error) => error.code.clone(),
                | None => ResponseErrorCode::UnknownError.to_string(),
            },
            field: match self.state.error {
                | Some(error) => error.field.clone(),
                | None => None,
            },
            message: Some(message),
        });
        self
    }
}
