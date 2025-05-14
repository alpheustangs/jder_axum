pub mod connect_info;
pub mod form;
pub mod header;
pub mod host;
pub mod json;
pub mod matched_path;
pub mod multipart;
pub mod nested_path;
pub mod path;
pub mod query;
pub mod request_body_limit;
pub mod request_time_limit;
pub mod scheme;
pub mod typed_header;

#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use jder_axum::response::json::JsonResponse;

    use crate::router::create_server;

    #[tokio::test]
    async fn test() {
        let server: TestServer = create_server();

        let res: JsonResponse = server.get("/").await.json::<JsonResponse>();

        assert_eq!(res.success, true);
    }
}
