/// Typed multipart extractor module,
/// available with `typed-multipart` feature.
#[cfg(feature = "typed_multipart")]
pub mod typed;

#[cfg(feature = "typed_multipart")]
pub use crate::extract::multipart::typed::TypedMultipart;
