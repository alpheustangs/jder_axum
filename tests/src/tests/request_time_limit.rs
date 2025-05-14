#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use jder_axum::response::json::{
        JsonResponse, JsonResponseError, JsonResponseErrorCode,
    };

    use crate::router::create_server;

    #[tokio::test]
    async fn test_1s() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse;

        let res: RouteResponse =
            server.post("/request_time_limit/ok").await.json::<RouteResponse>();

        assert_eq!(res.success, true);
    }

    #[tokio::test]
    async fn test_2s() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse;

        let res: RouteResponse = server
            .post("/request_time_limit/timeout")
            .await
            .json::<RouteResponse>();

        assert_eq!(res.success, false);

        let err: JsonResponseError = res.error.unwrap();

        assert_eq!(err.code, JsonResponseErrorCode::Timeout.as_str());
    }
}
