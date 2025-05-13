#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use jder_axum::response::json::{JsonResponse, JsonResponseErrorCode};

    use crate::router::create_server;
    use crate::router::path::RoutePathResponseData;

    #[tokio::test]
    async fn test() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse<RoutePathResponseData>;

        let res: RouteResponse =
            server.post("/path/123/Name").await.json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RoutePathResponseData = res.data.unwrap();

        assert_eq!(data.id, 123);
        assert_eq!(data.name, "Name");
    }

    #[tokio::test]
    async fn test_error() {
        let server: TestServer = create_server();

        let res: JsonResponse =
            server.post("/path/123A/Name").await.json::<JsonResponse>();

        assert_eq!(res.success, false);
        assert_eq!(
            res.error.unwrap().code,
            JsonResponseErrorCode::Parse.to_string()
        );
    }
}
