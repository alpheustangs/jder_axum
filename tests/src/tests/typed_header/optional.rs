#[cfg(test)]
mod test {
    use axum::http::header;
    use axum_test::TestServer;
    use jder_axum::response::json::JsonResponse;

    use crate::router::create_server;

    type RouteResponse = JsonResponse<String>;

    #[tokio::test]
    async fn test() {
        let server: TestServer = create_server();

        let res: RouteResponse = server
            .post("/typed_header/optional")
            .add_header(header::USER_AGENT, "jder_axum")
            .await
            .json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: String = res.data.unwrap();

        assert_eq!(data, "jder_axum");
    }

    #[tokio::test]
    async fn test_empty() {
        let server: TestServer = create_server();

        let res: RouteResponse =
            server.post("/typed_header/optional").await.json::<RouteResponse>();

        assert_eq!(res.success, true);
    }
}
