use axum::response::{IntoResponse, Response as AxumResponse};
use axum_typed_multipart::{BaseMultipart, TypedMultipartError};

use crate::internal::{
    extract::json::Json,
    response::json::{
        JsonResponse, JsonResponseError, error::JsonResponseErrorCode,
    },
};

/// Multipart failure response.
pub type MultipartFailureResponse = JsonResponse<()>;

impl IntoResponse for MultipartFailureResponse {
    fn into_response(self) -> AxumResponse {
        Json(self).into_response()
    }
}

impl From<TypedMultipartError> for MultipartFailureResponse {
    fn from(error: TypedMultipartError) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(JsonResponseError {
                code: JsonResponseErrorCode::Parse.to_string(),
                field: None,
                message: Some(error.to_string()),
            }),
        }
    }
}

/// Extractor that parses `multipart/form-data` requests.
///
/// Check [`axum_typed_multipart`] for more information.
///
/// ## Example
///
/// ```no_run
/// use axum_typed_multipart::TryFromMultipart;
/// use jder_axum::extract::Multipart;
///
/// #[derive(TryFromMultipart)]
/// struct Data {
///     name: String,
/// }
///
/// async fn route(data: Multipart<Data>) {
///     // ...
/// }
/// ```
///
/// ## Large uploads
///
/// By default, [`axum`] will limit the size of body to 2MB,
/// and [`axum_typed_multipart`] will limit
/// the size of each data field to 1MiB.
/// To increase the limit, you may follow the instructions below.
///
/// #### Increase Data Field Limit
///
/// Increase the limit of each data field with
/// `KiB`, `MiB` or `GiB` suffix:
///
/// ```no_run
/// use axum::body::Bytes;
/// use axum_typed_multipart::{
///     TryFromMultipart,
///     FieldData,
/// };
/// use jder_axum::extract::Multipart;
///
/// #[derive(TryFromMultipart)]
/// struct Data {
///     #[form_data(limit = "10MiB")]
///     image: FieldData<Bytes>,
/// }
///
/// async fn route(data: Multipart<Data>) {
///     // ...
/// }
/// ```
///
/// #### Increase Body Limit
///
/// Increase the limit of the body in router:
///
/// ```no_run
/// use axum::{
///     Router,
///     extract::DefaultBodyLimit,
/// };
///
/// let router: Router = Router::new()
///     // 20MiB
///     .layer(DefaultBodyLimit::max(20 * 1024 * 1024));
/// ```
///
/// ## Enums
///
/// You can use enums in multipart fields with
/// the help of [`TryFromField`](axum_typed_multipart::TryFromField).
///
/// ```no_run
/// use axum_typed_multipart::TryFromField;
///
/// #[derive(TryFromField)]
/// enum AccountType {
///     #[field(rename = "admin")]
///     Admin,
///     #[field(rename = "user")]
///     User,
/// }
/// ```
pub type Multipart<T> = BaseMultipart<T, MultipartFailureResponse>;
