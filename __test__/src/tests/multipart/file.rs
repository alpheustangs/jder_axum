#[cfg(test)]
mod test {
    use axum::{body::Bytes, http::StatusCode};
    use axum_test::{
        multipart::{MultipartForm, Part},
        TestResponse, TestServer,
    };

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

        let res: TestResponse = server.post("/multipart/file").await;

        assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_error_empty_body() {
        let server: TestServer = create_server();

        let res: TestResponse = server
            .post("/multipart/file")
            .multipart(MultipartForm::new())
            .await;

        assert_eq!(res.status_code(), StatusCode::NOT_FOUND);
    }
}
