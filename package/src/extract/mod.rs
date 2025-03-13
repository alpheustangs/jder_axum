pub mod connect_info;
pub mod form;
pub mod json;
pub mod matched_path;
pub mod nested_path;
pub mod original_uri;
pub mod path;
pub mod query;
pub mod state;

/// Multipart extractor module,
/// available with `multipart` feature.
#[cfg(feature = "multipart")]
pub mod multipart;

/// axum extra extractor module,
/// available with `extra` feature.
#[cfg(feature = "extra")]
pub mod extra;

pub use crate::extract::connect_info::ConnectInfo;
pub use crate::extract::form::Form;
pub use crate::extract::json::Json;
pub use crate::extract::matched_path::MatchedPath;
pub use crate::extract::nested_path::NestedPath;
pub use crate::extract::original_uri::OriginalUri;
pub use crate::extract::path::Path;
pub use crate::extract::query::Query;
pub use crate::extract::state::State;

#[cfg(feature = "multipart")]
pub use crate::extract::multipart::Multipart;
