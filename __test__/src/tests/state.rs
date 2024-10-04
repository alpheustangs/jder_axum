#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use jder_axum::response::JsonResponse;

    use crate::router::create_server;
    use crate::router::state::RouteStateResponseData;

    #[tokio::test]
    async fn test() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse<RouteStateResponseData>;

        // 1

        let res: RouteResponse =
            server.post("/state").await.json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteStateResponseData = res.data.unwrap();

        assert_eq!(data.view, 1);

        // 2

        let res: RouteResponse =
            server.post("/state").await.json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteStateResponseData = res.data.unwrap();

        assert_eq!(data.view, 2);
    }
}
