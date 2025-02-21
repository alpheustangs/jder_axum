pub mod file;

#[cfg(test)]
mod test {
    use axum_test::{TestServer, multipart::MultipartForm};
    use jder_axum::{
        extract::multipart::MultipartFailureResponse,
        response::json::{JsonResponse, JsonResponseErrorCode},
    };

    use crate::router::create_server;
    use crate::router::multipart::RouteMultipartResponseData;

    #[tokio::test]
    async fn test() {
        let server: TestServer = create_server();

        type RouteResponse = JsonResponse<RouteMultipartResponseData>;

        let form: MultipartForm = MultipartForm::new()
            .add_text("string", "String")
            .add_text("number", "1");

        let res: RouteResponse = server
            .post("/multipart")
            .multipart(form)
            .await
            .json::<RouteResponse>();

        assert_eq!(res.success, true);

        let data: RouteMultipartResponseData = res.data.unwrap();

        assert_eq!(data.string, Some("String".to_string()));
        assert_eq!(data.number, Some(1));
    }

    #[tokio::test]
    async fn test_error_nobody() {
        let server: TestServer = create_server();

        let res: MultipartFailureResponse =
            server.post("/multipart").await.json::<MultipartFailureResponse>();

        assert_eq!(res.success, false);
        assert_eq!(
            res.error.unwrap().code,
            JsonResponseErrorCode::Parse.to_string()
        );
    }

    #[tokio::test]
    async fn test_error_empty_body() {
        let server: TestServer = create_server();

        let res: MultipartFailureResponse = server
            .post("/multipart")
            .multipart(MultipartForm::new())
            .await
            .json::<MultipartFailureResponse>();

        assert_eq!(res.success, false);
        assert_eq!(
            res.error.unwrap().code,
            JsonResponseErrorCode::Parse.to_string()
        );
    }

    #[tokio::test]
    async fn test_error_parse() {
        let server: TestServer = create_server();

        let form: MultipartForm = MultipartForm::new()
            .add_text("string", "String")
            .add_text("number", "1A");

        let res: MultipartFailureResponse = server
            .post("/multipart")
            .multipart(form)
            .await
            .json::<MultipartFailureResponse>();

        assert_eq!(res.success, false);
        assert_eq!(
            res.error.unwrap().code,
            JsonResponseErrorCode::Parse.to_string()
        );
    }
}
