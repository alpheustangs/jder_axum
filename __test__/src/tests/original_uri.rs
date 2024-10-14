#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use jder_axum::response::json::JsonResponse;

    use crate::router::create_server;
    use crate::router::original_uri::RouteOriginalUriResponseData;

    #[tokio::test]
    async fn test() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse<RouteOriginalUriResponseData>;

        let res: RouteResponse =
            server.post("/original_uri").await.json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteOriginalUriResponseData = res.data.unwrap();

        assert_eq!(data.uri, "/original_uri".to_string());
        assert_eq!(data.original_uri, "/original_uri".to_string());
    }

    #[tokio::test]
    async fn test_nested() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse<RouteOriginalUriResponseData>;

        let res: RouteResponse =
            server.post("/123/original_uri").await.json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteOriginalUriResponseData = res.data.unwrap();

        assert_eq!(data.uri, "/original_uri".to_string());
        assert_eq!(data.original_uri, "/123/original_uri".to_string());
    }
}
