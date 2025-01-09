use std::ops::{Deref, DerefMut};

use axum::{
    extract::{FromRef, FromRequestParts, State as _State},
    http::{request::Parts, StatusCode},
};

use crate::internal::response::{
    json::{error::JsonResponseErrorCode, CreateJsonResponse},
    Response,
};

/// Extractor for state.
///
/// See [`State`](axum::extract::State) for more information.
///
/// ## Example
///
/// Basic example:
///
/// ```no_run
/// use axum::{
///     Router,
///     routing::get
/// };
/// use jder_axum::extract::State;
///
/// #[derive(Clone)]
/// struct AppState {
///     title: String,
/// }
///
/// let app_state: AppState = AppState {
///     title: "Hello, World!".to_string(),
/// };
///
/// async fn route(State(state): State<AppState>) {
///     let title: String = state.title;
///     // ...
/// }
///
/// let app: Router = Router::new()
///     .route("/", get(route))
///     .with_state(app_state);
/// ```
///
/// Mutable state example:
///
/// ```no_run
/// use std::sync::{Arc, Mutex, MutexGuard};
/// use axum::{
///     Router,
///     routing::get
/// };
/// use jder_axum::extract::State;
///
/// #[derive(Clone)]
/// struct AppState {
///     view: Arc<Mutex<usize>>,
/// }
///
/// let app_state: AppState = AppState {
///     view: Arc::new(Mutex::new(0)),
/// };
///
/// async fn route(State(state): State<AppState>) {
///     let mut view: MutexGuard<'_, usize> =
///         state.view.lock().unwrap();
///     *view += 1;
///     // ...
/// }
///
/// let app: Router = Router::new()
///     .route("/", get(route))
///     .with_state(app_state);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct State<S>(pub S);

impl<O, I> FromRequestParts<O> for State<I>
where
    I: FromRef<O>,
    O: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &O,
    ) -> Result<Self, Self::Rejection> {
        match _State::<I>::from_request_parts(parts, state).await {
            | Ok(value) => Ok(Self(value.0)),
            | Err(_) => Err(CreateJsonResponse::failure()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .error_code(JsonResponseErrorCode::Server.as_str())
                .send()),
        }
    }
}

impl<S> Deref for State<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for State<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
