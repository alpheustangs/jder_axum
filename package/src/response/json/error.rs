/// JSON response error code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonResponseErrorCode {
    /// Error while parsing.
    Parse,
    /// Payload too large.
    TooLarge,
    /// Timeout error.
    Timeout,
    /// Internal server error.
    Server,
    /// Unknown error.
    Unknown,
}

impl JsonResponseErrorCode {
    /// Get the error code as `&str`.
    pub fn as_str(&self) -> &str {
        match self {
            | Self::Parse => "parse",
            | Self::TooLarge => "too_large",
            | Self::Timeout => "timeout",
            | Self::Server => "server",
            | Self::Unknown => "unknown",
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

pub(crate) const FAILURE_RESPONSE_DEFAULT: &str = "{\"success\":false,\"data\":null,\"error\":{\"code\":\"server\",\"field\":null,\"message\":\"Internal server error.\"}}";
