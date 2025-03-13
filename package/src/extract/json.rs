use axum::{
    extract::{rejection::JsonRejection, FromRequest, Json as _Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;

use crate::response::{
    json::{CreateJsonResponse, JsonResponseErrorCode},
    Response,
};

/// JSON extractor / response.
///
/// Check [`Json`](axum::extract::Json) for more information.
///
/// ## Examples
///
/// An example of using `Json` as an extractor:
///
/// ```no_run
/// use jder_axum::extract::Json;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct CreateUser {
///     email: String,
///     password: String,
/// }
///
/// async fn route(
///     Json(payload): Json<CreateUser>
/// ) {
///     // ...
/// }
/// ```
///
/// An example of using `Json` as a response:
///
/// ```no_run
/// use jder_axum::extract::Json;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct User {
///     id: usize,
///     username: String,
/// }
///
/// async fn route() -> Json<User> {
///     Json(User {
///         id: 1,
///         username: "Name".to_string(),
///     })
/// }
/// ```
#[derive(Debug, Clone, Copy, Default, FromRequest)]
#[from_request(via(_Json), rejection(JsonError))]
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        let Self(value) = self;
        _Json(value).into_response()
    }
}

#[derive(Debug, Clone, Default)]
pub struct JsonError {
    pub status: StatusCode,
    pub code: String,
    pub message: String,
}

impl From<JsonRejection> for JsonError {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            status: rejection.status(),
            code: JsonResponseErrorCode::Parse.to_string(),
            message: rejection.body_text(),
        }
    }
}

impl IntoResponse for JsonError {
    fn into_response(self) -> Response {
        CreateJsonResponse::failure()
            .status(self.status)
            .error_code(self.code)
            .error_message(self.message)
            .send()
    }
}
