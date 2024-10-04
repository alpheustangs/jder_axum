/// Error message types.
pub enum ResponseErrorCode {
    ParseError,
    ServerError,
    UnknownError,
}

impl std::fmt::Display for ResponseErrorCode {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::ParseError => f.write_str("parse_error"),
            | Self::ServerError => f.write_str("server_error"),
            | Self::UnknownError => f.write_str("unknown_error"),
        }
    }
}

pub const SERVER_ERROR_RESPONSE: &str = "{\"success\":false,\"data\":null,\"error\":{\"code\":\"server_error\",\"field\":null,\"message\":\"Internal server error.\"}}";
