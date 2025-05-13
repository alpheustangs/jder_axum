pub mod optional;

#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use jder_axum::response::json::{JsonResponse, JsonResponseErrorCode};

    use crate::router::create_server;
    use crate::router::json::RouteJsonResponseData;

    type RouteResponse = JsonResponse<RouteJsonResponseData>;

    #[tokio::test]
    async fn test() {
        let server: TestServer = create_server();

        let body: RouteJsonResponseData = RouteJsonResponseData {
            id: Some(123),
            name: Some("Name".to_string()),
        };

        let res: RouteResponse =
            server.post("/json").json(&body).await.json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteJsonResponseData = res.data.unwrap();

        assert_eq!(data.id, Some(123));
        assert_eq!(data.name, Some("Name".to_string()));
    }

    #[tokio::test]
    async fn test_empty_id() {
        let server: TestServer = create_server();

        let body: RouteJsonResponseData =
            RouteJsonResponseData { id: None, name: Some("Name".to_string()) };

        let res: RouteResponse =
            server.post("/json").json(&body).await.json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteJsonResponseData = res.data.unwrap();

        assert_eq!(data.id, None);
        assert_eq!(data.name, Some("Name".to_string()));
    }

    #[tokio::test]
    async fn test_empty_name() {
        let server: TestServer = create_server();

        let body: RouteJsonResponseData =
            RouteJsonResponseData { id: Some(123), name: None };

        let res: RouteResponse =
            server.post("/json").json(&body).await.json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteJsonResponseData = res.data.unwrap();

        assert_eq!(data.id, Some(123));
        assert_eq!(data.name, None);
    }

    #[tokio::test]
    async fn test_empty_body() {
        let server: TestServer = create_server();

        let res: RouteResponse =
            server.post("/json").await.json::<RouteResponse>();

        assert_eq!(res.success, false);
        assert_eq!(
            res.error.unwrap().code,
            JsonResponseErrorCode::Parse.to_string()
        );
    }
}
