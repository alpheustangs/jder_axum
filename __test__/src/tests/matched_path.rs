#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use jder_axum::response::JsonResponse;

    use crate::router::create_server;
    use crate::router::matched_path::RouteMatchedPathResponseData;

    #[tokio::test]
    async fn test() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse<RouteMatchedPathResponseData>;

        let res: RouteResponse =
            server.post("/matched_path").await.json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteMatchedPathResponseData = res.data.unwrap();

        assert_eq!(data.path, "/matched_path".to_string());
    }
}
