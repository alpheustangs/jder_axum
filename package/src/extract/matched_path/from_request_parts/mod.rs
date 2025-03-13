pub mod optional;

use axum::{
    extract::{
        rejection::MatchedPathRejection, FromRequestParts,
        MatchedPath as _MatchedPath,
    },
    http::{request::Parts, StatusCode},
};

use crate::{
    extract::matched_path::MatchedPath,
    response::{
        json::{error::JsonResponseErrorCode, CreateJsonResponse},
        Response,
    },
};

impl<S> FromRequestParts<S> for MatchedPath
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match _MatchedPath::from_request_parts(parts, state).await {
            | Ok(value) => Ok(MatchedPath(value.as_str().into())),
            | Err(rejection) => Err(match rejection {
                | MatchedPathRejection::MatchedPathMissing(inner) => {
                    CreateJsonResponse::failure()
                        .status(inner.status())
                        .error_code(JsonResponseErrorCode::Parse.as_str())
                        .error_message(inner.body_text())
                        .send()
                },
                | _ => CreateJsonResponse::failure()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .error_code(JsonResponseErrorCode::Server.as_str())
                    .error_message(rejection.body_text())
                    .send(),
            }),
        }
    }
}
