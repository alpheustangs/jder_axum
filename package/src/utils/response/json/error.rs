/// JSON response error code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonResponseErrorCode {
    /// Error while parsing.
    Parse,
    /// Internal server error.
    Server,
    /// Unknown error.
    Unknown,
}

impl JsonResponseErrorCode {
    /// Get the error code as `&str`.
    pub fn as_str(&self) -> &str {
        match self {
            | Self::Parse => "parse_error",
            | Self::Server => "server_error",
            | Self::Unknown => "unknown_error",
        }
    }
}

impl std::fmt::Display for JsonResponseErrorCode {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub(crate) const FAILURE_RESPONSE_DEFAULT: &str = "{\"success\":false,\"data\":null,\"error\":{\"code\":\"server_error\",\"field\":null,\"message\":\"Internal server error.\"}}";
