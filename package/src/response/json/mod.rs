pub(crate) mod error;
pub(crate) mod failure;
pub(crate) mod success;

pub use crate::response::json::success::JsonSuccessResponseFunctions;

pub use crate::response::json::failure::JsonFailureResponseFunctions;

pub use crate::response::json::error::JsonResponseErrorCode;

use axum::{
    body::Body,
    http::{
        HeaderMap, HeaderValue, StatusCode, Version, header, response::Builder,
    },
    response::Response,
};
use serde::{Deserialize, Serialize};

use crate::response::json::error::FAILURE_RESPONSE_DEFAULT;

/// JSON response error.
///
/// For API documentation generation with utoipa,
/// `ToSchema` derive is available with the `utoipa` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct JsonResponseError {
    /// Error code.
    pub code: String,
    /// Field of the error.
    pub field: Option<String>,
    /// Message of the error.
    pub message: Option<String>,
}

/// JSON response.
///
/// For API documentation generation with utoipa,
/// `ToSchema` derive is available with the `utoipa` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct JsonResponse<D = ()> {
    /// Whether the response is successful.
    pub success: bool,
    /// Data for the response when `success` is `true`.
    pub data: Option<D>,
    /// Error for the response when `success` is `false`.
    pub error: Option<JsonResponseError>,
}

/// Internal state.
#[derive(Debug, Clone, Default)]
pub(crate) struct JsonResponseState<D> {
    status: StatusCode,
    version: Version,
    header_map: HeaderMap,
    is_header_map_failed: bool,
    success: bool,
    data: Option<D>,
    error: Option<JsonResponseError>,
}

pub(crate) fn create_json_response_send<D: Serialize>(
    state: JsonResponseState<D>
) -> Response {
    let server_error: Response = Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(FAILURE_RESPONSE_DEFAULT.to_string()))
        .unwrap();

    // header map error
    if state.is_header_map_failed {
        // create error
        let res_error: JsonResponseError = JsonResponseError {
            code: JsonResponseErrorCode::Parse.to_string(),
            field: Some("header_map".to_string()),
            message: Some("Failed to create header map.".to_string()),
        };

        let res: JsonResponse<D> =
            JsonResponse { success: false, data: None, error: Some(res_error) };

        // parse body
        let body: String = match serde_json::to_string(&res) {
            | Ok(body) => body,
            | Err(_) => {
                return server_error;
            },
        };

        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body))
            .unwrap();
    }

    // create response builder
    let mut builder: Builder =
        Response::builder().status(state.status).version(state.version);

    // set content type
    let mut header_map: HeaderMap = state.header_map;

    header_map.append(
        header::CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );

    // push headers
    for (header, value) in header_map {
        if let Some(header) = header {
            builder = builder.header(header, value);
        }
    }

    // create response
    let res: JsonResponse<D> = JsonResponse {
        success: state.success,
        data: state.data,
        error: state.error,
    };

    // parse body
    let body: String = match serde_json::to_string(&res) {
        | Ok(body) => body,
        | Err(_) => {
            return server_error;
        },
    };

    // result
    builder.body(Body::from(body)).unwrap()
}

/// Create a JSON response for a route.
///
/// ## Examples
///
/// A success JSON response without data:
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
///
/// A sucesss JSON response:
///
/// ```no_run
/// use jder_axum::response::{
///     Response,
///     json::CreateJsonResponse,
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
///
/// A failure JSON response:
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
#[derive(Debug, Clone, Copy, Default)]
pub struct CreateJsonResponse;

impl CreateJsonResponse {
    /// Create a success JSON response without data.
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
    pub fn dataless() -> JsonSuccessResponseFunctions<()> {
        JsonSuccessResponseFunctions {
            state: JsonResponseState {
                status: StatusCode::OK,
                version: Version::HTTP_11,
                header_map: HeaderMap::new(),
                is_header_map_failed: false,
                success: true,
                data: None,
                error: None,
            },
        }
    }

    /// Create a success JSON response.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::response::{
    ///     Response,
    ///     json::CreateJsonResponse,
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
    pub fn success<D>() -> JsonSuccessResponseFunctions<D> {
        JsonSuccessResponseFunctions {
            state: JsonResponseState {
                status: StatusCode::OK,
                version: Version::HTTP_11,
                header_map: HeaderMap::new(),
                is_header_map_failed: false,
                success: true,
                data: None,
                error: None,
            },
        }
    }

    /// Create a failure JSON response.
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
    pub fn failure() -> JsonFailureResponseFunctions<()> {
        JsonFailureResponseFunctions {
            state: JsonResponseState {
                status: StatusCode::BAD_REQUEST,
                version: Version::HTTP_11,
                header_map: HeaderMap::new(),
                is_header_map_failed: false,
                success: false,
                data: None,
                error: None,
            },
        }
    }
}
