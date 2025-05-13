#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use jder_axum::response::json::{JsonResponse, JsonResponseErrorCode};

    use crate::router::create_server;
    use crate::router::query::RouteQueryResponseData;

    type RouteResponse = JsonResponse<RouteQueryResponseData>;

    #[tokio::test]
    async fn test() {
        let server: TestServer = create_server();

        let res: RouteResponse = server
            .post("/query?num=1&empty=Empty&none=None")
            .await
            .json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteQueryResponseData = res.data.unwrap();

        assert_eq!(data.num, Some(1));
        assert_eq!(data.empty, Some("Empty".to_string()));
        assert_eq!(data.none, Some("None".to_string()));
    }

    #[tokio::test]
    async fn test_empty_num() {
        let server: TestServer = create_server();

        let res: RouteResponse = server
            .post("/query?num=&empty=Empty&none=None")
            .await
            .json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteQueryResponseData = res.data.unwrap();

        assert_eq!(data.num, None);
        assert_eq!(data.empty, Some("Empty".to_string()));
        assert_eq!(data.none, Some("None".to_string()));
    }

    #[tokio::test]
    async fn test_empty_empty() {
        let server: TestServer = create_server();

        let res: RouteResponse = server
            .post("/query?num=1&empty=&none=None")
            .await
            .json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteQueryResponseData = res.data.unwrap();

        assert_eq!(data.num, Some(1));
        assert_eq!(data.empty, Some("".to_string()));
        assert_eq!(data.none, Some("None".to_string()));
    }

    #[tokio::test]
    async fn test_empty_none() {
        let server: TestServer = create_server();

        let res: RouteResponse = server
            .post("/query?num=1&empty=Empty&none=")
            .await
            .json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteQueryResponseData = res.data.unwrap();

        assert_eq!(data.num, Some(1));
        assert_eq!(data.empty, Some("Empty".to_string()));
        assert_eq!(data.none, None);
    }

    #[tokio::test]
    async fn test_empty_query() {
        let server: TestServer = create_server();

        let res: RouteResponse =
            server.post("/query").await.json::<RouteResponse>();

        assert_eq!(res.success, true);
    }

    #[tokio::test]
    async fn test_error() {
        let server: TestServer = create_server();

        let res: JsonResponse =
            server.post("/query?num=1A").await.json::<JsonResponse>();

        assert_eq!(res.success, false);
        assert_eq!(
            res.error.unwrap().code,
            JsonResponseErrorCode::Parse.to_string()
        );
    }
}
