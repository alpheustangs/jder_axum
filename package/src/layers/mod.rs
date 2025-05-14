/// Request body limit layer,
/// available with `request_body_limit` feature.
#[cfg(feature = "request_body_limit")]
pub mod request_body_limit;

/// Request time limit layer,
/// available with `request_time_limit` feature.
#[cfg(feature = "request_time_limit")]
pub mod request_time_limit;

#[cfg(feature = "request_body_limit")]
pub use crate::layers::request_body_limit::RequestBodyLimit;

#[cfg(feature = "request_time_limit")]
pub use crate::layers::request_time_limit::RequestTimeLimit;
