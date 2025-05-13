#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use jder_axum::response::json::{JsonResponse, JsonResponseErrorCode};

    use crate::router::create_server;
    use crate::router::nested_path::RouteNestedPathResponseData;

    #[tokio::test]
    async fn test_error() {
        let server: TestServer = create_server();

        let res: JsonResponse =
            server.post("/nested_path").await.json::<JsonResponse>();

        assert_eq!(res.success, false);
        assert_eq!(
            res.error.unwrap().code,
            JsonResponseErrorCode::Parse.to_string()
        );
    }

    #[tokio::test]
    async fn test() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse<RouteNestedPathResponseData>;

        let res: RouteResponse =
            server.post("/123/nested_path").await.json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteNestedPathResponseData = res.data.unwrap();

        assert_eq!(data.path, "/{id}".to_string());
    }
}
