pub mod failure;
pub mod success;

use axum::{
    body::Body,
    http::{header, response::Builder, HeaderName, StatusCode, Version},
    response::Response,
};
use serde::{Deserialize, Serialize};

use crate::utils::response::{
    error::SERVER_ERROR_RESPONSE,
    json::{
        failure::JsonFailureResponseFunctions,
        success::JsonSuccessResponseFunctions,
    },
};

/// JSON response error type.
#[derive(Serialize, Deserialize)]
pub struct JsonResponseError {
    pub code: String,
    pub field: Option<String>,
    pub message: Option<String>,
}

/// JSON response type.
#[derive(Serialize, Deserialize)]
pub struct JsonResponse<D = ()> {
    pub success: bool,
    pub data: Option<D>,
    pub error: Option<JsonResponseError>,
}

pub struct JsonResponseState<D> {
    status: StatusCode,
    version: Version,
    headers: Vec<(HeaderName, String)>,
    success: bool,
    data: Option<D>,
    error: Option<JsonResponseError>,
}

pub fn create_json_response_send<D: Serialize>(
    mut state: JsonResponseState<D>
) -> Response {
    state.headers.push((header::CONTENT_TYPE, "application/json".to_string()));

    let res: JsonResponse<D> = JsonResponse {
        success: state.success,
        data: state.data,
        error: state.error,
    };

    let mut builder: Builder =
        Response::builder().status(state.status).version(state.version);

    for (header, value) in state.headers {
        builder = builder.header(header, value);
    }

    let body: String = match serde_json::to_string(&res) {
        | Ok(body) => body,
        | Err(_) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(SERVER_ERROR_RESPONSE.to_string()))
                .unwrap();
        },
    };

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
///     CreateJsonResponse,
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
///     CreateJsonResponse,
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
pub struct CreateJsonResponse;

impl CreateJsonResponse {
    /// Create a success JSON response without data.
    pub fn dataless() -> JsonSuccessResponseFunctions<()> {
        JsonSuccessResponseFunctions {
            state: JsonResponseState {
                status: StatusCode::OK,
                version: Version::HTTP_11,
                headers: vec![],
                success: true,
                data: None,
                error: None,
            },
        }
    }

    /// Create a success JSON response.
    pub fn success<D>() -> JsonSuccessResponseFunctions<D> {
        JsonSuccessResponseFunctions {
            state: JsonResponseState {
                status: StatusCode::OK,
                version: Version::HTTP_11,
                headers: vec![],
                success: true,
                data: None,
                error: None,
            },
        }
    }

    /// Create a failure JSON response.
    pub fn failure() -> JsonFailureResponseFunctions<()> {
        JsonFailureResponseFunctions {
            state: JsonResponseState {
                status: StatusCode::BAD_REQUEST,
                version: Version::HTTP_11,
                headers: vec![],
                success: false,
                data: None,
                error: None,
            },
        }
    }
}
