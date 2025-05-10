#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use jder_axum::response::json::JsonResponse;

    use crate::router::create_server;
    use crate::router::scheme::RouteSchemeResponseData;

    #[tokio::test]
    async fn test_forwarded() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse<RouteSchemeResponseData>;

        let res: RouteResponse = server
            .post("/scheme")
            .add_header("Forwarded", "proto=abcd")
            .await
            .json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteSchemeResponseData = res.data.unwrap();

        assert_eq!(data.scheme, "abcd");
    }

    #[tokio::test]
    async fn test_x_forwarded_proto() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse<RouteSchemeResponseData>;

        let res: RouteResponse = server
            .post("/scheme")
            .add_header("X-Forwarded-Proto", "abcd")
            .await
            .json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteSchemeResponseData = res.data.unwrap();

        assert_eq!(data.scheme, "abcd");
    }

    #[tokio::test]
    async fn test_error() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse<RouteSchemeResponseData>;

        let res: RouteResponse =
            server.post("/scheme").await.json::<RouteResponse>();

        assert_eq!(res.success, false);
    }
}
