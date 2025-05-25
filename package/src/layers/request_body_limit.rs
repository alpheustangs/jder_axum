use std::task::Context;

use http::{Request, StatusCode};
use http_body::Body;
use http_body_util::{BodyExt as _, LengthLimitError, Limited};
use tower_layer::Layer;
use tower_service::Service;

use crate::response::{
    Response as Res,
    json::{CreateJsonResponse, JsonResponseErrorCode},
};

#[derive(Debug, Clone, Copy)]
pub struct RequestBodyLimitService<S> {
    inner: S,
    limit: usize,
}

impl<B, S> Service<Request<B>> for RequestBodyLimitService<S>
where
    S: Service<Request<B>, Response = Res> + Clone + Send + 'static,
    S::Future: Send + 'static,
    B: Body + Send + From<bytes::Bytes> + 'static,
    B::Data: Send + 'static,
    B::Error: std::error::Error + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<Self::Response, Self::Error>,
                > + Send,
        >,
    >;

    fn poll_ready(
        &mut self,
        cx: &mut Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(
        &mut self,
        req: Request<B>,
    ) -> Self::Future {
        let (parts, body) = req.into_parts();

        let limited_body: Limited<B> = Limited::new(body, self.limit);

        let mut inner: S = self.inner.clone();

        Box::pin(async move {
            match limited_body.collect().await {
                | Ok(collected) => {
                    let new_body: B = collected.to_bytes().into();

                    let req: Request<B> = Request::from_parts(parts, new_body);

                    inner.call(req).await
                },
                | Err(err)
                    if err.downcast_ref::<LengthLimitError>().is_some() =>
                {
                    let res: Res = CreateJsonResponse::failure()
                        .status(StatusCode::PAYLOAD_TOO_LARGE)
                        .error_code(JsonResponseErrorCode::TooLarge.as_str())
                        .error_field("body")
                        .send();

                    Ok(res)
                },
                | Err(_) => {
                    let res: Res = CreateJsonResponse::failure()
                        .status(StatusCode::BAD_REQUEST)
                        .error_code(JsonResponseErrorCode::Parse.as_str())
                        .error_field("body")
                        .send();

                    Ok(res)
                },
            }
        })
    }
}

/// Layer for configuring the request body limit.
///
/// Following error will be returned if the request body exceeds the limit:
///
/// ```jsonc
/// // Status: 413
/// {
///     "success": false,
///     "data": null,
///     "error": {
///         "code": "too_large",
///         "field": "body"
///     }
/// }
/// ```
///
/// ## Example
///
/// ```no_run
/// use axum::{
///     Router,
///     extract::DefaultBodyLimit,
/// };
/// use jder_axum::layers::RequestBodyLimit;
///
/// let app: Router = Router::new()
///     .layer(DefaultBodyLimit::disable())
///     .layer(RequestBodyLimit::max(10 * 1024 * 1024));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RequestBodyLimit {
    limit: usize,
}

impl RequestBodyLimit {
    /// Set the request body limit in bytes.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use jder_axum::layers::RequestBodyLimit;
    ///
    /// RequestBodyLimit::max(10 * 1024 * 1024); // 10MiB
    /// ```
    pub fn max(limit: usize) -> Self {
        Self { limit }
    }
}

impl<S> Layer<S> for RequestBodyLimit {
    type Service = RequestBodyLimitService<S>;

    fn layer(
        &self,
        inner: S,
    ) -> Self::Service {
        RequestBodyLimitService { inner, limit: self.limit }
    }
}
