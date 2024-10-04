#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use jder_axum::response::{JsonResponse, ResponseErrorCode};

    use crate::router::create_server;
    use crate::router::query::RouteQueryResponseData;

    type RouteResponse = JsonResponse<RouteQueryResponseData>;

    #[tokio::test]
    async fn test() {
        let server: TestServer = create_server();

        let res: RouteResponse = server
            .post("/query?page=1&title=Title")
            .await
            .json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteQueryResponseData = res.data.unwrap();

        assert_eq!(data.page, Some(1));
        assert_eq!(data.title, Some("Title".to_string()));
    }

    #[tokio::test]
    async fn test_empty_page() {
        let server: TestServer = create_server();

        let res: RouteResponse = server
            .post("/query?page=&title=Title")
            .await
            .json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteQueryResponseData = res.data.unwrap();

        assert_eq!(data.page, None);
        assert_eq!(data.title, Some("Title".to_string()));
    }

    #[tokio::test]
    async fn test_empty_title() {
        let server: TestServer = create_server();

        let res: RouteResponse =
            server.post("/query?page=1&title=").await.json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteQueryResponseData = res.data.unwrap();

        assert_eq!(data.page, Some(1));
        assert_eq!(data.title, Some("".to_string()));
    }

    #[tokio::test]
    async fn test_error() {
        let server: TestServer = create_server();

        let res: JsonResponse = server
            .post("/query?page=1A&title=Title")
            .await
            .json::<JsonResponse>();

        assert_eq!(res.success, false);
        assert_eq!(
            res.error.unwrap().code,
            ResponseErrorCode::ParseError.to_string()
        );
    }
}
