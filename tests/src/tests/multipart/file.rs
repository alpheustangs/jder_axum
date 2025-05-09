#[cfg(test)]
mod test {
    use axum::{body::Bytes, http::StatusCode};
    use axum_test::{
        TestResponse, TestServer,
        multipart::{MultipartForm, Part},
    };
    use jder_axum::response::json::JsonResponse;

    use crate::router::create_server;

    #[tokio::test]
    async fn test() {
        let server: TestServer = create_server();

        let form: MultipartForm =
            MultipartForm::new().add_part("image", Part::bytes(Bytes::new()));

        let res: TestResponse =
            server.post("/multipart/file").multipart(form).await;

        assert_eq!(res.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_error_nobody() {
        let server: TestServer = create_server();

        let res: JsonResponse =
            server.post("/multipart/file").await.json::<JsonResponse>();

        assert_eq!(res.success, false);
    }

    #[tokio::test]
    async fn test_error_empty_body() {
        let server: TestServer = create_server();

        let res: JsonResponse = server
            .post("/multipart/file")
            .multipart(MultipartForm::new())
            .await
            .json::<JsonResponse>();

        assert_eq!(res.success, false);
    }
}
