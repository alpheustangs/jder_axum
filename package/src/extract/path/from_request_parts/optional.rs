use axum::{
    extract::{path::Path as _Path, OptionalFromRequestParts},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use crate::{
    extract::path::{from_request_parts::match_path_rejection, Path},
    response::Response,
};

impl<T, S> OptionalFromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Send + 'static,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        match _Path::<T>::from_request_parts(parts, state).await {
            | Ok(Some(value)) => Ok(Some(Self(value.0))),
            | Ok(None) => Ok(None),
            | Err(rejection) => Err(match_path_rejection(rejection)),
        }
    }
}
