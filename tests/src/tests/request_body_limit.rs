#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use jder_axum::response::json::{
        JsonResponse, JsonResponseError, JsonResponseErrorCode,
    };

    use crate::router::create_server;

    #[tokio::test]
    async fn test_1mb() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse;

        let res: RouteResponse = server
            .post("/request_body_limit/1mb")
            .await
            .json::<RouteResponse>();

        assert_eq!(res.success, true);
    }

    #[tokio::test]
    async fn test_1mb_too_large() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse;

        let res: RouteResponse = server
            .post("/request_body_limit/1mb")
            .text("a".repeat(1 * 1024 * 1024 + 1))
            .await
            .json::<RouteResponse>();

        assert_eq!(res.success, false);

        let err: JsonResponseError = res.error.unwrap();

        assert_eq!(err.code, JsonResponseErrorCode::TooLarge.as_str());
    }

    #[tokio::test]
    async fn test_10mb() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse;

        let res: RouteResponse = server
            .post("/request_body_limit/10mb")
            .text("a".repeat(1 * 1024 * 1024 + 1))
            .await
            .json::<RouteResponse>();

        assert_eq!(res.success, true);
    }

    #[tokio::test]
    async fn test_10mb_too_large() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse;

        let res: RouteResponse = server
            .post("/request_body_limit/10mb")
            .text("a".repeat(10 * 1024 * 1024 + 1))
            .await
            .json::<RouteResponse>();

        assert_eq!(res.success, false);

        let err: JsonResponseError = res.error.unwrap();

        assert_eq!(err.code, JsonResponseErrorCode::TooLarge.as_str());
    }
}
