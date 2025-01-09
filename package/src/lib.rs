//! # JDER axum
//!
//! A response builder for axum.
//!
//! This package includes several axum response builders and different
//! extractors based on the JSON response structure specified in
//! [JSON Data Error Response (JDER)](https://github.com/alpheustangs/jder).
//! With the builders and extractors provided, various kinds of
//! responses can be created easily instead of sending plain text responses.
//!
//! ## Usage
//!
//! To create a JSON response, use
//! [`CreateJsonResponse`](response::json::CreateJsonResponse):
//!
//! ```no_run
//! use jder_axum::response::{
//!     Response,
//!     json::CreateJsonResponse
//! };
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct RouteResponseData {
//!    title: String,
//! }
//!
//! async fn route() -> Response {
//!     CreateJsonResponse::success::<RouteResponseData>()
//!         .data(RouteResponseData {
//!             title: "Title".to_string(),
//!         })
//!         .send()
//! }
//! ```
//!
//! If no data is needed, use
//! [`dataless`](response::json::CreateJsonResponse::dataless)
//! function instead:
//!
//! ```no_run
//! use jder_axum::response::{
//!     Response,
//!     json::CreateJsonResponse
//! };
//!
//! async fn route() -> Response {
//!     CreateJsonResponse::dataless().send()
//! }
//! ```
//!
//! For returning content other than JSON, use
//! [`CreateResponse`](response::CreateResponse):
//!
//! ```no_run
//! use axum::http::header;
//! use jder_axum::response::{
//!     Response,
//!     CreateResponse
//! };
//! use serde::Serialize;
//!
//! async fn route() -> Response {
//!     CreateResponse::success()
//!         .header(header::CONTENT_TYPE, "text/plain")
//!         .body("hi")
//! }
//! ```

mod internal;

/// Extract module contains different extractors.
pub mod extract {
    pub use crate::internal::extract::connect_info::ConnectInfo;

    pub use crate::internal::extract::host::Host;

    pub use crate::internal::extract::json::Json;

    pub use crate::internal::extract::matched_path::MatchedPath;

    pub use crate::internal::extract::multipart::Multipart;

    /// Multipart extractor module.
    pub mod multipart {
        pub use crate::internal::extract::multipart::{
            Multipart, MultipartFailureResponse,
        };
    }

    pub use crate::internal::extract::nested_path::NestedPath;

    pub use crate::internal::extract::original_uri::OriginalUri;

    pub use crate::internal::extract::path::Path;

    pub use crate::internal::extract::query::Query;

    /// Query extractor module.
    pub mod query {
        pub use crate::internal::extract::query::{empty_to_none, Query};
    }

    pub use crate::internal::extract::state::State;
}

/// Response module contains different response functions.
pub mod response {
    // base
    pub use crate::internal::response::{
        CreateResponse, Response, ResponseFunctions,
    };

    /// JSON module.
    pub mod json {
        // base
        pub use crate::internal::response::json::{
            CreateJsonResponse, JsonResponse, JsonResponseError,
        };

        // success
        pub use crate::internal::response::json::success::JsonSuccessResponseFunctions;

        // failure
        pub use crate::internal::response::json::failure::JsonFailureResponseFunctions;

        // error
        pub use crate::internal::response::json::error::JsonResponseErrorCode;
    }

    /// Header module.
    pub mod header {
        pub use crate::internal::response::header::{
            get_header_from_key_value, get_header_name_from_key,
            get_header_value_from_value,
        };
    }
}
