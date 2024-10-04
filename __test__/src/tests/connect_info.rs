#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use jder_axum::response::JsonResponse;

    use crate::router::connect_info::RouteConnectInfoResponseData;
    use crate::router::create_server;

    #[tokio::test]
    async fn test() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse<RouteConnectInfoResponseData>;

        let res: RouteResponse =
            server.post("/connect_info").await.json::<RouteResponse>();

        assert_eq!(res.success, true);
    }
}
