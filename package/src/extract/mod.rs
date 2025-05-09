pub mod nested_path;
pub mod path;
pub mod state;

/// Form extractor module,
/// available with `form` feature.
#[cfg(feature = "form")]
pub mod form;

/// Json extractor module,
/// available with `json` feature.
#[cfg(feature = "json")]
pub mod json;

/// Matched path extractor module,
/// available with `matched_path` feature.
#[cfg(feature = "matched_path")]
pub mod matched_path;

/// Multipart extractor module,
/// available with `multipart` feature.
#[cfg(feature = "multipart")]
pub mod multipart;

/// Original URI extractor module,
/// available with `original_uri` feature.
#[cfg(feature = "original_uri")]
pub mod original_uri;

/// Query extractor module,
/// available with `query` feature.
#[cfg(feature = "query")]
pub mod query;

/// Connect info extractor module,
/// available with `tokio` feature.
#[cfg(feature = "tokio")]
pub mod connect_info;

/// axum extra extractor module,
/// available with `extra` feature.
#[cfg(feature = "extra")]
pub mod extra;

pub use crate::extract::nested_path::NestedPath;
pub use crate::extract::path::Path;
pub use crate::extract::state::State;

#[cfg(feature = "form")]
pub use crate::extract::form::Form;

#[cfg(feature = "json")]
pub use crate::extract::json::Json;

#[cfg(feature = "matched_path")]
pub use crate::extract::matched_path::MatchedPath;

#[cfg(feature = "original_uri")]
pub use crate::extract::original_uri::OriginalUri;

#[cfg(feature = "query")]
pub use crate::extract::query::Query;

#[cfg(feature = "tokio")]
pub use crate::extract::connect_info::ConnectInfo;
