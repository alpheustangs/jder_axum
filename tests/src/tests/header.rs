#[cfg(test)]
mod test {
    use std::io::Read;

    use axum::http::{HeaderName, HeaderValue};
    use jder_axum::response::header::{
        get_header_from_key_value, get_header_name_from_key,
        get_header_value_from_value,
    };

    #[tokio::test]
    async fn test_get_header_name_from_key() {
        let value: HeaderName = get_header_name_from_key("key").unwrap();

        assert_eq!(value.as_str(), "key");
    }

    #[tokio::test]
    async fn test_get_header_value_from_value() {
        let value: HeaderValue = get_header_value_from_value("value").unwrap();

        let mut buffer = String::new();

        value.as_bytes().read_to_string(&mut buffer).unwrap();

        assert_eq!(buffer, "value");
    }

    #[tokio::test]
    async fn test_get_header_from_key_value() {
        let (key, value): (HeaderName, HeaderValue) =
            get_header_from_key_value("key", "value").unwrap();

        let mut buffer = String::new();

        value.as_bytes().read_to_string(&mut buffer).unwrap();

        assert_eq!(key.as_str(), "key");
        assert_eq!(buffer, "value");
    }
}
