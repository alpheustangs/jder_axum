use axum::extract::Form as _Form;
use axum_core::{
    extract::{FromRequest, Request},
    response::IntoResponse,
};
use http::{StatusCode, header};
use serde::{Serialize, de::DeserializeOwned};

use crate::response::{
    CreateResponse, Response,
    json::{CreateJsonResponse, JsonResponseErrorCode},
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
#[derive(Debug, Clone, Copy, Default)]
pub struct Form<T>(pub T);

impl<T, S> FromRequest<S> for Form<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(
        req: Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match _Form::<T>::from_request(req, state).await {
            | Ok(val) => Ok(Self(val.0)),
            | Err(rej) => Err(CreateJsonResponse::failure()
                .status(rej.status())
                .error_code(JsonResponseErrorCode::Parse.as_str())
                .error_message(rej.body_text())
                .send()),
        }
    }
}

impl<T> IntoResponse for Form<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match serde_urlencoded::to_string(&self.0) {
            | Ok(body) => CreateResponse::success()
                .header(
                    header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded",
                )
                .body(body),
            | Err(err) => CreateJsonResponse::failure()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .error_code(JsonResponseErrorCode::Server.as_str())
                .error_message(err.to_string())
                .send(),
        }
    }
}

axum_core::__impl_deref!(Form);
