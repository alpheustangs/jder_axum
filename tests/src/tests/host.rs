#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use jder_axum::response::json::JsonResponse;

    use crate::router::create_server;
    use crate::router::host::RouteHostResponseData;

    #[tokio::test]
    async fn test() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse<RouteHostResponseData>;

        let res: RouteResponse =
            server.post("/host").await.json::<RouteResponse>();

        assert_eq!(res.success, true);
    }
}
