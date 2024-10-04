use std::sync::{Arc, Mutex, MutexGuard};

use axum::http::StatusCode;
use jder_axum::{
    extract::State,
    response::{CreateJsonResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RouteStateResponseData {
    pub view: usize,
}

#[derive(Clone)]
pub struct AppState {
    pub view: Arc<Mutex<usize>>,
}

pub async fn route_state(State(state): State<AppState>) -> Response {
    let mut view: MutexGuard<'_, usize> = state.view.lock().unwrap();
    *view += 1;

    CreateJsonResponse::success::<RouteStateResponseData>()
        .status(StatusCode::CREATED)
        .data(RouteStateResponseData { view: *view })
        .send()
}
