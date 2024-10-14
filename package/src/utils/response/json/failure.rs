use axum::http::{Error as HTTPError, HeaderName, HeaderValue, StatusCode};
use serde::Serialize;

use crate::utils::response::{
    json::{
        create_json_response_send, error::JsonResponseErrorCode,
        JsonResponseError, JsonResponseState,
    },
    Response,
};

/// Functions for creating an failure response.
#[derive(Debug, Clone)]
pub struct JsonFailureResponseFunctions<D> {
    pub(crate) state: JsonResponseState<D>,
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
    ///     json::CreateJsonResponse
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
    ///     json::CreateJsonResponse
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
    ///     CreateJsonResponse::failure()
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
    ///     json::{
    ///         CreateJsonResponse,
    ///         JsonResponseError,
    ///     },
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
    ///     json::CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::failure()
    ///         .error_code("parse_error")
    ///         .send()
    /// }
    /// ```
    pub fn error_code(
        mut self,
        code: &str,
    ) -> Self {
        self.state.error = Some(JsonResponseError {
            code: code.to_string(),
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
    ///     json::CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::failure()
    ///         .error_field("title")
    ///         .send()
    /// }
    /// ```
    pub fn error_field(
        mut self,
        field: &str,
    ) -> Self {
        self.state.error = Some(JsonResponseError {
            code: match &self.state.error {
                | Some(error) => &error.code,
                | None => JsonResponseErrorCode::Unknown.as_str(),
            }
            .to_string(),
            field: Some(field.to_string()),
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
    ///     json::CreateJsonResponse,
    /// };
    ///
    /// async fn route() -> Response {
    ///     CreateJsonResponse::failure()
    ///         .error_message("Invalid title")
    ///         .send()
    /// }
    /// ```
    pub fn error_message(
        mut self,
        message: &str,
    ) -> Self {
        self.state.error = Some(JsonResponseError {
            code: match &self.state.error {
                | Some(error) => &error.code,
                | None => JsonResponseErrorCode::Unknown.as_str(),
            }
            .to_string(),
            field: match self.state.error {
                | Some(error) => error.field,
                | None => None,
            },
            message: Some(message.to_string()),
        });

        self
    }
}
