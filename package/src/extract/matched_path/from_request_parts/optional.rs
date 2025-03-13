use axum::{
    extract::{MatchedPath as _MatchedPath, OptionalFromRequestParts},
    http::{request::Parts, StatusCode},
};

use crate::{
    extract::matched_path::MatchedPath,
    response::{
        json::{CreateJsonResponse, JsonResponseErrorCode},
        Response,
    },
};

impl<S> OptionalFromRequestParts<S> for MatchedPath
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        match _MatchedPath::from_request_parts(parts, state).await {
            | Ok(Some(value)) => Ok(Some(Self(value.as_str().into()))),
            | Ok(None) => Ok(None),
            | Err(_) => Err(CreateJsonResponse::failure()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .error_code(JsonResponseErrorCode::Server.as_str())
                .send()),
        }
    }
}
