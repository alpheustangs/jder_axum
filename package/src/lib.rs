//! # JDER axum
//!
//! A response builder for axum.
//!
//! This package includes different axum response builders,
//! extractors and layers based on the JSON response structure specified in
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

pub mod extract;

pub mod layers;

pub mod response;
