pub mod from_request_parts;

/// Extractor that parses path parameters.
///
/// Check [`Path`](axum::extract::Path) for more information.
///
/// ## Example
///
/// ```no_run
/// use jder_axum::extract::Path;
///
/// // /users/:id/:name
/// async fn route(
///     Path((id, name)): Path<(String, String)>,
/// ) {
///     // ...
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Path<T>(pub T);

axum_core::__impl_deref!(Path);
