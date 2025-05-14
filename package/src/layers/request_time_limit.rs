use std::{task::Context, time::Duration};

use http::{Request, StatusCode};
use tower_layer::Layer;
use tower_service::Service;

use crate::response::{
    Response as Res,
    json::{CreateJsonResponse, JsonResponseErrorCode},
};

#[derive(Debug, Clone, Copy)]
pub struct RequestTimeLimitService<S> {
    inner: S,
    limit: Duration,
}

impl<B, S> Service<Request<B>> for RequestTimeLimitService<S>
where
    S: Service<Request<B>, Response = Res>,
    S::Future: Send + 'static,
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
        let limit: Duration = self.limit;
        let fut: S::Future = self.inner.call(req);

        Box::pin(async move {
            match tokio::time::timeout(limit, fut).await {
                | Ok(result) => result,
                | Err(_) => {
                    let res: Res = CreateJsonResponse::failure()
                        .status(StatusCode::REQUEST_TIMEOUT)
                        .error_code(JsonResponseErrorCode::Timeout.as_str())
                        .send();

                    Ok(res)
                },
            }
        })
    }
}

/// Layer for configuring the request time limit.
///
/// ## Example
///
/// ```no_run
/// use std::time::Duration;
///
/// use axum::Router;
/// use jder_axum::layers::RequestTimeLimit;
///
/// let app: Router = Router::new()
///     .layer(RequestTimeLimit::max(Duration::from_secs(10)));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RequestTimeLimit {
    limit: Duration,
}

impl RequestTimeLimit {
    /// Set the request time limit with [`Duration`].
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use std::time::Duration;
    ///
    /// use jder_axum::layers::RequestTimeLimit;
    ///
    /// RequestTimeLimit::max(Duration::from_secs(10)); // 10s
    /// ```
    pub fn max(limit: Duration) -> Self {
        Self { limit }
    }
}

impl<S> Layer<S> for RequestTimeLimit {
    type Service = RequestTimeLimitService<S>;

    fn layer(
        &self,
        inner: S,
    ) -> Self::Service {
        RequestTimeLimitService { inner, limit: self.limit }
    }
}
