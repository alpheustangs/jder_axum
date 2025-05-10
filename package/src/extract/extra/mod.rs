pub mod host;

/// Scheme extractor module,
/// available with `extra_scheme` feature.
#[cfg(feature = "extra_scheme")]
pub mod scheme;

/// Typed header extractor module,
/// available with `extra_typed_header` feature.
#[cfg(feature = "extra_typed_header")]
pub mod typed_header;

pub use crate::extract::extra::host::Host;

#[cfg(feature = "extra_scheme")]
pub use crate::extract::extra::scheme::Scheme;

#[cfg(feature = "extra_typed_header")]
pub use crate::extract::extra::typed_header::TypedHeader;
