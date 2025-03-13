use axum::{
    extract::{rejection::FormRejection, Form as _Form, FromRequest},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;

use crate::response::{
    json::{CreateJsonResponse, JsonResponseErrorCode},
    Response,
};

/// URL encoded extractor and response.
///
/// Check [`Form`](axum::extract::Form) for more information.
///
/// ## Examples
///
/// An example of using `Form` as an extractor:
///
/// ```no_run
/// use jder_axum::extract::Form;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct CreateUser {
///     email: String,
///     password: String,
/// }
///
/// async fn route(
///     Form(payload): Form<CreateUser>
/// ) {
///     // ...
/// }
/// ```
///
/// An example of using `Form` as a response:
///
/// ```no_run
/// use jder_axum::extract::Form;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct User {
///     id: usize,
///     username: String,
/// }
///
/// async fn route() -> Form<User> {
///     Form(User {
///         id: 1,
///         username: "Name".to_string(),
///     })
/// }
/// ```
#[derive(Debug, Clone, Copy, Default, FromRequest)]
#[from_request(via(_Form), rejection(FormError))]
pub struct Form<T>(pub T);

impl<T: Serialize> IntoResponse for Form<T> {
    fn into_response(self) -> Response {
        let Self(value) = self;
        _Form(value).into_response()
    }
}

#[derive(Debug, Clone, Default)]
pub struct FormError {
    pub status: StatusCode,
    pub code: String,
    pub message: String,
}

impl From<FormRejection> for FormError {
    fn from(rejection: FormRejection) -> Self {
        Self {
            status: rejection.status(),
            code: JsonResponseErrorCode::Parse.to_string(),
            message: rejection.body_text(),
        }
    }
}

impl IntoResponse for FormError {
    fn into_response(self) -> Response {
        CreateJsonResponse::failure()
            .status(self.status)
            .error_code(self.code)
            .error_message(self.message)
            .send()
    }
}
